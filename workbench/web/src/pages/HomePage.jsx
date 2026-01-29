import { Link } from 'react-router-dom'
import { useEffect, useState } from 'react'
import {
  Download,
  Cpu,
  HardDrive,
  Zap,
  BarChart3,
  GitCompare,
  Globe,
  ArrowRight,
  Monitor,
  AlertCircle,
  Activity,
  Settings,
  MemoryStick,
} from 'lucide-react'
import { fetchStats } from '../api'

export default function HomePage() {
  const [stats, setStats] = useState({ totalRuns: 0, uniqueCpus: 0 })

  useEffect(() => {
    fetchStats().then(setStats).catch(console.error)
  }, [])

  const features = [
    {
      icon: Cpu,
      title: 'CPU Benchmarks',
      description: 'Single-thread, multi-thread, and mixed workload tests that reflect real-world professional tasks.',
    },
    {
      icon: HardDrive,
      title: 'Disk Performance',
      description: 'File enumeration, random reads, metadata operations, and directory traversal tests.',
    },
    {
      icon: Zap,
      title: 'Latency Tests',
      description: 'Process spawn, thread wake, and storage latency measurements for system responsiveness.',
    },
    {
      icon: Monitor,
      title: 'Real-World Applications',
      description: 'Build tools, archive operations, PowerShell scripts, and other professional workloads.',
    },
    {
      icon: GitCompare,
      title: 'Compare Results',
      description: 'Side-by-side comparison with community results or your own historical runs.',
    },
    {
      icon: Globe,
      title: 'Community Database',
      description: 'Share your results and see how your machine compares to others.',
    },
  ]

  const benchmarkGroups = [
    {
      category: 'Disk',
      icon: HardDrive,
      tests: [
        { name: 'File Enumeration', description: 'Simulates VS solution load, git status - measures how fast your system can list files across directories.' },
        { name: 'Random Read', description: 'Simulates loading source files - measures random 4KB read performance from storage.' },
        { name: 'Large File Read', description: 'Simulates opening large CAD files - measures sequential read throughput.' },
        { name: 'Metadata Operations', description: 'Simulates npm install, build temp files - measures file create/write/delete speed.' },
        { name: 'Directory Traversal', description: 'Simulates search in files - measures enumeration + reading file contents.' },
        { name: 'Storage Latency', description: 'Measures P50/P95/P99 read latency - critical for system responsiveness.' },
      ],
    },
    {
      category: 'CPU',
      icon: Cpu,
      tests: [
        { name: 'Single-Thread', description: 'Simulates single-file compilation - measures single-core compute performance.' },
        { name: 'Multi-Thread', description: 'Simulates parallel build - measures how well your CPU scales across all cores.' },
        { name: 'Mixed Workload', description: 'Simulates full build cycle - combines file I/O with CPU compression work.' },
      ],
    },
    {
      category: 'Memory',
      icon: MemoryStick,
      tests: [
        { name: 'Bandwidth', description: 'Multi-threaded memory copy - measures RAM throughput in GB/s.' },
        { name: 'Latency', description: 'Pointer-chasing benchmark - measures memory access latency.' },
      ],
    },
    {
      category: 'Responsiveness',
      icon: Activity,
      tests: [
        { name: 'Process Spawn', description: 'Simulates running build tools - measures how fast new processes can start.' },
        { name: 'App Launch', description: 'Launches Notepad, WordPad, Calculator, and more - measures real application startup time.' },
        { name: 'Thread Wake', description: 'Simulates async operations - measures thread synchronization latency.' },
      ],
    },
    {
      category: 'Windows & Apps',
      icon: Settings,
      tests: [
        { name: 'PowerShell', description: 'Execute various PowerShell operations - measures script execution speed.' },
        { name: 'Archive Ops', description: 'Compress and extract files using tar - measures archiving speed.' },
        { name: 'Registry', description: 'Query Windows registry - measures registry read performance.' },
        { name: 'Services', description: 'Query Windows services - measures system management overhead.' },
        { name: 'Network', description: 'DNS resolution, adapter queries - measures network subsystem speed.' },
      ],
    },
  ]

  return (
    <div>
      {/* Hero Section */}
      <section className="relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-wb-accent/20 via-transparent to-transparent" />
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 md:py-32 relative">
          <div className="text-center max-w-4xl mx-auto">
            <h1 className="text-4xl md:text-6xl font-bold mb-6">
              Benchmark Your
              <span className="text-wb-accent-light"> Workstation</span>
            </h1>
            <p className="text-xl text-wb-text-secondary mb-4 max-w-2xl mx-auto">
              <strong className="text-wb-text-primary">Measure what matters in real-world use.</strong> Benchmark your workstation's actual performance—CPU, disk, memory, and everyday applications like compilers, scripts, and file operations.
            </p>
            <p className="text-lg text-wb-text-secondary mb-8 max-w-2xl mx-auto">
              Not a GPU or gaming benchmark. Focused on <strong className="text-wb-text-primary">system responsiveness</strong> and <strong className="text-wb-text-primary">everyday usage</strong>—the things that affect how fast your machine feels.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <a href="#download" className="btn-primary flex items-center justify-center gap-2 text-lg">
                <Download size={20} />
                Download for Windows
              </a>
              <Link to="/results" className="btn-secondary flex items-center justify-center gap-2 text-lg">
                <BarChart3 size={20} />
                View Community Results
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="border-y border-wb-border bg-wb-bg-card/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
            <div>
              <div className="text-3xl md:text-4xl font-bold text-wb-accent-light">30+</div>
              <div className="text-wb-text-secondary mt-1">Benchmarks</div>
            </div>
            <div>
              <div className="text-3xl md:text-4xl font-bold text-wb-accent-light">{stats.totalRuns}</div>
              <div className="text-wb-text-secondary mt-1">Community Results</div>
            </div>
            <div>
              <div className="text-3xl md:text-4xl font-bold text-wb-accent-light">{stats.uniqueCpus}</div>
              <div className="text-wb-text-secondary mt-1">Unique CPUs</div>
            </div>
            <div>
              <div className="text-3xl md:text-4xl font-bold text-wb-accent-light">Free</div>
              <div className="text-wb-text-secondary mt-1">Open Source</div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              Real-World Performance Testing
            </h2>
            <p className="text-wb-text-secondary text-lg max-w-2xl mx-auto">
              Unlike synthetic benchmarks, WorkBench-Pro measures the operations that matter
              for your daily professional workflow.
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {features.map((feature, i) => (
              <div
                key={i}
                className="group relative rounded-2xl border border-wb-border bg-gradient-to-br from-wb-accent/10 to-transparent p-6 transition-all duration-300 hover:scale-[1.02] hover:shadow-xl hover:border-wb-accent/50"
              >
                <div className="p-2 rounded-xl bg-wb-bg-card/80 backdrop-blur w-fit mb-4">
                  <feature.icon className="text-wb-accent-light" size={24} />
                </div>
                <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                <p className="text-wb-text-secondary">{feature.description}</p>
                <div className="absolute inset-0 overflow-hidden pointer-events-none rounded-2xl">
                  <div className="absolute -bottom-8 -right-8 opacity-5 group-hover:opacity-10 transition-opacity duration-300">
                    <feature.icon size={120} />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Benchmarks List */}
      <section className="py-20 bg-wb-bg-card/30">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              Comprehensive Test Suite
            </h2>
            <p className="text-wb-text-secondary text-lg max-w-3xl mx-auto mb-4">
              Over 30 individual tests across multiple categories
            </p>
            <p className="text-wb-text-secondary max-w-2xl mx-auto">
              <strong className="text-wb-accent-light">Not a synthetic benchmark.</strong>{' '}
              Unlike traditional benchmarks that measure peak theoretical performance, WorkBench-Pro tests the operations you actually use every day—how fast your system <em>feels</em>, not just how fast it can run artificial loops.
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {benchmarkGroups.map((group, groupIdx) => (
              <div
                key={groupIdx}
                className="group relative rounded-2xl border border-wb-border bg-gradient-to-br from-wb-accent/10 to-transparent p-6 transition-all duration-300 hover:scale-[1.02] hover:shadow-xl hover:border-wb-accent/50"
              >
                <div className="flex items-center gap-3 mb-4">
                  <div className="p-2 rounded-xl bg-wb-bg-card/80 backdrop-blur">
                    <group.icon size={24} className="text-wb-accent-light" />
                  </div>
                  <h3 className="text-lg font-bold">{group.category}</h3>
                </div>
                <div className="flex flex-wrap gap-2">
                  {group.tests.map((benchmark, i) => (
                    <div
                      key={i}
                      className="group/item relative"
                    >
                      <span className="inline-block px-3 py-1.5 text-sm bg-wb-bg-card/60 backdrop-blur-sm rounded-full border border-wb-border/50 hover:bg-wb-bg-card hover:border-wb-accent/50 transition-all duration-200 cursor-default">
                        {benchmark.name}
                      </span>
                      <div className="absolute left-1/2 -translate-x-1/2 bottom-full mb-2 z-50 opacity-0 invisible group-hover/item:opacity-100 group-hover/item:visible transition-all duration-200 pointer-events-none">
                        <div className="w-64 p-3 bg-wb-bg-card border border-wb-border rounded-lg shadow-xl">
                          <p className="text-xs text-wb-text-secondary leading-relaxed">{benchmark.description}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
                <div className="absolute inset-0 overflow-hidden pointer-events-none rounded-2xl">
                  <div className="absolute -bottom-8 -right-8 opacity-5 group-hover:opacity-10 transition-opacity duration-300">
                    <group.icon size={120} />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Download Section */}
      <section id="download" className="py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="card max-w-3xl mx-auto text-center">
            <Download className="mx-auto text-wb-accent-light mb-6" size={48} />
            <h2 className="text-3xl font-bold mb-4">Download WorkBench-Pro</h2>
            <p className="text-wb-text-secondary mb-8">
              Available for Windows 10/11. No installation required - just download and run.
            </p>

            <div className="space-y-4">
              <a
                href="https://github.com/johanmcad/WorkBenchPro/releases/latest/download/workbench-pro.exe"
                className="btn-primary inline-flex items-center gap-2 text-lg"
              >
                <Download size={20} />
                Download for Windows (x64)
              </a>

              <p className="text-sm text-wb-text-secondary">
                Or view all releases on{' '}
                <a
                  href="https://github.com/johanmcad/WorkBenchPro/releases"
                  className="text-wb-accent-light hover:underline"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  GitHub Releases
                </a>
              </p>
            </div>

            <div className="mt-8 pt-8 border-t border-wb-border">
              <h3 className="font-semibold mb-3">System Requirements</h3>
              <ul className="text-wb-text-secondary text-sm space-y-1">
                <li>Windows 10 or Windows 11</li>
                <li>4 GB RAM minimum (8+ GB recommended)</li>
                <li>~100 MB disk space for tests</li>
              </ul>
            </div>

            <div className="mt-6 pt-6 border-t border-wb-border">
              <div className="flex items-start gap-3 text-left bg-wb-bg-secondary/50 rounded-lg p-4">
                <AlertCircle size={20} className="text-wb-accent-light flex-shrink-0 mt-0.5" />
                <div>
                  <h4 className="font-semibold text-sm mb-2">First time running?</h4>
                  <p className="text-wb-text-secondary text-sm mb-2">
                    After downloading, you may need to unblock the file:
                  </p>
                  <ol className="text-wb-text-secondary text-sm space-y-1 list-decimal list-inside">
                    <li>Right-click <code className="bg-wb-bg-card px-1 rounded">workbench-pro.exe</code></li>
                    <li>Select <strong className="text-wb-text-primary">Properties</strong></li>
                    <li>Check <strong className="text-wb-text-primary">Unblock</strong> at the bottom</li>
                    <li>Click <strong className="text-wb-text-primary">OK</strong></li>
                  </ol>
                  <ul className="text-wb-text-secondary text-xs mt-3 space-y-1">
                    <li><strong className="text-wb-text-primary">Portable:</strong> No installation or admin access required</li>
                    <li><strong className="text-wb-text-primary">Privacy:</strong> No files saved to your computer, no personal data collected</li>
                    <li><strong className="text-wb-text-primary">Open source:</strong> Full source code available on GitHub</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 bg-gradient-to-r from-wb-accent/20 to-wb-bg-secondary/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            See How Your Machine Compares
          </h2>
          <p className="text-wb-text-secondary text-lg mb-8 max-w-2xl mx-auto">
            Browse community results and compare your workstation against similar configurations.
          </p>
          <Link
            to="/results"
            className="btn-primary inline-flex items-center gap-2 text-lg"
          >
            Browse Community Results
            <ArrowRight size={20} />
          </Link>
        </div>
      </section>
    </div>
  )
}
