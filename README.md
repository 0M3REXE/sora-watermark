# 🎬 Sora AI Watermark Service

A Rust-based web service that applies Sora AI watermark overlay to videos with automatic green screen removal.

## ✨ Features

- 🎥 **Multiple Video Format Support**: MP4, AVI, MOV, WebM, and more
- 🎨 **Green Screen Removal**: Automatic chroma key removal for watermark overlay
- ⚡ **Async Processing**: Handles multiple concurrent users efficiently
- 🔒 **No Storage**: Videos processed in-memory, not stored on server
- 🚀 **Fast**: Powered by FFmpeg and Rust's performance
- 🌐 **Web Interface**: Beautiful, responsive UI for easy video processing

## 📋 Prerequisites

1. **Rust** (latest stable version)
   - Install from: https://rustup.rs/

2. **FFmpeg** (required for video processing)
   - Windows: Download from https://ffmpeg.org/download.html
   - Add to PATH environment variable
   - Verify installation: `ffmpeg -version`

3. **Sora Watermark Video**
   - Place `sora-watermark.mp4` in the project root directory
   - Must have green screen background for removal

## 🚀 Quick Start

1. **Clone or navigate to the project**:
   ```powershell
   cd c:\Users\OMER\Desktop\Fust\webtest
   ```

2. **Ensure watermark file exists**:
   ```powershell
   # Check if sora-watermark.mp4 is in current directory
   dir sora-watermark.mp4
   ```

3. **Build the project**:
   ```powershell
   cargo build --release
   ```

4. **Run the server**:
   ```powershell
   cargo run --release
   ```

5. **Open your browser**:
   - Navigate to: http://127.0.0.1:8000
   - Upload a video and apply the watermark!

## 📡 API Endpoints

### `GET /`
Returns the web interface

### `GET /health`
Health check endpoint
```json
{
  "status": "ok",
  "service": "Sora AI Watermark Service"
}
```

### `POST /api/process`
Process video with watermark overlay

**Request**:
- Method: `POST`
- Content-Type: `multipart/form-data`
- Body: Video file (field name: `video`)

**Response**:
- Content-Type: `video/mp4`
- Body: Processed video with watermark (binary data)
- Header: `Content-Disposition: attachment; filename="watermarked_[uuid].mp4"`

**Example using curl**:
```powershell
curl -X POST -F "video=@input.mp4" http://127.0.0.1:8000/api/process --output output.mp4
```

**Example using JavaScript**:
```javascript
const formData = new FormData();
formData.append('video', fileInput.files[0]);

const response = await fetch('http://127.0.0.1:8000/api/process', {
    method: 'POST',
    body: formData
});

if (response.ok) {
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    // Use url for download
}
```

## 🎨 Video Processing Details

The service uses FFmpeg with the following process:

1. **Input**: User's video + Sora watermark video
2. **Green Screen Removal**: Chroma key filter removes green (#00FF00)
3. **Overlay**: Watermark overlaid on original video
4. **Output**: H.264 encoded MP4 with AAC audio

**FFmpeg Command**:
```bash
ffmpeg -i input.mp4 -i sora-watermark.mp4 \
  -filter_complex "[1:v]chromakey=0x00FF00:0.3:0.1,format=yuva420p[overlay];[0:v][overlay]overlay=shortest=1" \
  -c:v libx264 -preset fast -crf 23 -c:a aac -b:a 192k output.mp4
```

## ⚙️ Configuration

Edit `src/main.rs` to modify settings:

```rust
const MAX_FILE_SIZE: usize = 500 * 1024 * 1024; // 500MB
const WATERMARK_PATH: &str = "sora-watermark.mp4";
```

Bind address (line ~178):
```rust
let bind_addr = "127.0.0.1:8000";
```

Worker threads (line ~181):
```rust
.workers(4) // Adjust for your CPU
```

## 🔧 Advanced Usage

### Custom Chroma Key Settings

Modify the chromakey filter parameters in `apply_watermark()`:

```rust
"[1:v]chromakey=0x00FF00:0.3:0.1,format=yuva420p[overlay]"
//              ^color  ^sim ^blend
```

- **color**: Green screen color (0x00FF00 = bright green)
- **similarity**: 0.0-1.0, how similar colors to remove (0.3 = 30%)
- **blend**: 0.0-1.0, edge blending (0.1 = 10%)

### Encoding Quality

Adjust CRF value (lower = better quality, larger file):
```rust
"-crf", "23",  // Range: 0-51, recommended: 18-28
```

Adjust encoding speed:
```rust
"-preset", "fast",  // Options: ultrafast, fast, medium, slow, veryslow
```

## 🐛 Troubleshooting

### FFmpeg not found
```
ERROR: FFmpeg not found in PATH!
```
**Solution**: Install FFmpeg and add to PATH environment variable

### Watermark file not found
```
ERROR: Watermark file 'sora-watermark.mp4' not found!
```
**Solution**: Place `sora-watermark.mp4` in project root directory

### Port already in use
```
Error: Address already in use
```
**Solution**: Change port in `main.rs` or stop process using port 8000

### Large file upload fails
**Solution**: Increase `MAX_FILE_SIZE` constant

### Processing takes too long
**Solution**: 
- Use faster FFmpeg preset
- Reduce input video resolution
- Increase worker threads

## 📊 Performance

- **Concurrency**: Handles 4 concurrent users by default (configurable)
- **Processing Speed**: Depends on video length and resolution
- **Memory Usage**: Temporary files cleaned up automatically
- **No Disk Storage**: Videos processed in temp directory, deleted after response

## 🔐 Security Notes

- ✅ File size limits prevent DOS attacks
- ✅ Multipart upload validation
- ✅ No persistent storage of user videos
- ✅ Temporary files automatically cleaned up
- ⚠️ Consider adding authentication for production use
- ⚠️ Consider rate limiting for public deployment

## 📝 License

This project is for demonstration purposes.

## 🤝 Contributing

Feel free to submit issues or pull requests!

## 📧 Support

For issues or questions, please create an issue in the repository.
