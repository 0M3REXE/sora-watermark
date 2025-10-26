use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use uuid::Uuid;

// Configuration
const MAX_FILE_SIZE: usize = 500 * 1024 * 1024; // 500MB default limit
const WATERMARK_PATH: &str = "sora-watermark.mp4";

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Sora AI Watermark Service"
    }))
}

async fn process_video(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    log::info!("Received video upload request");

    // Create temporary file for the uploaded video with .mp4 extension
    let mut temp_input = tempfile::Builder::new()
        .suffix(".mp4")
        .tempfile()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create temp file: {}", e)))?;
    
    let mut file_size = 0usize;
    let mut filename = String::from("video.mp4");

    // Process multipart data
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(actix_web::error::ErrorBadRequest)?;
        
        // Get filename from content disposition
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(name) = content_disposition.get_filename() {
                filename = name.to_string();
            }
        }

        // Write chunks to temporary file
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(actix_web::error::ErrorBadRequest)?;
            file_size += data.len();
            
            if file_size > MAX_FILE_SIZE {
                return Err(actix_web::error::ErrorPayloadTooLarge(
                    "File size exceeds maximum allowed size"
                ));
            }
            
            temp_input.write_all(&data)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to write data: {}", e)))?;
        }
    }

    log::info!("Uploaded file: {} ({} bytes)", filename, file_size);

    // Flush the input file
    temp_input.flush()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to flush file: {}", e)))?;

    let input_path = temp_input.path().to_path_buf();

    // Create temporary file for output with .mp4 extension
    let output_temp = tempfile::Builder::new()
        .suffix(".mp4")
        .tempfile()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create output temp file: {}", e)))?;
    let output_path = output_temp.path().to_path_buf();
    let output_path_clone = output_path.clone();

    // Process video with FFmpeg
    log::info!("Starting video processing with watermark overlay");
    
    let result = tokio::task::spawn_blocking(move || {
        apply_watermark(&input_path, &output_path_clone)
    }).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Task join error: {}", e)))?;

    match result {
        Ok(_) => {
            log::info!("Video processing completed successfully");
            
            // Read the processed video
            let video_data = std::fs::read(output_path)
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to read output: {}", e)))?;
            
            log::info!("Sending processed video ({} bytes)", video_data.len());
            
            // Return the processed video
            Ok(HttpResponse::Ok()
                .content_type("video/mp4")
                .append_header(("Content-Disposition", format!("attachment; filename=\"watermarked_{}.mp4\"", Uuid::new_v4())))
                .body(video_data))
        }
        Err(e) => {
            log::error!("Video processing failed: {}", e);
            Err(actix_web::error::ErrorInternalServerError(format!("Video processing failed: {}", e)))
        }
    }
}

fn apply_watermark(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), String> {
    // Check if watermark file exists
    if !std::path::Path::new(WATERMARK_PATH).exists() {
        return Err(format!("Watermark file not found: {}", WATERMARK_PATH));
    }

    log::info!("FFmpeg processing started - input: {:?}, output: {:?}", input_path, output_path);
    
    // FFmpeg command to overlay watermark with green screen removal
    // Optimized to match original video size/quality
    
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", input_path.to_str().unwrap(),           // Input video
            "-stream_loop", "-1",                          // Loop watermark video
            "-i", WATERMARK_PATH,                          // Watermark video
            "-filter_complex",
            "[1:1]colorkey=0x00FF00:0.6:0.3[wm];[0:v][wm]overlay=shortest=1",
            "-c:v", "libx264",                            // Video codec
            "-preset", "medium",                          // Good balance of speed and compression
            "-crf", "23",                                 // Standard quality (matches most inputs)
            "-pix_fmt", "yuv420p",                        // Pixel format for compatibility
            "-movflags", "+faststart",                    // Optimize for web streaming
            "-c:a", "copy",                               // Copy audio (preserves original quality/size)
            "-map", "0:a?",                                // Map audio from first input if exists
            "-y",                                          // Overwrite output
            output_path.to_str().unwrap()
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute FFmpeg: {}. Make sure FFmpeg is installed and in PATH.", e))?;

    log::info!("FFmpeg process completed with status: {}", output.status);
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("FFmpeg error output: {}", stderr);
        return Err(format!("FFmpeg error: {}", stderr));
    }
    
    log::info!("Video processing successful");
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger to write to stdout instead of stderr (Railway displays stderr as errors)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init();

    // Check if watermark file exists
    if !std::path::Path::new(WATERMARK_PATH).exists() {
        eprintln!("ERROR: Watermark file '{}' not found in current directory!", WATERMARK_PATH);
        eprintln!("Please make sure 'sora-watermark.mp4' is in the same directory as the executable.");
        std::process::exit(1);
    }

    // Check if FFmpeg is installed
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(_) => log::info!("FFmpeg found"),
        Err(_) => {
            eprintln!("ERROR: FFmpeg not found in PATH!");
            eprintln!("Please install FFmpeg: https://ffmpeg.org/download.html");
            std::process::exit(1);
        }
    }

    // Get bind address from environment or use default
    // Railway sets PORT, other platforms may set BIND_ADDRESS
    // For local dev without env vars, use 127.0.0.1:8000
    let bind_addr = if let Ok(port) = std::env::var("PORT") {
        format!("0.0.0.0:{}", port)
    } else {
        std::env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:8000".to_string())
    };
    
    log::info!("Starting Sora AI Watermark Service on http://{}", bind_addr);
    log::info!("Watermark file: {}", WATERMARK_PATH);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/api/process", web::post().to(process_video))
    })
    .bind(bind_addr)?
    .workers(1) // Single worker to avoid confusing deployment platforms
    .run()
    .await
}
