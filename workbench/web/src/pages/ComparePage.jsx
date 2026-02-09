import { useState, useEffect } from 'react'
import { useSearchParams, Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { ArrowLeft, Loader2, GitCompare, Search, TrendingUp, TrendingDown, Minus } from 'lucide-react'
import { fetchBenchmarkRuns, fetchBenchmarkRun } from '../api'

export default function ComparePage() {
  const [searchParams] = useSearchParams()
  const preselectedId = searchParams.get('id')

  const [runA, setRunA] = useState(null)
  const [runB, setRunB] = useState(null)
  const [availableRuns, setAvailableRuns] = useState([])
  const [loading, setLoading] = useState(true)
  const [loadingRun, setLoadingRun] = useState(false)

  useEffect(() => {
    const load = async () => {
      setLoading(true)
      try {
        const runs = await fetchBenchmarkRuns({ limit: 100 })
        setAvailableRuns(runs)

        // If preselected, load that run as run A
        if (preselectedId) {
          const run = await fetchBenchmarkRun(preselectedId)
          if (run) setRunA(run)
        }
      } catch (err) {
        console.error(err)
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [preselectedId])

  const selectRun = async (id, slot) => {
    setLoadingRun(true)
    try {
      const run = await fetchBenchmarkRun(id)
      if (slot === 'A') {
        setRunA(run)
      } else {
        setRunB(run)
      }
    } catch (err) {
      console.error(err)
    } finally {
      setLoadingRun(false)
    }
  }

  const isHigherBetter = (unit) => {
    const u = unit.toLowerCase()
    return u.includes('/s') || u.includes('ops') || u.includes('files') || u.includes('iops')
  }

  const calculateDiff = (a, b, unit) => {
    if (!a || !b || a === 0) return { pct: 0, better: null }
    const pct = ((b - a) / a) * 100
    const higherIsBetter = isHigherBetter(unit)
    let better = null
    if (Math.abs(pct) >= 1) {
      better = higherIsBetter ? (pct > 0 ? 'B' : 'A') : (pct < 0 ? 'B' : 'A')
    }
    return { pct, better }
  }

  const DiffIndicator = ({ diff }) => {
    if (diff.better === null) {
      return <Minus size={16} className="text-wb-text-secondary" />
    }
    if (diff.better === 'B') {
      return (
        <span className="flex items-center gap-1 text-wb-success">
          <TrendingUp size={16} />
          {Math.abs(diff.pct).toFixed(1)}%
        </span>
      )
    }
    return (
      <span className="flex items-center gap-1 text-wb-error">
        <TrendingDown size={16} />
        {Math.abs(diff.pct).toFixed(1)}%
      </span>
    )
  }

  const helmetTitle = 'Compare Benchmark Results Side-by-Side | WorkBench-Pro'
  const helmetDescription = 'Compare workstation benchmark results side-by-side. See CPU, disk, memory, and responsiveness differences between different hardware configurations.'

  if (loading) {
    return (
      <div className="flex items-center justify-center py-32">
        <Loader2 size={32} className="animate-spin text-wb-accent" />
      </div>
    )
  }

  const categories = [
    { key: 'project_operations', name: 'Project Operations' },
    { key: 'build_performance', name: 'Build Performance' },
    { key: 'responsiveness', name: 'Responsiveness' },
  ]

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <Helmet>
        <title>{helmetTitle}</title>
        <meta name="description" content={helmetDescription} />
        <link rel="canonical" href="https://www.workbench-pro.com/compare" />
        <meta property="og:url" content="https://www.workbench-pro.com/compare" />
        <meta property="og:title" content={helmetTitle} />
        <meta property="og:description" content={helmetDescription} />
        <meta property="twitter:url" content="https://www.workbench-pro.com/compare" />
        <meta property="twitter:title" content={helmetTitle} />
        <meta property="twitter:description" content={helmetDescription} />
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "Compare" }
          ]
        })}</script>
      </Helmet>

      {/* Back link */}
      <Link
        to="/results"
        className="inline-flex items-center gap-2 text-wb-text-secondary hover:text-white mb-6 transition-colors"
      >
        <ArrowLeft size={20} />
        Back to Results
      </Link>

      <h1 className="text-3xl font-bold mb-8 flex items-center gap-3">
        <GitCompare className="text-wb-accent-light" />
        Compare Results
      </h1>

      {/* Selection */}
      <div className="grid md:grid-cols-2 gap-6 mb-8">
        {/* Run A Selection */}
        <div className="card">
          <h3 className="font-semibold mb-4 text-wb-accent-light">Run A (Baseline)</h3>
          {runA ? (
            <div className="mb-4">
              <div className="font-medium">{runA.display_name}</div>
              <div className="text-sm text-wb-text-secondary">{runA.system_info.cpu.name}</div>
              <button
                onClick={() => setRunA(null)}
                className="text-sm text-wb-accent-light hover:underline mt-2"
              >
                Change
              </button>
            </div>
          ) : (
            <select
              className="input w-full"
              value=""
              onChange={(e) => selectRun(e.target.value, 'A')}
              disabled={loadingRun}
            >
              <option value="">Select a run...</option>
              {availableRuns.map((r) => (
                <option key={r.id} value={r.id}>
                  {r.display_name} - {r.cpu_name}
                </option>
              ))}
            </select>
          )}
        </div>

        {/* Run B Selection */}
        <div className="card">
          <h3 className="font-semibold mb-4 text-orange-400">Run B (Compare)</h3>
          {runB ? (
            <div className="mb-4">
              <div className="font-medium">{runB.display_name}</div>
              <div className="text-sm text-wb-text-secondary">{runB.system_info.cpu.name}</div>
              <button
                onClick={() => setRunB(null)}
                className="text-sm text-wb-accent-light hover:underline mt-2"
              >
                Change
              </button>
            </div>
          ) : (
            <select
              className="input w-full"
              value=""
              onChange={(e) => selectRun(e.target.value, 'B')}
              disabled={loadingRun || !runA}
            >
              <option value="">Select a run to compare...</option>
              {availableRuns
                .filter((r) => r.id !== runA?.id)
                .map((r) => (
                  <option key={r.id} value={r.id}>
                    {r.display_name} - {r.cpu_name}
                  </option>
                ))}
            </select>
          )}
        </div>
      </div>

      {loadingRun && (
        <div className="flex items-center justify-center py-8">
          <Loader2 size={24} className="animate-spin text-wb-accent" />
        </div>
      )}

      {/* Comparison Results */}
      {runA && runB && !loadingRun && (
        <div className="space-y-6">
          {/* System Comparison */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-4">System Comparison</h2>
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-wb-border">
                    <th className="text-left py-3 px-4 text-wb-text-secondary">Spec</th>
                    <th className="text-left py-3 px-4 text-wb-accent-light">Run A</th>
                    <th className="text-left py-3 px-4 text-orange-400">Run B</th>
                  </tr>
                </thead>
                <tbody>
                  <tr className="border-b border-wb-border/50">
                    <td className="py-3 px-4 text-wb-text-secondary">CPU</td>
                    <td className="py-3 px-4">{runA.system_info.cpu.name}</td>
                    <td className="py-3 px-4">{runB.system_info.cpu.name}</td>
                  </tr>
                  <tr className="border-b border-wb-border/50">
                    <td className="py-3 px-4 text-wb-text-secondary">Cores/Threads</td>
                    <td className="py-3 px-4">{runA.system_info.cpu.cores}C/{runA.system_info.cpu.threads}T</td>
                    <td className="py-3 px-4">{runB.system_info.cpu.cores}C/{runB.system_info.cpu.threads}T</td>
                  </tr>
                  <tr className="border-b border-wb-border/50">
                    <td className="py-3 px-4 text-wb-text-secondary">Memory</td>
                    <td className="py-3 px-4">{(runA.system_info.memory.total_bytes / 1073741824).toFixed(0)} GB</td>
                    <td className="py-3 px-4">{(runB.system_info.memory.total_bytes / 1073741824).toFixed(0)} GB</td>
                  </tr>
                  <tr className="border-b border-wb-border/50">
                    <td className="py-3 px-4 text-wb-text-secondary">OS</td>
                    <td className="py-3 px-4">{runA.system_info.os.name}</td>
                    <td className="py-3 px-4">{runB.system_info.os.name}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          {/* Benchmark Comparisons */}
          {categories.map(({ key, name }) => {
            const resultsA = runA.results[key] || []
            const resultsB = runB.results[key] || []
            if (resultsA.length === 0 && resultsB.length === 0) return null

            // Match by test_id
            const allTestIds = [...new Set([
              ...resultsA.map((r) => r.test_id),
              ...resultsB.map((r) => r.test_id),
            ])]

            return (
              <div key={key} className="card">
                <h2 className="text-xl font-semibold mb-4">{name}</h2>
                <div className="overflow-x-auto">
                  <table className="w-full text-sm">
                    <thead>
                      <tr className="border-b border-wb-border">
                        <th className="text-left py-3 px-4 text-wb-text-secondary">Test</th>
                        <th className="text-right py-3 px-4 text-wb-accent-light">Run A</th>
                        <th className="text-right py-3 px-4 text-orange-400">Run B</th>
                        <th className="text-right py-3 px-4 text-wb-text-secondary">Diff</th>
                      </tr>
                    </thead>
                    <tbody>
                      {allTestIds.map((testId) => {
                        const a = resultsA.find((r) => r.test_id === testId)
                        const b = resultsB.find((r) => r.test_id === testId)
                        const diff = calculateDiff(a?.value, b?.value, a?.unit || b?.unit || '')

                        return (
                          <tr key={testId} className="border-b border-wb-border/50 hover:bg-wb-bg-secondary/30">
                            <td className="py-3 px-4">
                              {a?.name || b?.name}
                            </td>
                            <td className="text-right py-3 px-4 font-mono">
                              {a ? `${a.value.toFixed(2)} ${a.unit}` : '-'}
                            </td>
                            <td className="text-right py-3 px-4 font-mono">
                              {b ? `${b.value.toFixed(2)} ${b.unit}` : '-'}
                            </td>
                            <td className="text-right py-3 px-4">
                              {a && b ? <DiffIndicator diff={diff} /> : '-'}
                            </td>
                          </tr>
                        )
                      })}
                    </tbody>
                  </table>
                </div>
              </div>
            )
          })}
        </div>
      )}

      {/* Empty state */}
      {(!runA || !runB) && !loadingRun && (
        <div className="card text-center py-12">
          <GitCompare size={48} className="mx-auto text-wb-text-secondary mb-4" />
          <p className="text-wb-text-secondary">
            Pick two runs above to see how they stack up.
          </p>
        </div>
      )}
    </div>
  )
}
