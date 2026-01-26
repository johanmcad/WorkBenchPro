import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Search, Filter, Cpu, HardDrive, Monitor, Calendar, ChevronRight, Loader2 } from 'lucide-react'
import { fetchBenchmarkRuns } from '../api'

export default function ResultsPage() {
  const [results, setResults] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)

  // Filters
  const [cpuFilter, setCpuFilter] = useState('')
  const [osFilter, setOsFilter] = useState('')
  const [minMemory, setMinMemory] = useState('')

  const loadResults = async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await fetchBenchmarkRuns({
        cpuFilter: cpuFilter || undefined,
        osFilter: osFilter || undefined,
        minMemory: minMemory ? parseFloat(minMemory) : undefined,
      })
      setResults(data)
    } catch (err) {
      setError(err.message)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadResults()
  }, [])

  const handleFilter = (e) => {
    e.preventDefault()
    loadResults()
  }

  const formatDate = (dateStr) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    })
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Community Results</h1>
        <p className="text-wb-text-secondary">
          Browse benchmark results uploaded by the community
        </p>
      </div>

      {/* Filters */}
      <form onSubmit={handleFilter} className="card mb-8">
        <div className="flex flex-wrap gap-4 items-end">
          <div className="flex-1 min-w-[200px]">
            <label className="block text-sm text-wb-text-secondary mb-2">CPU</label>
            <div className="relative">
              <Cpu size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-wb-text-secondary" />
              <input
                type="text"
                value={cpuFilter}
                onChange={(e) => setCpuFilter(e.target.value)}
                placeholder="e.g. Ryzen, i7"
                className="input w-full pl-10"
              />
            </div>
          </div>

          <div className="flex-1 min-w-[200px]">
            <label className="block text-sm text-wb-text-secondary mb-2">Operating System</label>
            <div className="relative">
              <Monitor size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-wb-text-secondary" />
              <input
                type="text"
                value={osFilter}
                onChange={(e) => setOsFilter(e.target.value)}
                placeholder="e.g. Windows"
                className="input w-full pl-10"
              />
            </div>
          </div>

          <div className="w-32">
            <label className="block text-sm text-wb-text-secondary mb-2">Min RAM (GB)</label>
            <input
              type="number"
              value={minMemory}
              onChange={(e) => setMinMemory(e.target.value)}
              placeholder="16"
              className="input w-full"
            />
          </div>

          <button type="submit" className="btn-primary flex items-center gap-2">
            <Filter size={18} />
            Apply
          </button>
        </div>
      </form>

      {/* Results */}
      {loading ? (
        <div className="flex items-center justify-center py-20">
          <Loader2 size={32} className="animate-spin text-wb-accent" />
        </div>
      ) : error ? (
        <div className="card border-wb-error/50 text-center py-12">
          <p className="text-wb-error mb-4">Failed to load results: {error}</p>
          <button onClick={loadResults} className="btn-secondary">
            Try Again
          </button>
        </div>
      ) : results.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-wb-text-secondary text-lg mb-2">No results found</p>
          <p className="text-wb-text-secondary text-sm">
            Be the first to upload your benchmark! Download the tool and share your results.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          <p className="text-wb-text-secondary text-sm mb-4">
            {results.length} result{results.length !== 1 ? 's' : ''} found
          </p>

          {results.map((result) => (
            <Link
              key={result.id}
              to={`/results/${result.id}`}
              className="card block hover:border-wb-accent/50 transition-colors group"
            >
              <div className="flex flex-col md:flex-row md:items-center gap-4">
                {/* Main Info */}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="font-semibold text-lg truncate">
                      {result.display_name}
                    </h3>
                    <span className="text-wb-text-secondary text-sm flex items-center gap-1">
                      <Calendar size={14} />
                      {formatDate(result.uploaded_at)}
                    </span>
                  </div>

                  {/* Specs */}
                  <div className="flex flex-wrap gap-3">
                    <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-wb-bg-secondary rounded-full text-sm">
                      <Cpu size={14} className="text-wb-accent-light" />
                      {result.cpu_name}
                    </span>
                    <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-wb-bg-secondary rounded-full text-sm">
                      {result.cpu_cores}C/{result.cpu_threads}T
                    </span>
                    <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-wb-bg-secondary rounded-full text-sm">
                      {Math.round(result.memory_gb)} GB
                    </span>
                    <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-wb-bg-secondary rounded-full text-sm">
                      <Monitor size={14} className="text-wb-accent-light" />
                      {result.os_name}
                    </span>
                    {result.storage_type && (
                      <span className="inline-flex items-center gap-1.5 px-3 py-1 bg-wb-bg-secondary rounded-full text-sm">
                        <HardDrive size={14} className="text-wb-accent-light" />
                        {result.storage_type}
                      </span>
                    )}
                  </div>
                </div>

                {/* Arrow */}
                <ChevronRight
                  size={24}
                  className="text-wb-text-secondary group-hover:text-wb-accent-light transition-colors flex-shrink-0"
                />
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
