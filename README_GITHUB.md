# Sora AI Watermark Service 🎬

A high-performance Rust backend service that applies Sora AI watermarks to videos with automatic green screen removal.

## ✨ Features

- 🎥 **Video Processing**: Apply Sora AI watermark overlay to any video
- 🎨 **Green Screen Removal**: Automatic chroma key removal using FFmpeg
- ⚡ **Fast & Efficient**: Built with Rust and Actix-web
- 🔄 **Concurrent Processing**: Handles multiple users simultaneously
- 🔒 **Privacy Focused**: No server-side storage of videos
- 🌐 **Web Interface**: Beautiful, responsive UI included
- 📱 **API Ready**: RESTful API for programmatic access

## 🚀 Quick Start

### Local Development

```bash
# Prerequisites: Rust, FFmpeg 8.0+, sora-watermark.mp4

# Build
cargo build --release

# Run
cargo run --release

# Visit http://127.0.0.1:8000
```

### Deploy to Render

See [DEPLOYMENT.md](DEPLOYMENT.md) for complete deployment instructions.

**Quick Deploy:**
1. Push to GitHub
2. Connect to Render
3. Deploy with Docker
4. Done! ✅

## 📡 API Endpoints

### `GET /`
Web interface for video upload

### `GET /health`
Health check endpoint
```json
{"status": "ok", "service": "Sora AI Watermark Service"}
```

### `POST /api/process`
Process video with watermark overlay

**Request:**
- Method: `POST`
- Content-Type: `multipart/form-data`
- Body: Video file

**Response:**
- Content-Type: `video/mp4`
- Body: Processed video with watermark

**Example:**
```bash
curl -X POST -F "video=@input.mp4" \
  http://localhost:8000/api/process \
  --output watermarked.mp4
```

## 🛠️ Tech Stack

- **Backend**: Rust, Actix-web
- **Video Processing**: FFmpeg 8.0
- **Concurrency**: Tokio async runtime
- **Deployment**: Docker, Render.com

## 📝 Configuration

Edit `src/main.rs`:
```rust
const MAX_FILE_SIZE: usize = 500 * 1024 * 1024; // 500MB
const WATERMARK_PATH: &str = "sora-watermark.mp4";
```

Quality settings (line 132-133):
```rust
"-preset", "slow",  // ultrafast, fast, medium, slow, veryslow
"-crf", "18",       // 0-51 (lower = better quality)
```

## 📦 Project Structure

```
webtest/
├── src/
│   └── main.rs              # Main application
├── static/
│   └── index.html           # Web interface
├── Dockerfile               # Docker configuration
├── render.yaml              # Render deployment config
├── Cargo.toml               # Rust dependencies
├── sora-watermark.mp4       # Watermark video (with green screen)
└── DEPLOYMENT.md            # Deployment guide
```

## 🔧 Requirements

- **Rust** 1.75+
- **FFmpeg** 8.0+ (with chromakey/colorkey support)
- **sora-watermark.mp4** (green screen watermark video)

## 📖 Documentation

- [DEPLOYMENT.md](DEPLOYMENT.md) - Complete deployment guide
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - Project overview
- [README.md](README.md) - Full documentation

## 🐛 Troubleshooting

**Watermark not visible?**
- Check green screen in watermark video
- Adjust colorkey parameters in `main.rs`

**FFmpeg not found?**
- Install FFmpeg 8.0+
- Add to system PATH

**Build errors?**
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

## 📄 License

This project is for demonstration purposes.

## 🤝 Contributing

Issues and pull requests welcome!

## 📧 Support

For issues, please create a GitHub issue or refer to [DEPLOYMENT.md](DEPLOYMENT.md).

---

**Live Demo**: [Deploy your own!](DEPLOYMENT.md)

Made with ❤️ and Rust 🦀
