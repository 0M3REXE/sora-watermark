# 🚀 Deploying Sora AI Watermark Service to Render

This guide will help you deploy your Rust backend to Render.com with Docker.

## 📋 Prerequisites

1. **GitHub Account** - Your code needs to be in a GitHub repository
2. **Render Account** - Sign up at https://render.com (free tier available)
3. **Git installed** - To push your code to GitHub

## 🔧 Step 1: Prepare Your Code

### 1.1 Create a GitHub Repository

```powershell
# Initialize git if not already done
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Sora AI Watermark Service"

# Create a new repository on GitHub (https://github.com/new)
# Then link and push:
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
git branch -M main
git push -u origin main
```

### 1.2 Verify Required Files

Make sure these files are in your repository:
- ✅ `Dockerfile` - Container configuration
- ✅ `render.yaml` - Render deployment configuration
- ✅ `.dockerignore` - Files to exclude from Docker build
- ✅ `sora-watermark.mp4` - Your watermark video
- ✅ `Cargo.toml` & `Cargo.lock` - Rust dependencies
- ✅ `src/main.rs` - Application code
- ✅ `static/index.html` - Frontend

## 🌐 Step 2: Deploy to Render

### 2.1 Connect GitHub to Render

1. Go to https://dashboard.render.com
2. Click **"New +"** → **"Web Service"**
3. Click **"Connect GitHub"** and authorize Render
4. Select your repository

### 2.2 Configure the Service

Render will auto-detect the `render.yaml` file. Verify these settings:

- **Name**: `sora-watermark-service`
- **Environment**: `Docker`
- **Region**: Choose closest to your users
- **Branch**: `main`
- **Plan**: 
  - **Free** (limited, sleeps after inactivity) - Good for testing
  - **Starter** ($7/month) - Recommended for production
  - **Standard** ($25/month) - For higher traffic

### 2.3 Environment Variables (Optional)

These are set in `render.yaml`, but you can override in the dashboard:
- `BIND_ADDRESS`: `0.0.0.0:8000` (default)
- `RUST_LOG`: `info` (logging level)

### 2.4 Deploy!

1. Click **"Create Web Service"**
2. Render will:
   - Build your Docker image (takes 5-10 minutes first time)
   - Install FFmpeg
   - Start your service
3. Monitor the build logs in real-time

## 🎯 Step 3: Access Your Service

### 3.1 Get Your URL

Once deployed, Render provides a URL like:
```
https://sora-watermark-service.onrender.com
```

### 3.2 Test the Service

**Health Check:**
```bash
curl https://sora-watermark-service.onrender.com/health
```

**Web Interface:**
Open in browser: `https://sora-watermark-service.onrender.com`

**API Endpoint:**
```bash
curl -X POST -F "video=@your-video.mp4" \
  https://sora-watermark-service.onrender.com/api/process \
  --output watermarked-video.mp4
```

## ⚙️ Configuration Options

### Modify render.yaml for Your Needs

**Change Plan:**
```yaml
plan: starter  # Options: free, starter, standard, pro
```

**Adjust Workers:**
In `src/main.rs`, line 186:
```rust
.workers(4) // Increase for more concurrent users
```

**Change Video Quality:**
In `src/main.rs`, lines 132-133:
```rust
"-preset", "slow",   // Options: ultrafast, fast, medium, slow, veryslow
"-crf", "18",        // Range: 0-51 (lower = better quality)
```

## 🔒 Important Notes

### Free Tier Limitations
- **Spin Down**: Service sleeps after 15 minutes of inactivity
- **Spin Up**: Takes ~30 seconds to wake up
- **Memory**: Limited RAM (may fail on large videos)
- **Monthly Hours**: 750 hours/month free

### Production Recommendations
- Use **Starter** plan minimum ($7/month)
- Add custom domain
- Enable **Auto-Deploy** from GitHub (already enabled)
- Set up monitoring and alerts

## 🐛 Troubleshooting

### Build Fails

**Problem**: Docker build times out
```
Solution: Render free tier has limited resources
→ Try: Push to GitHub and let Render build
→ Or: Upgrade to Starter plan
```

**Problem**: FFmpeg not found
```
Solution: Check Dockerfile has ffmpeg installation
→ Line 17 should have: apt-get install -y ffmpeg
```

### Runtime Issues

**Problem**: Videos fail to process
```
Check logs in Render dashboard:
→ FFmpeg errors indicate codec issues
→ Memory errors indicate file too large
→ Timeout errors indicate video too long
```

**Problem**: Service not responding
```
Check if service is sleeping (free tier):
→ First request takes ~30s to wake up
→ Consider Starter plan for always-on
```

### Logs

View logs in Render dashboard:
```
Dashboard → Your Service → Logs
```

Or use Render CLI:
```bash
npm install -g @render/cli
render login
render logs sora-watermark-service
```

## 📊 Monitoring

### Health Check Endpoint
```bash
# Check if service is running
curl https://your-app.onrender.com/health

# Expected response:
{
  "status": "ok",
  "service": "Sora AI Watermark Service"
}
```

### Render Dashboard
Monitor in real-time:
- CPU usage
- Memory usage
- Request count
- Response times
- Error rates

## 🔄 Updating Your Service

### Deploy New Changes

```powershell
# Make your changes to the code
git add .
git commit -m "Your update message"
git push origin main

# Render auto-deploys (takes 5-10 min)
```

### Manual Deploy
In Render dashboard:
1. Go to your service
2. Click **"Manual Deploy"**
3. Select branch
4. Click **"Deploy"**

## 💰 Cost Estimate

### Free Tier
- **Cost**: $0/month
- **Good for**: Testing, demos, low traffic
- **Limitations**: Sleeps after inactivity, limited resources

### Starter Plan
- **Cost**: $7/month
- **Good for**: Small production apps
- **Features**: Always on, 512 MB RAM, better CPU

### Standard Plan
- **Cost**: $25/month
- **Good for**: Production with traffic
- **Features**: 2 GB RAM, better performance

### Bandwidth
- Free tier: 100 GB/month
- Overage: $0.10/GB
- **Note**: Video processing uses significant bandwidth!

## 🎨 Custom Domain (Optional)

1. Go to your service in Render dashboard
2. Click **"Settings"** → **"Custom Domain"**
3. Add your domain (e.g., `watermark.yourdomain.com`)
4. Update DNS records as instructed
5. Render provides free SSL certificate

## 🔐 Security Considerations

### For Production:

1. **Add Authentication**
   - Implement API keys
   - Add user authentication
   - Rate limiting

2. **Environment Variables**
   - Store sensitive data in Render env vars
   - Never commit secrets to Git

3. **CORS Configuration**
   - Restrict allowed origins
   - Update in `main.rs` if needed

4. **File Size Limits**
   - Already set in code (500MB)
   - Adjust in `src/main.rs` line 12

## 📱 Frontend Deployment

Your frontend is bundled with the backend (served from `/`). 

### Separate Frontend (Optional)
If you want to deploy frontend separately (e.g., on Vercel):

1. Extract `static/index.html` to separate repo
2. Update API endpoint in JavaScript:
```javascript
const API_URL = 'https://sora-watermark-service.onrender.com';
fetch(`${API_URL}/api/process`, {...});
```
3. Deploy to Vercel/Netlify
4. Update CORS settings in Rust backend

## ✅ Final Checklist

Before going live:
- [ ] Code pushed to GitHub
- [ ] `sora-watermark.mp4` committed to repo
- [ ] Render service created and deployed
- [ ] Health check endpoint working
- [ ] Test video upload works
- [ ] Monitor logs for errors
- [ ] Consider upgrading from free tier
- [ ] Set up custom domain (optional)
- [ ] Add authentication (production)

## 🆘 Support

**Render Documentation**: https://render.com/docs
**Render Community**: https://community.render.com
**FFmpeg Documentation**: https://ffmpeg.org/documentation.html

## 🎉 Success!

Your Sora AI Watermark service is now live! 🚀

**Your URLs:**
- Web Interface: `https://your-service.onrender.com`
- API Endpoint: `https://your-service.onrender.com/api/process`
- Health Check: `https://your-service.onrender.com/health`

Share the web interface URL with users to upload videos and apply the Sora AI watermark!
