import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import {
  Loader2,
  Cpu,
  MemoryStick,
  Monitor,
  HardDrive,
  Trash2,
  X,
  PanelLeftClose,
  PanelLeftOpen,
} from 'lucide-react'
import { fetchBenchmarkRuns, fetchBenchmarkRun, fetchTestStatistics, fetchPercentileRank, deleteBenchmarkRun } from '../api'
import CompactComparisonChart from '../components/CompactComparisonChart'

export default function ResultsPage() {
  const { id: urlId } = useParams()

  const [results, setResults] = useState([])
  const [selectedId, setSelectedId] = useState(urlId || null)
  const [selectedRun, setSelectedRun] = useState(null)
  const [statistics, setStatistics] = useState([])
  const [percentileRanks, setPercentileRanks] = useState([])

  const [loadingList, setLoadingList] = useState(true)
  const [loadingDetail, setLoadingDetail] = useState(false)
  const [error, setError] = useState(null)

  // UI state
  const [panelCollapsed, setPanelCollapsed] = useState(false)

  // Delete modal state
  const [showDeleteModal, setShowDeleteModal] = useState(false)
  const [deletePassword, setDeletePassword] = useState('')
  const [deleteError, setDeleteError] = useState(null)
  const [deleting, setDeleting] = useState(false)

  // Load results list
  useEffect(() => {
    const loadResults = async () => {
      setLoadingList(true)
      try {
        const data = await fetchBenchmarkRuns({})
        setResults(data)
        // Auto-select first result if none selected
        if (!selectedId && data.length > 0) {
          setSelectedId(data[0].id)
        }
      } catch (err) {
        setError(err.message)
      } finally {
        setLoadingList(false)
      }
    }
    loadResults()
  }, [])

  // Load selected result details
  useEffect(() => {
    if (!selectedId) return

    const loadDetail = async () => {
      setLoadingDetail(true)
      try {
        const [runData, statsData, ranksData] = await Promise.all([
          fetchBenchmarkRun(selectedId),
          fetchTestStatistics(),
          fetchPercentileRank(selectedId),
        ])
        setSelectedRun(runData)
        setStatistics(statsData)
        setPercentileRanks(ranksData)
      } catch (err) {
        setError(err.message)
      } finally {
        setLoadingDetail(false)
      }
    }
    loadDetail()
  }, [selectedId])

  const handleDelete = async (e) => {
    e.preventDefault()
    setDeleting(true)
    setDeleteError(null)
    try {
      await deleteBenchmarkRun(selectedId, deletePassword)
      // Remove from list and select next
      const newResults = results.filter(r => r.id !== selectedId)
      setResults(newResults)
      setSelectedId(newResults.length > 0 ? newResults[0].id : null)
      setSelectedRun(null)
      setShowDeleteModal(false)
    } catch (err) {
      setDeleteError(err.message)
    } finally {
      setDeleting(false)
    }
  }

  const formatDate = (dateStr) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
    })
  }

  // Build comparison data
  const categories = {
    project_operations: { name: 'Project Operations', color: '#3b82f6', tests: [] },
    build_performance: { name: 'Build Performance', color: '#8b5cf6', tests: [] },
    responsiveness: { name: 'Responsiveness', color: '#10b981', tests: [] },
  }

  if (selectedRun) {
    const userTestsMap = new Map()
    const allUserTests = [
      ...(selectedRun.results.project_operations || []),
      ...(selectedRun.results.build_performance || []),
      ...(selectedRun.results.responsiveness || []),
    ]
    allUserTests.forEach(t => userTestsMap.set(t.test_id, t))

    const percentileMap = new Map()
    percentileRanks.forEach(p => percentileMap.set(p.test_id, p))

    statistics.forEach((stat) => {
      const userTest = userTestsMap.get(stat.test_id)
      const percentile = percentileMap.get(stat.test_id)

      let category = null
      if (selectedRun.results.project_operations?.some(t => t.test_id === stat.test_id)) {
        category = 'project_operations'
      } else if (selectedRun.results.build_performance?.some(t => t.test_id === stat.test_id)) {
        category = 'build_performance'
      } else if (selectedRun.results.responsiveness?.some(t => t.test_id === stat.test_id)) {
        category = 'responsiveness'
      }

      if (category && categories[category]) {
        categories[category].tests.push({
          ...stat,
          userTest,
          percentile,
        })
      }
    })
  }

  const overallStats = calculateOverallStats(percentileRanks)

  return (
    <div className="h-[calc(100vh-120px)] flex">
      {/* Left Panel - Results List */}
      <div className={`${panelCollapsed ? 'w-10' : 'w-72'} border-r border-wb-border flex flex-col bg-wb-bg-card transition-all duration-200`}>
        <div className="p-2 border-b border-wb-border flex items-center justify-between">
          {!panelCollapsed && (
            <div className="px-2">
              <h2 className="font-semibold text-sm">Results</h2>
              <p className="text-[10px] text-wb-text-secondary">
                {results.length} benchmark{results.length !== 1 ? 's' : ''}
              </p>
            </div>
          )}
          <button
            onClick={() => setPanelCollapsed(!panelCollapsed)}
            className="p-1.5 hover:bg-wb-bg-secondary rounded transition-colors text-wb-text-secondary hover:text-white"
            title={panelCollapsed ? 'Expand panel' : 'Collapse panel'}
          >
            {panelCollapsed ? <PanelLeftOpen size={16} /> : <PanelLeftClose size={16} />}
          </button>
        </div>

        {!panelCollapsed && (
          <div className="flex-1 overflow-y-auto">
            {loadingList ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 size={24} className="animate-spin text-wb-accent" />
              </div>
            ) : results.length === 0 ? (
              <div className="p-4 text-center text-wb-text-secondary text-sm">
                No results yet
              </div>
            ) : (
              <div className="divide-y divide-wb-border/50">
                {results.map((result) => (
                  <button
                    key={result.id}
                    onClick={() => setSelectedId(result.id)}
                    className={`w-full text-left p-2 hover:bg-wb-bg-secondary/50 transition-colors ${
                      selectedId === result.id ? 'bg-wb-bg-secondary border-l-2 border-wb-accent' : ''
                    }`}
                  >
                    <div className="flex items-center justify-between gap-2">
                      <span className="font-medium text-xs truncate flex-1">
                        {result.display_name}
                      </span>
                      <span className="text-[10px] text-wb-text-secondary shrink-0">
                        {formatDate(result.uploaded_at)}
                      </span>
                    </div>
                    <div className="flex items-center gap-1 mt-1 text-[10px] text-wb-text-secondary">
                      <span className="truncate">{result.cpu_name}</span>
                      <span>•</span>
                      <span>{Math.round(result.memory_gb)}GB</span>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Right Panel - Comparison View */}
      <div className="flex-1 overflow-y-auto">
        {!selectedId ? (
          <div className="flex items-center justify-center h-full text-wb-text-secondary">
            Select a result to view comparison
          </div>
        ) : loadingDetail ? (
          <div className="flex items-center justify-center h-full">
            <Loader2 size={32} className="animate-spin text-wb-accent" />
          </div>
        ) : !selectedRun ? (
          <div className="flex items-center justify-center h-full text-wb-text-secondary">
            Failed to load result
          </div>
        ) : (
          <div className="p-6">
            {/* Header */}
            <div className="flex items-start justify-between gap-4 mb-6">
              <div>
                <h1 className="text-xl font-bold mb-2">{selectedRun.display_name}</h1>
                <div className="flex flex-wrap gap-2 text-sm">
                  <span className="inline-flex items-center gap-1.5 px-2 py-1 bg-wb-bg-secondary rounded">
                    <Cpu size={14} className="text-wb-accent-light" />
                    {selectedRun.cpu_name}
                  </span>
                  <span className="inline-flex items-center gap-1.5 px-2 py-1 bg-wb-bg-secondary rounded">
                    <MemoryStick size={14} className="text-wb-accent-light" />
                    {selectedRun.memory_gb.toFixed(0)} GB
                  </span>
                  <span className="inline-flex items-center gap-1.5 px-2 py-1 bg-wb-bg-secondary rounded">
                    <Monitor size={14} className="text-wb-accent-light" />
                    {selectedRun.os_name}
                  </span>
                  {selectedRun.storage_type && (
                    <span className="inline-flex items-center gap-1.5 px-2 py-1 bg-wb-bg-secondary rounded">
                      <HardDrive size={14} className="text-wb-accent-light" />
                      {selectedRun.storage_type}
                    </span>
                  )}
                </div>
              </div>

              <button
                onClick={() => {
                  setShowDeleteModal(true)
                  setDeletePassword('')
                  setDeleteError(null)
                }}
                className="btn-secondary flex items-center gap-2 text-sm text-wb-error hover:bg-wb-error/20 hover:border-wb-error shrink-0"
              >
                <Trash2 size={16} />
              </button>
            </div>

            {/* Summary Stats */}
            <div className="flex gap-6 mb-6 pb-6 border-b border-wb-border">
              <div>
                <div className="text-2xl font-bold text-white">
                  {overallStats.totalTests}
                </div>
                <div className="text-xs text-wb-text-secondary">Tests</div>
              </div>
            </div>

            {/* Comparison Charts */}
            <div className="space-y-6">
              {Object.entries(categories).map(([key, cat]) => (
                cat.tests.length > 0 && (
                  <CompactComparisonChart
                    key={key}
                    tests={cat.tests}
                    title={cat.name}
                  />
                )
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Delete Modal */}
      {showDeleteModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="card max-w-md w-full mx-4">
            <div className="flex items-center justify-between mb-6">
              <h3 className="text-xl font-semibold text-wb-warning">Remove Upload</h3>
              <button
                onClick={() => setShowDeleteModal(false)}
                className="text-wb-text-secondary hover:text-white transition-colors"
              >
                <X size={24} />
              </button>
            </div>
            <p className="text-wb-text-secondary mb-6">
              Admin authentication required to remove this benchmark.
            </p>
            <form onSubmit={handleDelete}>
              <div className="mb-4">
                <label className="block text-sm text-wb-text-secondary mb-2">
                  Admin Password
                </label>
                <input
                  type="password"
                  value={deletePassword}
                  onChange={(e) => setDeletePassword(e.target.value)}
                  placeholder="Enter admin password"
                  className="input w-full"
                  autoFocus
                />
              </div>
              {deleteError && (
                <p className="text-wb-error text-sm mb-4">{deleteError}</p>
              )}
              <div className="flex gap-3 justify-end">
                <button
                  type="button"
                  onClick={() => setShowDeleteModal(false)}
                  className="btn-secondary"
                  disabled={deleting}
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="btn-primary bg-wb-error hover:bg-wb-error/80 flex items-center gap-2"
                  disabled={deleting || !deletePassword}
                >
                  {deleting ? (
                    <Loader2 size={18} className="animate-spin" />
                  ) : (
                    <Trash2 size={18} />
                  )}
                  Remove
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}

function calculateOverallStats(percentileRanks) {
  return { totalTests: percentileRanks.length }
}
