import { Helmet } from 'react-helmet-async'

const releases = [
  {
    version: '1.2.2',
    date: 'February 2026',
    changes: [
      'Added safe mode and lite benchmarks to avoid antivirus interference',
      'Improved screenshot hover zoom on website',
      'Added app screenshots to website homepage',
    ],
  },
  {
    version: '1.1.0',
    date: 'January 2026',
    changes: [
      'Added memory bandwidth and latency benchmarks',
      'Added thread wake latency test',
      'Added app launch benchmark (Notepad, WordPad, Calculator)',
      'Improved multi-thread CPU benchmark scaling',
      'Community results comparison with percentile ranks',
      'Side-by-side comparison page',
    ],
  },
  {
    version: '1.0.0',
    date: 'January 2026',
    changes: [
      'Initial release',
      'Project operations benchmarks: file enumeration, random read, metadata ops, directory traversal',
      'Build performance benchmarks: single-thread, multi-thread, mixed workload',
      'Responsiveness benchmarks: process spawn, storage latency',
      'Windows system benchmarks: PowerShell, archive ops, registry, services, network',
      'Community result upload and comparison',
      'Website with results browser',
    ],
  },
]

export default function ChangelogPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <Helmet>
        <title>Changelog - Version History | WorkBench-Pro</title>
        <meta name="description" content="WorkBench-Pro version history and changelog. See what's new in each release, including new benchmarks, features, and improvements." />
        <link rel="canonical" href="https://www.workbench-pro.com/changelog" />
        <meta property="og:url" content="https://www.workbench-pro.com/changelog" />
        <meta property="og:title" content="Changelog - Version History | WorkBench-Pro" />
        <meta property="og:description" content="WorkBench-Pro version history. See what's new in each release." />
        <meta property="twitter:url" content="https://www.workbench-pro.com/changelog" />
        <meta property="twitter:title" content="Changelog - Version History | WorkBench-Pro" />
        <meta property="twitter:description" content="WorkBench-Pro version history. See what's new in each release." />
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "Changelog" }
          ]
        })}</script>
      </Helmet>

      <h1 className="text-4xl font-bold mb-8">Changelog</h1>

      <div className="space-y-8">
        {releases.map((release) => (
          <div key={release.version} className="border border-wb-border rounded-lg p-6">
            <div className="flex items-center gap-3 mb-4">
              <span className="px-3 py-1 bg-wb-accent/20 text-wb-accent-light rounded-full text-sm font-semibold">
                v{release.version}
              </span>
              <span className="text-wb-text-secondary text-sm">{release.date}</span>
            </div>
            <ul className="space-y-2">
              {release.changes.map((change, i) => (
                <li key={i} className="flex items-start gap-2 text-wb-text-secondary text-sm">
                  <span className="text-wb-accent-light mt-1.5 shrink-0">•</span>
                  {change}
                </li>
              ))}
            </ul>
          </div>
        ))}
      </div>

      <div className="mt-8 text-center">
        <a
          href="https://github.com/johanmcad/WorkBenchPro/releases"
          target="_blank"
          rel="noopener noreferrer"
          className="text-wb-accent-light hover:underline text-sm"
        >
          View all releases on GitHub
        </a>
      </div>
    </div>
  )
}
