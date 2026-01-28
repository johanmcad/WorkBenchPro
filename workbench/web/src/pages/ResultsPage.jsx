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
  Search,
  Trophy,
  ArrowUpDown,
  Pencil,
} from 'lucide-react'
import { fetchBenchmarkRuns, fetchBenchmarkRun, fetchTestStatistics, fetchPercentileRank, deleteBenchmarkRun, updateBenchmarkRun } from '../api'
import CompactComparisonChart from '../components/CompactComparisonChart'

// Colors for multi-select (up to 3)
const SELECTION_COLORS = [
  { name: 'Green', bg: 'bg-green-400', text: 'text-green-400', hex: '#4ade80' },
  { name: 'Blue', bg: 'bg-blue-400', text: 'text-blue-400', hex: '#60a5fa' },
  { name: 'Orange', bg: 'bg-orange-400', text: 'text-orange-400', hex: '#fb923c' },
]

export default function ResultsPage() {
  const { id: urlId } = useParams()

  const [results, setResults] = useState([])
  const [selectedIds, setSelectedIds] = useState(urlId ? [urlId] : []) // Up to 3 selections
  const [selectedRuns, setSelectedRuns] = useState({}) // { id: runData }
  const [statistics, setStatistics] = useState([])
  const [allPercentileRanks, setAllPercentileRanks] = useState({}) // { id: ranksData }

  const [loadingList, setLoadingList] = useState(true)
  const [loadingDetail, setLoadingDetail] = useState(false)
  const [error, setError] = useState(null)

  // UI state
  const [panelCollapsed, setPanelCollapsed] = useState(false)
  const [filterText, setFilterText] = useState('')
  const [sortMode, setSortMode] = useState('score') // 'score', 'name', 'date'
  const [groupByDevice, setGroupByDevice] = useState(false)
  const [runScores, setRunScores] = useState({}) // { runId: { betterThanMedian, totalTests } }

  // Detect device type based on Intel/AMD CPU naming conventions
  // Sources: Intel & AMD official naming schemes
  const getDeviceType = (cpuName) => {
    if (!cpuName) return 'unknown'
    const cpu = cpuName.toUpperCase()

    // Server/VDI patterns (check first - most specific)
    // Only true server CPUs: AMD EPYC, Intel Xeon Scalable (Gold/Platinum/Silver/Bronze)
    // Virtual machine indicators
    const serverPatterns = [
      /EPYC/i,                   // AMD EPYC - always server
      /PLATINUM/i,               // Intel Xeon Platinum - datacenter
      /\bGOLD\b.*\d{4}/i,        // Intel Xeon Gold 6xxx - datacenter
      /\bSILVER\b.*\d{4}/i,      // Intel Xeon Silver - datacenter
      /\bBRONZE\b.*\d{4}/i,      // Intel Xeon Bronze - datacenter
      /VIRTUAL/i,                // Virtual CPU indicator
      /QEMU/i,                   // QEMU virtual CPU
      /KVM/i,                    // KVM virtual CPU
      /HYPERVISOR/i,             // Hypervisor indicator
      /\bE7-/i,                  // Intel Xeon E7 - typically multi-socket server
    ]

    // Desktop workstation patterns (Xeons used in desktops)
    // Intel Xeon E5 (v1-v4), Xeon W, Xeon E are commonly desktop workstation
    const workstationPatterns = [
      /XEON.*E5-/i,              // Intel Xeon E5 - desktop workstation (like E5-1630)
      /XEON.*E3-/i,              // Intel Xeon E3 - desktop workstation
      /XEON.*W-/i,               // Intel Xeon W - workstation
      /XEON.*E-\d{4}/i,          // Intel Xeon E (E-2288G) - workstation
      /THREADRIPPER/i,           // AMD Threadripper (including PRO) - HEDT/workstation
    ]

    // Laptop/Mobile suffixes:
    // Intel: U (ultra-low power), H/HK (high-perf mobile), P (balanced mobile)
    // AMD: U (ultrathin), H/HS/HX (high-perf mobile), C (chromebook), E (fanless)
    const laptopPatterns = [
      /\d{4,5}[UH]\b/,           // Intel/AMD: ends in U or H (i7-1260U, Ryzen 7 6800H)
      /\d{4,5}H[KSX]\b/,         // Intel HK, AMD HS/HX (i9-12900HK, Ryzen 9 6900HX)
      /\d{4,5}P\b/,              // Intel P-series (i7-1280P)
      /\d{3,4}[UH]\b/,           // Older 4-digit models (i7-8550U)
      /MOBILE/i,                  // Explicit mobile keyword
      /\bM[123]\b/,              // Apple M1, M2, M3
      /\bM[123] (PRO|MAX|ULTRA)/i, // Apple M1/M2/M3 Pro/Max/Ultra
    ]

    // Desktop suffixes:
    // Intel: K/KF (unlocked), F (no iGPU), T (low power), X/XE (extreme)
    // AMD: X/XT (high perf), X3D (3D V-Cache), G/GE (APU), F (no iGPU)
    const desktopPatterns = [
      /\d{4,5}K\b/,              // Intel K (i9-13900K)
      /\d{4,5}KF\b/,             // Intel KF (i9-13900KF)
      /\d{4,5}F\b/,              // Intel/AMD F - no iGPU (i5-13400F)
      /\d{4,5}T\b/,              // Intel T - low power desktop
      /\d{4,5}X\b/,              // AMD X (Ryzen 9 5900X)
      /\d{4,5}XT\b/,             // AMD XT (Ryzen 7 3800XT)
      /\d{4,5}X3D\b/,            // AMD X3D (Ryzen 9 7950X3D)
      /\d{4,5}G\b/,              // AMD G - APU (Ryzen 5 5600G)
      /\d{4,5}GE\b/,             // AMD GE - efficient APU
      /\d{3,4}K\b/,              // Older Intel K (i7-8700K)
      /THREADRIPPER\b/i,         // AMD Threadripper (HEDT, not PRO)
    ]

    // Check server patterns first (most specific)
    for (const pattern of serverPatterns) {
      if (pattern.test(cpu)) return 'server'
    }

    // Check workstation patterns (desktop workstation Xeons, Threadripper)
    for (const pattern of workstationPatterns) {
      if (pattern.test(cpu)) return 'desktop'
    }

    // Check laptop patterns
    for (const pattern of laptopPatterns) {
      if (pattern.test(cpu)) return 'laptop'
    }

    // Check desktop patterns
    for (const pattern of desktopPatterns) {
      if (pattern.test(cpu)) return 'desktop'
    }

    // Default heuristics for unmarked CPUs
    // CPUs without suffix are usually desktop (e.g., i5-13400, Ryzen 5 5600)
    return 'desktop'
  }

  // Delete modal state
  const [showDeleteModal, setShowDeleteModal] = useState(false)
  const [deletePassword, setDeletePassword] = useState('')
  const [deleteError, setDeleteError] = useState(null)
  const [deleting, setDeleting] = useState(false)

  // Edit modal state
  const [showEditModal, setShowEditModal] = useState(false)
  const [editDisplayName, setEditDisplayName] = useState('')
  const [editUserName, setEditUserName] = useState('')
  const [editDescription, setEditDescription] = useState('')
  const [editPassword, setEditPassword] = useState('')
  const [editError, setEditError] = useState(null)
  const [editing, setEditing] = useState(false)

  // Load results list and statistics on mount
  useEffect(() => {
    const loadInitialData = async () => {
      setLoadingList(true)
      try {
        const [data, statsData] = await Promise.all([
          fetchBenchmarkRuns({}),
          fetchTestStatistics(),
        ])
        setResults(data)
        setStatistics(statsData)
      } catch (err) {
        setError(err.message)
      } finally {
        setLoadingList(false)
      }
    }
    loadInitialData()
  }, [])

  // Load selected result details for all selected IDs
  useEffect(() => {
    if (selectedIds.length === 0) return

    const loadDetails = async () => {
      setLoadingDetail(true)
      try {
        // Load details for each selected ID
        const newRuns = {}
        const newRanks = {}

        for (const id of selectedIds) {
          // Skip if already loaded
          if (selectedRuns[id] && allPercentileRanks[id]) continue

          const [runData, ranksData] = await Promise.all([
            fetchBenchmarkRun(id),
            fetchPercentileRank(id),
          ])
          newRuns[id] = runData
          newRanks[id] = ranksData

          // Calculate and store score
          if (ranksData && ranksData.length > 0) {
            const betterThanMedian = ranksData.filter(r => r.percentile_rank > 50).length
            setRunScores(prev => ({
              ...prev,
              [id]: { betterThanMedian, totalTests: ranksData.length }
            }))
          }
        }

        // Merge with existing data, remove deselected
        setSelectedRuns(prev => {
          const updated = { ...prev, ...newRuns }
          // Remove runs that are no longer selected
          Object.keys(updated).forEach(id => {
            if (!selectedIds.includes(id)) delete updated[id]
          })
          return updated
        })
        setAllPercentileRanks(prev => {
          const updated = { ...prev, ...newRanks }
          Object.keys(updated).forEach(id => {
            if (!selectedIds.includes(id)) delete updated[id]
          })
          return updated
        })
      } catch (err) {
        setError(err.message)
      } finally {
        setLoadingDetail(false)
      }
    }
    loadDetails()
  }, [selectedIds])

  // Load scores for all runs in background
  useEffect(() => {
    if (results.length === 0) return

    const loadAllScores = async () => {
      for (const result of results) {
        if (runScores[result.id]) continue // Skip if already loaded
        try {
          const ranksData = await fetchPercentileRank(result.id)
          if (ranksData && ranksData.length > 0) {
            const betterThanMedian = ranksData.filter(r => r.percentile_rank > 50).length
            setRunScores(prev => ({
              ...prev,
              [result.id]: { betterThanMedian, totalTests: ranksData.length }
            }))
          }
        } catch (err) {
          // Silently ignore errors for background loading
        }
      }
    }
    loadAllScores()
  }, [results])

  // Toggle selection of a run (add/remove from selected, max 3)
  const toggleSelection = (id) => {
    setSelectedIds(prev => {
      if (prev.includes(id)) {
        // Remove from selection
        return prev.filter(i => i !== id)
      } else if (prev.length < 3) {
        // Add to selection (max 3)
        return [...prev, id]
      }
      // Already at max, replace the last one
      return [...prev.slice(0, 2), id]
    })
  }

  // Get color for a selected ID (based on position in selectedIds array)
  const getSelectionColor = (id) => {
    const index = selectedIds.indexOf(id)
    return index >= 0 ? SELECTION_COLORS[index] : null
  }

  // For modals/actions, use the first selected ID as "primary"
  const primarySelectedId = selectedIds[0] || null
  const primarySelectedRun = selectedRuns[primarySelectedId] || null

  const handleDelete = async (e) => {
    e.preventDefault()
    setDeleting(true)
    setDeleteError(null)
    try {
      await deleteBenchmarkRun(primarySelectedId, deletePassword)
      // Remove from list and selections
      const newResults = results.filter(r => r.id !== primarySelectedId)
      setResults(newResults)
      setSelectedIds(prev => prev.filter(id => id !== primarySelectedId))
      setShowDeleteModal(false)
    } catch (err) {
      setDeleteError(err.message)
    } finally {
      setDeleting(false)
    }
  }

  const handleEdit = async (e) => {
    e.preventDefault()
    setEditing(true)
    setEditError(null)
    try {
      await updateBenchmarkRun(primarySelectedId, editPassword, {
        display_name: editDisplayName,
        user_name: editUserName,
        description: editDescription,
      })
      // Update local state
      setResults(results.map(r =>
        r.id === primarySelectedId
          ? { ...r, display_name: editDisplayName, user_name: editUserName, description: editDescription }
          : r
      ))
      setSelectedRuns(prev => prev[primarySelectedId] ? {
        ...prev,
        [primarySelectedId]: {
          ...prev[primarySelectedId],
          display_name: editDisplayName,
          user_name: editUserName,
          description: editDescription,
        }
      } : prev)
      setShowEditModal(false)
    } catch (err) {
      setEditError(err.message)
    } finally {
      setEditing(false)
    }
  }

  const formatDate = (dateStr) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
    })
  }

  // Build comparison data for all selected runs
  const categories = {
    project_operations: { name: 'Project Operations', color: '#3b82f6', tests: [] },
    build_performance: { name: 'Build Performance', color: '#8b5cf6', tests: [] },
    responsiveness: { name: 'Responsiveness', color: '#10b981', tests: [] },
  }

  // Category mapping based on test_id patterns
  const getCategoryForTest = (testId) => {
    const projectOps = ['file_enum', 'random_read', 'metadata_ops', 'dir_traversal', 'large_file', 'registry_ops', 'windows_services', 'network_tools']
    const buildPerf = ['single_thread', 'multi_thread', 'mixed_workload', 'cargo_build', 'csharp_compile', 'archive_ops', 'powershell', 'memory_bandwidth', 'memory_latency']
    const responsive = ['process_spawn', 'thread_wake', 'storage_latency']

    if (projectOps.some(p => testId.includes(p))) return 'project_operations'
    if (buildPerf.some(p => testId.includes(p))) return 'build_performance'
    if (responsive.some(p => testId.includes(p))) return 'responsiveness'
    return 'project_operations' // default
  }

  // Build selections array with data for passing to chart
  const selections = selectedIds.map((id, index) => ({
    id,
    displayName: selectedRuns[id]?.display_name || 'Loading...',
    color: SELECTION_COLORS[index],
    run: selectedRuns[id],
    percentileRanks: allPercentileRanks[id] || [],
  })).filter(s => s.run) // Only include loaded runs

  // Build categories from statistics (works with or without selections)
  statistics.forEach((stat) => {
    const category = getCategoryForTest(stat.test_id)

    if (category && categories[category]) {
      // Build selections data for this specific test
      const testSelections = selections.map(sel => {
        const allTests = [
          ...(sel.run.results.project_operations || []),
          ...(sel.run.results.build_performance || []),
          ...(sel.run.results.responsiveness || []),
        ]
        const userTest = allTests.find(t => t.test_id === stat.test_id)
        const percentile = sel.percentileRanks.find(p => p.test_id === stat.test_id)
        return {
          id: sel.id,
          displayName: sel.displayName,
          color: sel.color,
          userTest,
          percentile,
        }
      })

      categories[category].tests.push({
        ...stat,
        selections: testSelections,
      })
    }
  })

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
          <div className="flex-1 overflow-y-auto flex flex-col">
            {/* Filter input */}
            <div className="p-2 border-b border-wb-border space-y-2">
              <div className="relative">
                <Search size={14} className="absolute left-2 top-1/2 -translate-y-1/2 text-wb-text-secondary" />
                <input
                  type="text"
                  value={filterText}
                  onChange={(e) => setFilterText(e.target.value)}
                  placeholder="Filter..."
                  className="w-full pl-7 pr-2 py-1 text-xs bg-wb-bg-secondary border border-wb-border rounded focus:outline-none focus:border-wb-accent"
                />
              </div>
              <div className="flex items-center gap-2">
                <select
                  value={sortMode}
                  onChange={(e) => setSortMode(e.target.value)}
                  className="text-[10px] px-2 py-1 bg-wb-bg-secondary border border-wb-border rounded focus:outline-none focus:border-wb-accent text-wb-text-primary"
                >
                  <option value="score">Sort: Score</option>
                  <option value="name">Sort: Name</option>
                  <option value="date">Sort: Date</option>
                </select>
                <button
                  onClick={() => setGroupByDevice(!groupByDevice)}
                  className={`text-[10px] px-2 py-1 rounded transition-colors whitespace-nowrap ${
                    groupByDevice ? 'bg-wb-accent/20 text-wb-accent' : 'text-wb-text-secondary hover:text-white border border-wb-border'
                  }`}
                  title="Group by device type (Desktop/Laptop/Server)"
                >
                  Group by type
                </button>
              </div>
            </div>
            {loadingList ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 size={24} className="animate-spin text-wb-accent" />
              </div>
            ) : results.length === 0 ? (
              <div className="p-4 text-center text-wb-text-secondary text-sm">
                No results yet
              </div>
            ) : (
              <div className="divide-y divide-wb-border/50 flex-1 overflow-y-auto">
                {(() => {
                  // Filter and sort results
                  const filtered = results.filter(r =>
                    !filterText ||
                    r.display_name?.toLowerCase().includes(filterText.toLowerCase()) ||
                    r.cpu_name?.toLowerCase().includes(filterText.toLowerCase())
                  ).sort((a, b) => {
                    if (sortMode === 'score') {
                      const scoreA = runScores[a.id]?.betterThanMedian ?? -1
                      const scoreB = runScores[b.id]?.betterThanMedian ?? -1
                      return scoreB - scoreA
                    } else if (sortMode === 'name') {
                      return (a.display_name || '').localeCompare(b.display_name || '')
                    } else if (sortMode === 'date') {
                      return new Date(b.uploaded_at) - new Date(a.uploaded_at)
                    }
                    return 0
                  })

                  // Group by device type if enabled
                  if (groupByDevice) {
                    const groups = { desktop: [], laptop: [], server: [] }
                    filtered.forEach(r => {
                      const type = getDeviceType(r.cpu_name)
                      if (type === 'server') groups.server.push(r)
                      else if (type === 'laptop') groups.laptop.push(r)
                      else groups.desktop.push(r)
                    })

                    const renderResult = (result) => {
                      const selectionColor = getSelectionColor(result.id)
                      const isSelected = selectedIds.includes(result.id)
                      return (
                        <button
                          key={result.id}
                          onClick={() => toggleSelection(result.id)}
                          className={`w-full text-left p-2 hover:bg-wb-bg-secondary/50 transition-colors ${
                            isSelected ? 'bg-wb-bg-secondary' : ''
                          }`}
                          style={isSelected ? { borderLeft: `3px solid ${selectionColor?.hex}` } : {}}
                        >
                          <div className="flex items-center justify-between gap-2">
                            <div className="flex items-center gap-2 min-w-0 flex-1">
                              {isSelected && (
                                <div
                                  className="w-2 h-2 rounded-full shrink-0"
                                  style={{ backgroundColor: selectionColor?.hex }}
                                />
                              )}
                              <span className="font-medium text-xs truncate">
                                {result.display_name}
                              </span>
                            </div>
                            {runScores[result.id] ? (
                              <span className="text-[10px] text-green-400 shrink-0 font-medium">
                                {runScores[result.id].betterThanMedian}/{runScores[result.id].totalTests}
                              </span>
                            ) : (
                              <span className="text-[10px] text-wb-text-secondary shrink-0">
                                {formatDate(result.uploaded_at)}
                              </span>
                            )}
                          </div>
                          <div className="flex items-center gap-1 mt-1 text-[10px] text-wb-text-secondary">
                            <span className="truncate">{result.cpu_name}</span>
                            <span>•</span>
                            <span>{Math.round(result.memory_gb)}GB</span>
                          </div>
                        </button>
                      )
                    }

                    return (
                      <>
                        {groups.desktop.length > 0 && (
                          <>
                            <div className="px-3 py-1.5 bg-wb-bg-secondary/50 text-[10px] font-medium text-wb-text-secondary uppercase tracking-wide">
                              Desktop ({groups.desktop.length})
                            </div>
                            {groups.desktop.map(renderResult)}
                          </>
                        )}
                        {groups.laptop.length > 0 && (
                          <>
                            <div className="px-3 py-1.5 bg-wb-bg-secondary/50 text-[10px] font-medium text-wb-text-secondary uppercase tracking-wide">
                              Laptop ({groups.laptop.length})
                            </div>
                            {groups.laptop.map(renderResult)}
                          </>
                        )}
                        {groups.server.length > 0 && (
                          <>
                            <div className="px-3 py-1.5 bg-wb-bg-secondary/50 text-[10px] font-medium text-wb-text-secondary uppercase tracking-wide">
                              Server / VDI ({groups.server.length})
                            </div>
                            {groups.server.map(renderResult)}
                          </>
                        )}
                      </>
                    )
                  }

                  // No grouping - render flat list
                  return filtered.map((result) => {
                    const selectionColor = getSelectionColor(result.id)
                    const isSelected = selectedIds.includes(result.id)
                    return (
                      <button
                        key={result.id}
                        onClick={() => toggleSelection(result.id)}
                        className={`w-full text-left p-2 hover:bg-wb-bg-secondary/50 transition-colors ${
                          isSelected ? 'bg-wb-bg-secondary' : ''
                        }`}
                        style={isSelected ? { borderLeft: `3px solid ${selectionColor?.hex}` } : {}}
                      >
                        <div className="flex items-center justify-between gap-2">
                          <div className="flex items-center gap-2 min-w-0 flex-1">
                            {isSelected && (
                              <div
                                className="w-2 h-2 rounded-full shrink-0"
                                style={{ backgroundColor: selectionColor?.hex }}
                              />
                            )}
                            <span className="font-medium text-xs truncate">
                              {result.display_name}
                            </span>
                          </div>
                          {runScores[result.id] ? (
                            <span className="text-[10px] text-green-400 shrink-0 font-medium">
                              {runScores[result.id].betterThanMedian}/{runScores[result.id].totalTests}
                            </span>
                          ) : (
                            <span className="text-[10px] text-wb-text-secondary shrink-0">
                              {formatDate(result.uploaded_at)}
                            </span>
                          )}
                        </div>
                        <div className="flex items-center gap-1 mt-1 text-[10px] text-wb-text-secondary">
                          <span className="truncate">{result.cpu_name}</span>
                          <span>•</span>
                          <span>{Math.round(result.memory_gb)}GB</span>
                        </div>
                      </button>
                    )
                  })
                })()}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Right Panel - Comparison View */}
      <div className="flex-1 overflow-y-auto">
        {loadingList ? (
          <div className="flex items-center justify-center h-full">
            <Loader2 size={32} className="animate-spin text-wb-accent" />
          </div>
        ) : loadingDetail ? (
          <div className="flex items-center justify-center h-full">
            <Loader2 size={32} className="animate-spin text-wb-accent" />
          </div>
        ) : statistics.length === 0 ? (
          <div className="flex items-center justify-center h-full text-wb-text-secondary">
            No statistics available
          </div>
        ) : (
          <div className="p-6">
            {/* Header - show all selected computers or community stats message */}
            {selections.length === 0 ? (
              <div className="mb-6 p-4 rounded-lg bg-wb-bg-card border border-wb-border">
                <p className="text-wb-text-secondary text-sm">
                  Showing community statistics. Select a result from the left panel to compare your benchmark.
                </p>
              </div>
            ) : (
            <div className="mb-6 space-y-3">
              {selections.map((sel, index) => (
                <div
                  key={sel.id}
                  className="flex items-start justify-between gap-4 p-3 rounded-lg"
                  style={{ backgroundColor: `${sel.color.hex}10`, borderLeft: `3px solid ${sel.color.hex}` }}
                >
                  <div>
                    <div className="flex items-center gap-3 mb-2">
                      <div
                        className="w-3 h-3 rounded-full shrink-0"
                        style={{ backgroundColor: sel.color.hex }}
                      />
                      <h2 className="text-lg font-bold">{sel.displayName}</h2>
                      {sel.run.user_name && (
                        <span className="text-wb-text-secondary text-sm">by {sel.run.user_name}</span>
                      )}
                      {sel.run.description && (
                        <span className="text-wb-text-secondary text-sm">({sel.run.description})</span>
                      )}
                      {runScores[sel.id] && (
                        <span className="inline-flex items-center gap-1.5 px-2 py-0.5 bg-green-500/20 text-green-400 rounded text-xs font-medium">
                          <Trophy size={12} />
                          {runScores[sel.id].betterThanMedian}/{runScores[sel.id].totalTests}
                        </span>
                      )}
                    </div>
                    <div className="flex flex-wrap gap-2 text-xs">
                      <span className="inline-flex items-center gap-1 px-2 py-0.5 bg-wb-bg-secondary rounded">
                        <Cpu size={12} className="text-wb-accent-light" />
                        {sel.run.cpu_name}
                      </span>
                      <span className="inline-flex items-center gap-1 px-2 py-0.5 bg-wb-bg-secondary rounded">
                        <MemoryStick size={12} className="text-wb-accent-light" />
                        {sel.run.memory_gb.toFixed(0)} GB
                      </span>
                      <span className="inline-flex items-center gap-1 px-2 py-0.5 bg-wb-bg-secondary rounded">
                        <Monitor size={12} className="text-wb-accent-light" />
                        {sel.run.os_name}
                      </span>
                      {sel.run.storage_type && (
                        <span className="inline-flex items-center gap-1 px-2 py-0.5 bg-wb-bg-secondary rounded">
                          <HardDrive size={12} className="text-wb-accent-light" />
                          {sel.run.storage_type}
                        </span>
                      )}
                    </div>
                  </div>

                  {/* Only show edit/delete for first selection */}
                  {index === 0 && (
                    <div className="flex items-center gap-2 shrink-0">
                      <button
                        onClick={() => {
                          setShowEditModal(true)
                          setEditDisplayName(sel.run.display_name || '')
                          setEditUserName(sel.run.user_name || '')
                          setEditDescription(sel.run.description || '')
                          setEditPassword('')
                          setEditError(null)
                        }}
                        className="btn-secondary flex items-center gap-2 text-xs hover:bg-wb-bg-secondary shrink-0 p-1.5"
                        title="Edit"
                      >
                        <Pencil size={14} />
                      </button>
                      <button
                        onClick={() => {
                          setShowDeleteModal(true)
                          setDeletePassword('')
                          setDeleteError(null)
                        }}
                        className="btn-secondary flex items-center gap-2 text-xs text-wb-error hover:bg-wb-error/20 hover:border-wb-error shrink-0 p-1.5"
                        title="Delete"
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  )}
                </div>
              ))}
            </div>
            )}

            {/* Comparison Charts */}
            <div className="space-y-6">
              {Object.entries(categories).map(([key, cat]) => (
                cat.tests.length > 0 && (
                  <CompactComparisonChart
                    key={key}
                    tests={cat.tests}
                    title={cat.name}
                    selections={selections}
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

      {/* Edit Modal */}
      {showEditModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="card max-w-md w-full mx-4">
            <div className="flex items-center justify-between mb-6">
              <h3 className="text-xl font-semibold">Edit Benchmark</h3>
              <button
                onClick={() => setShowEditModal(false)}
                className="text-wb-text-secondary hover:text-white transition-colors"
              >
                <X size={24} />
              </button>
            </div>
            <form onSubmit={handleEdit}>
              <div className="mb-4">
                <label className="block text-sm text-wb-text-secondary mb-2">
                  Display Name
                </label>
                <input
                  type="text"
                  value={editDisplayName}
                  onChange={(e) => setEditDisplayName(e.target.value)}
                  placeholder="Enter display name"
                  className="input w-full"
                  autoFocus
                />
              </div>
              <div className="mb-4">
                <label className="block text-sm text-wb-text-secondary mb-2">
                  User Name (optional)
                </label>
                <input
                  type="text"
                  value={editUserName}
                  onChange={(e) => setEditUserName(e.target.value)}
                  placeholder="Your name or alias"
                  className="input w-full"
                />
              </div>
              <div className="mb-4">
                <label className="block text-sm text-wb-text-secondary mb-2">
                  Description (optional)
                </label>
                <input
                  type="text"
                  value={editDescription}
                  onChange={(e) => setEditDescription(e.target.value)}
                  placeholder="e.g. VDI, Desktop, Laptop"
                  className="input w-full"
                />
              </div>
              <div className="mb-4">
                <label className="block text-sm text-wb-text-secondary mb-2">
                  Admin Password
                </label>
                <input
                  type="password"
                  value={editPassword}
                  onChange={(e) => setEditPassword(e.target.value)}
                  placeholder="Enter admin password"
                  className="input w-full"
                />
              </div>
              {editError && (
                <p className="text-wb-error text-sm mb-4">{editError}</p>
              )}
              <div className="flex gap-3 justify-end">
                <button
                  type="button"
                  onClick={() => setShowEditModal(false)}
                  className="btn-secondary"
                  disabled={editing}
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="btn-primary flex items-center gap-2"
                  disabled={editing || !editPassword || !editDisplayName.trim()}
                >
                  {editing ? (
                    <Loader2 size={18} className="animate-spin" />
                  ) : (
                    <Pencil size={18} />
                  )}
                  Save
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}
