import { Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { Cpu, HardDrive, Activity, Settings, BarChart3, Shield } from 'lucide-react'

export default function AboutPage() {
  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <Helmet>
        <title>About WorkBench-Pro | Methodology & Real-World Benchmarking</title>
        <meta name="description" content="Learn how WorkBench-Pro measures real-world workstation performance. Our methodology tests actual developer and professional workflows, not synthetic loops." />
        <link rel="canonical" href="https://www.workbench-pro.com/about" />
        <meta property="og:url" content="https://www.workbench-pro.com/about" />
        <meta property="og:title" content="About WorkBench-Pro | Methodology & Real-World Benchmarking" />
        <meta property="og:description" content="Learn how WorkBench-Pro measures real-world workstation performance with actual developer workflows." />
        <meta property="twitter:url" content="https://www.workbench-pro.com/about" />
        <meta property="twitter:title" content="About WorkBench-Pro | Methodology & Real-World Benchmarking" />
        <meta property="twitter:description" content="Learn how WorkBench-Pro measures real-world workstation performance with actual developer workflows." />
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "About" }
          ]
        })}</script>
      </Helmet>

      <h1 className="text-4xl font-bold mb-8">About WorkBench-Pro</h1>

      <div className="prose prose-invert max-w-none space-y-8">
        <section>
          <h2 className="text-2xl font-semibold mb-4">Why Another Benchmark?</h2>
          <p className="text-wb-text-secondary leading-relaxed">
            Most benchmarks measure peak theoretical performance — how fast your CPU can run a tight mathematical loop,
            or how many sequential megabytes per second your SSD can push. Those numbers look impressive but rarely reflect
            how your system actually <em>feels</em> when you're working.
          </p>
          <p className="text-wb-text-secondary leading-relaxed mt-4">
            WorkBench-Pro measures the things that actually affect your day-to-day experience: how fast your IDE loads a
            project, how quickly a build completes, how responsive your system is when spawning processes, and how your
            storage handles the mixed random I/O patterns that real applications generate.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-semibold mb-4">Methodology</h2>
          <p className="text-wb-text-secondary leading-relaxed mb-6">
            Every test in WorkBench-Pro is designed to simulate a real workflow. We don't use synthetic loops or artificial
            patterns — each benchmark mirrors an actual operation you'd perform on a professional workstation.
          </p>

          <div className="grid md:grid-cols-2 gap-4">
            <div className="p-4 rounded-lg border border-wb-border bg-wb-bg-card/50">
              <div className="flex items-center gap-2 mb-2">
                <HardDrive size={20} className="text-wb-accent-light" />
                <h3 className="font-semibold">Project Operations</h3>
              </div>
              <p className="text-wb-text-secondary text-sm">
                File enumeration (simulates VS solution load), random reads (simulates opening source files),
                metadata operations (simulates npm install), and directory traversal (simulates search-in-files).
              </p>
            </div>
            <div className="p-4 rounded-lg border border-wb-border bg-wb-bg-card/50">
              <div className="flex items-center gap-2 mb-2">
                <Cpu size={20} className="text-wb-accent-light" />
                <h3 className="font-semibold">Build Performance</h3>
              </div>
              <p className="text-wb-text-secondary text-sm">
                Single-thread (simulates single-file compilation), multi-thread (simulates parallel build), and mixed
                workloads that combine file I/O with CPU compression — just like a real build cycle.
              </p>
            </div>
            <div className="p-4 rounded-lg border border-wb-border bg-wb-bg-card/50">
              <div className="flex items-center gap-2 mb-2">
                <Activity size={20} className="text-wb-accent-light" />
                <h3 className="font-semibold">Responsiveness</h3>
              </div>
              <p className="text-wb-text-secondary text-sm">
                Process spawn time (how fast build tools start), thread wake latency (async operation speed),
                and storage latency percentiles (P50/P95/P99) that determine system feel.
              </p>
            </div>
            <div className="p-4 rounded-lg border border-wb-border bg-wb-bg-card/50">
              <div className="flex items-center gap-2 mb-2">
                <Settings size={20} className="text-wb-accent-light" />
                <h3 className="font-semibold">Windows System</h3>
              </div>
              <p className="text-wb-text-secondary text-sm">
                PowerShell execution, archive operations with tar, registry queries, Windows services enumeration,
                and network subsystem tests (DNS, adapter queries).
              </p>
            </div>
          </div>
        </section>

        <section>
          <h2 className="text-2xl font-semibold mb-4">How Scoring Works</h2>
          <p className="text-wb-text-secondary leading-relaxed">
            Each test result is compared against the community database. Your score shows how many tests performed
            above the median (50th percentile). A score of 20/25 means 20 of your 25 test results were better than
            at least half the community. This gives you a practical sense of where your machine stands without
            reducing everything to a single abstract number.
          </p>
        </section>

        <section>
          <h2 className="text-2xl font-semibold mb-4">Privacy & Transparency</h2>
          <div className="flex items-start gap-3 p-4 rounded-lg border border-wb-border bg-wb-bg-card/50">
            <Shield size={20} className="text-wb-accent-light mt-0.5" />
            <div className="text-wb-text-secondary text-sm space-y-2">
              <p>WorkBench-Pro is fully open source. The complete source code is available on GitHub.</p>
              <p>No files are saved to your computer. No personal data is collected — only hardware specs
                and benchmark scores are uploaded, and only if you choose to share your results.</p>
              <p>The application is portable (no installation required) and needs no admin privileges to run.</p>
            </div>
          </div>
        </section>

        <div className="flex gap-4 pt-4">
          <Link to="/results" className="btn-primary inline-flex items-center gap-2">
            <BarChart3 size={18} />
            View Results
          </Link>
          <a
            href="https://github.com/johanmcad/WorkBenchPro"
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary inline-flex items-center gap-2"
          >
            View Source Code
          </a>
        </div>
      </div>
    </div>
  )
}
