import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import {
  ArrowLeft,
  Loader2,
  BarChart3,
  Trophy,
  TrendingUp,
  TrendingDown,
} from 'lucide-react'
import { fetchBenchmarkRun, fetchTestStatistics, fetchPercentileRank } from '../api'
import DistributionChart from '../components/DistributionChart'

export default function CommunityComparisonPage() {
  const { id } = useParams()
  const [run, setRun] = useState(null)
  const [statistics, setStatistics] = useState([])
  const [percentileRanks, setPercentileRanks] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [activeCategory, setActiveCategory] = useState('all')

  useEffect(() => {
    const load = async () => {
      setLoading(true)
      setError(null)
      try {
        // Fetch all data in parallel
        const [runData, statsData, ranksData] = await Promise.all([
          fetchBenchmarkRun(id),
          fetchTestStatistics(),
          fetchPercentileRank(id),
        ])

        setRun(runData)
        setStatistics(statsData)
        setPercentileRanks(ranksData)
      } catch (err) {
        console.error('Error loading comparison data:', err)
        setError(err.message)
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [id])

  if (loading) {
    return (
      <div className="flex items-center justify-center py-32">
        <Loader2 size={32} className="animate-spin text-wb-accent" />
      </div>
    )
  }

  if (error || !run) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="card text-center py-12">
          <p className="text-wb-error mb-4">{error || 'Result not found'}</p>
          <Link to="/results" className="btn-secondary">
            Back to Results
          </Link>
        </div>
      </div>
    )
  }

  // Categorize tests
  const categories = {
    project_operations: { name: 'Project Operations', color: '#3b82f6', tests: [] },
    build_performance: { name: 'Build Performance', color: '#8b5cf6', tests: [] },
    responsiveness: { name: 'Responsiveness', color: '#10b981', tests: [] },
  }

  // Map user's tests by test_id for quick lookup
  const userTestsMap = new Map()
  const allUserTests = [
    ...(run.results.project_operations || []),
    ...(run.results.build_performance || []),
    ...(run.results.responsiveness || []),
  ]
  allUserTests.forEach(t => userTestsMap.set(t.test_id, t))

  // Map percentile ranks by test_id
  const percentileMap = new Map()
  percentileRanks.forEach(p => percentileMap.set(p.test_id, p))

  // Categorize statistics
  statistics.forEach((stat) => {
    const userTest = userTestsMap.get(stat.test_id)
    const percentile = percentileMap.get(stat.test_id)

    // Determine category by checking which array the test is in
    let category = null
    if (run.results.project_operations?.some(t => t.test_id === stat.test_id)) {
      category = 'project_operations'
    } else if (run.results.build_performance?.some(t => t.test_id === stat.test_id)) {
      category = 'build_performance'
    } else if (run.results.responsiveness?.some(t => t.test_id === stat.test_id)) {
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

  // Calculate overall stats
  const overallStats = calculateOverallStats(percentileRanks)

  // Get tests to display based on active category
  const getDisplayedTests = () => {
    if (activeCategory === 'all') {
      return Object.entries(categories).flatMap(([key, cat]) =>
        cat.tests.map(t => ({ ...t, categoryKey: key, categoryColor: cat.color }))
      )
    }
    const cat = categories[activeCategory]
    return cat ? cat.tests.map(t => ({ ...t, categoryKey: activeCategory, categoryColor: cat.color })) : []
  }

  const displayedTests = getDisplayedTests()

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Back link */}
      <Link
        to={`/results/${id}`}
        className="inline-flex items-center gap-2 text-wb-text-secondary hover:text-white mb-6 transition-colors"
      >
        <ArrowLeft size={20} />
        Back to Results
      </Link>

      {/* Header */}
      <div className="card mb-8">
        <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
          <div>
            <h1 className="text-2xl md:text-3xl font-bold flex items-center gap-3">
              <BarChart3 className="text-wb-accent" />
              Community Comparison
            </h1>
            <p className="text-wb-text-secondary mt-1">
              See how your benchmark results compare to the community
            </p>
          </div>

          {/* Overall Summary */}
          <div className="flex gap-4">
            <div className="bg-wb-bg-secondary px-4 py-3 rounded-lg text-center">
              <div className="text-2xl font-bold text-wb-accent">
                {overallStats.avgPercentile.toFixed(0)}%
              </div>
              <div className="text-xs text-wb-text-secondary">Avg Percentile</div>
            </div>
            <div className="bg-wb-bg-secondary px-4 py-3 rounded-lg text-center">
              <div className="text-2xl font-bold text-green-400">
                {overallStats.top25Count}
              </div>
              <div className="text-xs text-wb-text-secondary">Top 25% Tests</div>
            </div>
            <div className="bg-wb-bg-secondary px-4 py-3 rounded-lg text-center">
              <div className="text-2xl font-bold text-white">
                {overallStats.totalTests}
              </div>
              <div className="text-xs text-wb-text-secondary">Total Tests</div>
            </div>
          </div>
        </div>
      </div>

      {/* Run Info */}
      <div className="card mb-6">
        <h2 className="text-lg font-semibold mb-3">{run.display_name}</h2>
        <div className="flex flex-wrap gap-4 text-sm">
          <span className="bg-wb-bg-secondary px-3 py-1 rounded">
            {run.cpu_name}
          </span>
          <span className="bg-wb-bg-secondary px-3 py-1 rounded">
            {run.memory_gb.toFixed(0)} GB RAM
          </span>
          <span className="bg-wb-bg-secondary px-3 py-1 rounded">
            {run.os_name}
          </span>
          {run.storage_type && (
            <span className="bg-wb-bg-secondary px-3 py-1 rounded">
              {run.storage_type}
            </span>
          )}
        </div>
      </div>

      {/* Category Tabs */}
      <div className="flex flex-wrap gap-2 mb-6">
        <button
          onClick={() => setActiveCategory('all')}
          className={`px-4 py-2 rounded-lg transition-colors ${
            activeCategory === 'all'
              ? 'bg-wb-accent text-white'
              : 'bg-wb-bg-card text-wb-text-secondary hover:text-white'
          }`}
        >
          All Tests ({displayedTests.length})
        </button>
        {Object.entries(categories).map(([key, cat]) => (
          <button
            key={key}
            onClick={() => setActiveCategory(key)}
            className={`px-4 py-2 rounded-lg transition-colors ${
              activeCategory === key
                ? 'text-white'
                : 'bg-wb-bg-card text-wb-text-secondary hover:text-white'
            }`}
            style={activeCategory === key ? { backgroundColor: cat.color } : {}}
          >
            {cat.name} ({cat.tests.length})
          </button>
        ))}
      </div>

      {/* Percentile Summary Grid */}
      <div className="card mb-6">
        <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
          <Trophy className="text-yellow-400" size={20} />
          Your Rankings
        </h3>
        <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
          {displayedTests.map((test) => {
            const percentile = test.percentile
            if (!percentile) return null

            const topPercent = 100 - percentile.percentile_rank
            const bgColor = getPercentileColor(topPercent)

            return (
              <div
                key={test.test_id}
                className={`${bgColor} px-3 py-2 rounded-lg text-center`}
              >
                <div className="text-xs text-white/70 truncate" title={test.test_name}>
                  {test.test_name}
                </div>
                <div className="text-lg font-bold text-white flex items-center justify-center gap-1">
                  {topPercent <= 50 ? (
                    <TrendingUp size={14} className="text-green-300" />
                  ) : (
                    <TrendingDown size={14} className="text-red-300" />
                  )}
                  Top {topPercent.toFixed(0)}%
                </div>
              </div>
            )
          })}
        </div>
      </div>

      {/* Distribution Charts */}
      <div className="space-y-6">
        <h3 className="text-lg font-semibold">Distribution Charts</h3>
        {displayedTests.length === 0 ? (
          <div className="card text-center py-8">
            <p className="text-wb-text-secondary">No test data available</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {displayedTests.map((test) => (
              <DistributionChart
                key={test.test_id}
                statistics={test}
                percentileData={test.percentile}
                title={test.test_name}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  )
}

function calculateOverallStats(percentileRanks) {
  if (!percentileRanks.length) {
    return { avgPercentile: 0, top25Count: 0, totalTests: 0 }
  }

  const totalTests = percentileRanks.length
  const avgPercentile =
    percentileRanks.reduce((sum, p) => sum + p.percentile_rank, 0) / totalTests

  // Count tests where user is in top 25%
  const top25Count = percentileRanks.filter(
    (p) => 100 - p.percentile_rank <= 25
  ).length

  return { avgPercentile, top25Count, totalTests }
}

function getPercentileColor(topPercent) {
  if (topPercent <= 10) return 'bg-green-600'
  if (topPercent <= 25) return 'bg-green-500/80'
  if (topPercent <= 50) return 'bg-yellow-500/80'
  if (topPercent <= 75) return 'bg-orange-500/80'
  return 'bg-red-500/80'
}
