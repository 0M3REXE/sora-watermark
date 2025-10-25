# 🚀 Quick Deployment Checklist

Follow these steps to deploy your Sora AI Watermark Service to Render.

## ☑️ Step 1: Prepare Repository

```powershell
# Initialize Git (if not done)
git init

# Add all files
git add .

# Commit
git commit -m "Deploy: Sora AI Watermark Service"
```

## ☑️ Step 2: Create GitHub Repository

1. Go to https://github.com/new
2. Create a new repository (e.g., `sora-watermark-service`)
3. **Important**: Do NOT initialize with README (we already have files)

```powershell
# Link to your GitHub repo
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git
git branch -M main
git push -u origin main
```

## ☑️ Step 3: Deploy to Render

### Option A: Blueprint (Recommended)
1. Go to https://dashboard.render.com
2. Click **"New +"** → **"Blueprint"**
3. Connect your GitHub repository
4. Render will read `render.yaml` and configure everything automatically
5. Click **"Apply"**

### Option B: Manual Setup
1. Go to https://dashboard.render.com
2. Click **"New +"** → **"Web Service"**
3. Connect GitHub and select your repository
4. Configure:
   - **Name**: `sora-watermark-service`
   - **Environment**: Docker
   - **Region**: Choose closest to you
   - **Branch**: `main`
   - **Dockerfile Path**: `./Dockerfile`
   - **Plan**: Free (or Starter for production)
5. Click **"Create Web Service"**

## ☑️ Step 4: Wait for Build

- First build takes **5-10 minutes**
- Watch the logs in Render dashboard
- Status will change to **"Live"** when ready

## ☑️ Step 5: Test Your Service

You'll get a URL like: `https://sora-watermark-service.onrender.com`

**Test it:**
```powershell
# Health check
curl https://YOUR-SERVICE.onrender.com/health

# Open in browser
start https://YOUR-SERVICE.onrender.com
```

## ⚠️ Important Notes

### Free Tier
- ✅ Good for testing
- ❌ Sleeps after 15 min inactivity (30s wake time)
- ❌ Limited resources (may fail on large videos)

### Production
- Upgrade to **Starter** plan ($7/mo) minimum
- Always-on, better performance
- More reliable for users

## 🔧 After Deployment

### Update Your Service
```powershell
# Make changes to code
git add .
git commit -m "Update: description of changes"
git push origin main

# Render auto-deploys in ~5-10 minutes
```

### Monitor
- Check **Logs** in Render dashboard
- Monitor CPU/Memory usage
- Set up alerts for errors

## ✅ Success Checklist

- [ ] Code pushed to GitHub
- [ ] `sora-watermark.mp4` is in repository
- [ ] Render service created
- [ ] Build completed successfully
- [ ] Health check returns 200 OK
- [ ] Web interface loads
- [ ] Test video processing works
- [ ] (Optional) Custom domain configured
- [ ] (Optional) Upgraded to paid plan

## 📞 Need Help?

- **Deployment Issues**: See [DEPLOYMENT.md](DEPLOYMENT.md)
- **Render Support**: https://community.render.com
- **Build Errors**: Check logs in Render dashboard

## 🎉 You're Done!

Your service is now live at: `https://your-service.onrender.com`

Share this URL with anyone who wants to add Sora AI watermarks to their videos!
