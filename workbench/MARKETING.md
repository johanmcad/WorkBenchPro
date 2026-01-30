# WorkBench-Pro Marketing Guide

## Quick Links
- **Download**: https://github.com/johanmcad/WorkBenchPro/releases
- **Results Browser**: https://www.workbench-pro.com/results
- **GitHub**: https://github.com/johanmcad/WorkBenchPro

---

## 1. Social Media Image (og-image.png)

Create a 1200x630px image with:
- "WorkBench-Pro" title
- "Free PC Benchmark for Developers" subtitle
- Screenshot of the app or branded graphic
- Dark theme to match the app

**Tools**: Canva (free), Figma, or screenshot

**Save to**: `web/public/og-image.png`

---

## 2. Reddit Posts

### r/programming, r/webdev

**Title**: `I built a free PC benchmark designed for developers - WorkBench-Pro`

**Body**:
```
Hey everyone,

I got frustrated that benchmarks like Cinebench and CrystalDiskMark don't measure what actually matters for development work. So I built WorkBench-Pro.

**What it tests:**
- File I/O (enumeration, random reads, metadata ops)
- Git operations performance
- C# compilation speed
- CPU single/multi-thread with real compression workloads
- Memory bandwidth & latency
- Windows Defender impact on file operations
- Process spawn time, thread wake latency

**Features:**
- Compare your results with the community
- Identify bottlenecks (slow tests highlighted in red)
- Free & open source (Rust + React)

Download: https://github.com/johanmcad/WorkBenchPro/releases
Results browser: https://www.workbench-pro.com/results

Would love feedback on what tests to add. What operations slow you down the most?
```

### r/windows

**Title**: `Free benchmark tool that measures real developer workloads on Windows`

**Body**:
```
Built a benchmark specifically for Windows developer workstations. Unlike synthetic benchmarks, it tests:

- File enumeration (30,000 files across 500 directories)
- Git operations on large repos
- C# compilation with dotnet build
- Windows Defender impact measurement
- Registry, services, and WMI query performance
- PowerShell script execution speed

Results are compared against the community so you can see how your machine stacks up.

Download: https://github.com/johanmcad/WorkBenchPro/releases
Browse results: https://www.workbench-pro.com/results

Free and open source. Would love to hear what Windows-specific tests would be useful!
```

### r/buildapc

**Title**: `Built a benchmark that tests what actually matters for coding - file I/O, compilation, git ops`

**Body**:
```
If you're building a PC for development work, Cinebench scores don't tell the whole story.

I built WorkBench-Pro to test real developer workflows:
- How fast can it enumerate 30,000 project files?
- Git status/diff performance on large repos
- C# compilation speed
- NVMe random read latency
- Process spawn overhead

After collecting community results, found some interesting things:
- NVMe vs SATA makes 3-5x difference in file enumeration
- Windows Defender can add 2-5ms overhead per file operation
- Thread wake latency varies a lot between CPUs

Try it: https://github.com/johanmcad/WorkBenchPro/releases
Compare: https://www.workbench-pro.com/results
```

**Posting strategy**: Space posts 2-3 days apart to avoid looking spammy.

---

## 3. Dev.to / Hashnode Article

**Title**: `Why Generic Benchmarks Don't Tell the Full Story for Developers`

**Tags**: `productivity`, `programming`, `windows`, `performance`

```markdown
## The Problem with Cinebench

You just got a new laptop. Cinebench says it scores 15,000 points. Great! But then you open your project and `npm install` takes forever. What gives?

Generic benchmarks measure synthetic workloads that don't reflect developer workflows:
- **Cinebench**: Pure CPU rendering (when did you last render a 3D scene?)
- **CrystalDiskMark**: Sequential reads (how often do you read one giant file?)
- **Geekbench**: Mixed synthetic tests

## What Actually Matters for Developers

Your daily work involves:
- **File enumeration**: IDE scanning 50,000 files on startup
- **Random small reads**: Loading hundreds of source files
- **Git operations**: Status, diff, log on large repos
- **Process spawning**: Build tools launching thousands of processes
- **Compilation**: Actually building your code

## I Built Something Different

WorkBench-Pro measures these real-world operations:

| Test | What It Measures |
|------|-----------------|
| File Enumeration | Scanning 30,000 files across 500 directories |
| Git Operations | git status, diff, log on a 5,000 file repo |
| C# Compilation | Building a multi-file .NET project |
| Process Spawn | Launching 100 cmd.exe processes |
| Antivirus Impact | Windows Defender overhead on builds |

## Surprising Findings

After collecting community results, I found:
- NVMe vs SATA SSD makes a 3-5x difference in file enumeration
- Windows Defender can add 2-5ms per file operation
- Thread wake latency varies wildly between CPUs

## Try It Yourself

- **Download**: [GitHub Releases](https://github.com/johanmcad/WorkBenchPro/releases)
- **Compare Results**: [Community Browser](https://www.workbench-pro.com/results)

It's free and open source. I'd love to hear what tests you'd find useful!

---

*What benchmarks do you wish existed for your workflow? Let me know in the comments.*
```

---

## 4. Hacker News (Show HN)

**Title**: `Show HN: WorkBench-Pro – PC benchmark designed for developer workflows`

**URL**: `https://github.com/johanmcad/WorkBenchPro`

**Text** (optional, for self-post):
```
I built a benchmark tool that measures what actually matters for development: file I/O, git operations, compilation speed, process spawn time, and memory bandwidth.

Unlike Cinebench or CrystalDiskMark, it tests real operations like enumerating 30,000 files, running git status on large repos, and measuring Windows Defender overhead.

Built with Rust (desktop app) and React (results browser). Free and open source.

Download: https://github.com/johanmcad/WorkBenchPro/releases
Compare results: https://www.workbench-pro.com/results
```

**Best times to post**: Weekday mornings (US time), Tuesday-Thursday

---

## 5. Product Hunt Launch

**URL**: https://www.producthunt.com/

**Product Name**: WorkBench-Pro

**Tagline** (60 chars max): `Free PC benchmark designed for developers`

**Description**:
```
WorkBench-Pro measures what generic benchmarks miss: the real-world operations that matter for software development.

🔍 **What it tests:**
• File I/O (enumeration, random reads, metadata)
• Git operations (status, diff, log)
• C# compilation performance
• CPU single & multi-thread workloads
• Memory bandwidth & latency
• Windows Defender impact
• Process spawn time

📊 **Features:**
• Compare with community results
• Identify bottlenecks (slow tests highlighted)
• Detailed statistics and percentiles
• Free & open source

Built with Rust for the desktop app and React for the web results browser.
```

**Topics**: Developer Tools, Productivity, Open Source, Windows

**Best launch days**: Tuesday, Wednesday, Thursday

---

## 6. Custom Domain Setup

### Recommended Registrars (cheapest)
- **Porkbun**: Often cheapest, ~$8-10/year
- **Cloudflare Registrar**: At-cost pricing, ~$9-10/year
- **Namecheap**: ~$9-12/year

### Domain Ideas
- workbench-pro.com
- workbenchpro.dev
- devbenchmark.com
- workbench.dev

### Vercel Setup
1. Buy domain from registrar
2. Go to Vercel → Project → Settings → Domains
3. Add your domain (e.g., `workbench-pro.com`)
4. Add DNS records as shown by Vercel:
   - Type: `A`, Name: `@`, Value: `76.76.21.21`
   - Type: `CNAME`, Name: `www`, Value: `cname.vercel-dns.com`
5. Wait for DNS propagation (up to 48 hours, usually faster)
6. Update all URLs in:
   - `web/index.html` (canonical, og:url, etc.)
   - `web/public/sitemap.xml`
   - `web/public/robots.txt`
   - Resubmit sitemap in Google Search Console

---

## 7. GitHub Optimization

### Topics (already added)
- benchmark
- windows
- developer-tools
- performance
- cpu-benchmark
- ssd-benchmark
- rust

### README Tips
- Add badges (build status, downloads, license)
- Add screenshots/GIFs
- Clear installation instructions
- Link to web results browser

### GitHub Social Preview
- Go to repo Settings → Social preview
- Upload a 1280x640px image

---

## 8. Tracking & Analytics

### Google Search Console
- URL: https://search.google.com/search-console
- Check weekly for indexing progress
- Monitor search queries that find your site

### Check Indexing
Search on Google: `site:www.workbench-pro.com`

### Optional: Add Analytics
- Vercel Analytics (built-in, paid)
- Plausible (privacy-focused, paid)
- Google Analytics (free, privacy concerns)

---

## Checklist

- [ ] Create og-image.png (1200x630px)
- [ ] Post to r/programming
- [ ] Post to r/webdev (wait 2-3 days)
- [ ] Post to r/windows (wait 2-3 days)
- [ ] Post to r/buildapc (wait 2-3 days)
- [ ] Publish Dev.to article
- [ ] Submit to Hacker News (Show HN)
- [ ] Create Product Hunt account
- [ ] Schedule Product Hunt launch
- [ ] Buy custom domain
- [ ] Set up domain in Vercel
- [ ] Update URLs after domain change
- [ ] Resubmit sitemap with new domain
- [ ] Add GitHub social preview image
