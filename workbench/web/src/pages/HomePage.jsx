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
  CheckCircle,
  ArrowRight,
  Monitor,
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
      description: 'Single-thread, multi-thread, and mixed workload tests that mirror real development tasks.',
    },
    {
      icon: HardDrive,
      title: 'Disk Performance',
      description: 'File enumeration, random reads, metadata operations, and directory traversal tests.',
    },
    {
      icon: Zap,
      title: 'Latency Tests',
      description: 'Process spawn, thread wake, and storage latency measurements for responsiveness.',
    },
    {
      icon: Monitor,
      title: 'Real-World Apps',
      description: 'Cargo build, C# compile, archive operations, and PowerShell script execution.',
    },
    {
      icon: GitCompare,
      title: 'Compare Results',
      description: 'Side-by-side comparison with community results or your own historical runs.',
    },
    {
      icon: Globe,
      title: 'Community Database',
      description: 'Share your results and see how your machine stacks up against others.',
    },
  ]

  const benchmarks = [
    'File Enumeration',
    'Random Read',
    'Metadata Operations',
    'Directory Traversal',
    'Large File Read',
    'Single-Thread CPU',
    'Multi-Thread CPU',
    'Mixed Workload',
    'Cargo Build',
    'C# Compile',
    'Archive Operations',
    'PowerShell Scripts',
    'Process Spawn',
    'Thread Wake',
    'Storage Latency',
    'Memory Bandwidth',
    'Memory Latency',
    'Registry Operations',
    'Windows Services',
    'Network Tools',
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
              <span className="text-wb-accent-light"> Developer Workstation</span>
            </h1>
            <p className="text-xl text-wb-text-secondary mb-4 max-w-2xl mx-auto">
              <strong className="text-wb-text-primary">Measure what matters for developers.</strong> Benchmark your workstation's real-world performance—CPU, disk, memory, and the apps you actually use like compilers and PowerShell.
            </p>
            <p className="text-lg text-wb-text-secondary mb-8 max-w-2xl mx-auto">
              <strong className="text-wb-text-primary">Know your machine. Optimize your workflow.</strong> Get actionable insights to identify bottlenecks and compare systems, so you spend less time waiting and more time coding.
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
              Built for Developers
            </h2>
            <p className="text-wb-text-secondary text-lg max-w-2xl mx-auto">
              Unlike generic benchmarks, WorkBench focuses on the operations that matter
              for your daily development workflow.
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {features.map((feature, i) => (
              <div key={i} className="card hover:border-wb-accent/50 transition-colors">
                <feature.icon className="text-wb-accent-light mb-4" size={32} />
                <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                <p className="text-wb-text-secondary">{feature.description}</p>
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
            <p className="text-wb-text-secondary text-lg">
              Over 30 individual tests across multiple categories
            </p>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-3">
            {benchmarks.map((name, i) => (
              <div
                key={i}
                className="flex items-center gap-2 px-4 py-3 bg-wb-bg-card rounded-lg border border-wb-border"
              >
                <CheckCircle size={16} className="text-wb-success flex-shrink-0" />
                <span className="text-sm truncate">{name}</span>
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
            <h2 className="text-3xl font-bold mb-4">Download WorkBench</h2>
            <p className="text-wb-text-secondary mb-8">
              Available for Windows 10/11. No installation required - just download and run.
            </p>

            <div className="space-y-4">
              <a
                href="https://github.com/johanmcad/workbench/releases/latest/download/workbench.exe"
                className="btn-primary inline-flex items-center gap-2 text-lg"
              >
                <Download size={20} />
                Download for Windows (x64)
              </a>

              <p className="text-sm text-wb-text-secondary">
                Or view all releases on{' '}
                <a
                  href="https://github.com/johanmcad/workbench/releases"
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
