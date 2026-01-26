import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import {
  ArrowLeft,
  Cpu,
  HardDrive,
  Monitor,
  Calendar,
  Loader2,
  GitCompare,
  MemoryStick,
} from 'lucide-react'
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from 'recharts'
import { fetchBenchmarkRun } from '../api'

export default function ResultDetailPage() {
  const { id } = useParams()
  const [result, setResult] = useState(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)

  useEffect(() => {
    const load = async () => {
      setLoading(true)
      setError(null)
      try {
        const data = await fetchBenchmarkRun(id)
        setResult(data)
      } catch (err) {
        setError(err.message)
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [id])

  const formatDate = (dateStr) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center py-32">
        <Loader2 size={32} className="animate-spin text-wb-accent" />
      </div>
    )
  }

  if (error || !result) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="card text-center py-12">
          <p className="text-wb-error mb-4">
            {error || 'Result not found'}
          </p>
          <Link to="/results" className="btn-secondary">
            Back to Results
          </Link>
        </div>
      </div>
    )
  }

  const { results, system_info } = result

  const categories = [
    { key: 'project_operations', name: 'Project Operations', color: '#3b82f6' },
    { key: 'build_performance', name: 'Build Performance', color: '#8b5cf6' },
    { key: 'responsiveness', name: 'Responsiveness', color: '#10b981' },
  ]

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Back link */}
      <Link
        to="/results"
        className="inline-flex items-center gap-2 text-wb-text-secondary hover:text-white mb-6 transition-colors"
      >
        <ArrowLeft size={20} />
        Back to Results
      </Link>

      {/* Header Card */}
      <div className="card mb-8">
        <div className="flex flex-col md:flex-row md:items-start md:justify-between gap-6">
          <div>
            <h1 className="text-2xl md:text-3xl font-bold mb-2">
              {result.display_name}
            </h1>
            <div className="flex items-center gap-2 text-wb-text-secondary mb-4">
              <Calendar size={16} />
              {formatDate(result.uploaded_at)}
            </div>

            {/* System Specs */}
            <div className="flex flex-wrap gap-3">
              <span className="inline-flex items-center gap-2 px-4 py-2 bg-wb-bg-secondary rounded-lg">
                <Cpu size={18} className="text-wb-accent-light" />
                <div>
                  <div className="text-sm text-wb-text-secondary">CPU</div>
                  <div className="font-medium">{system_info.cpu.name}</div>
                </div>
              </span>
              <span className="inline-flex items-center gap-2 px-4 py-2 bg-wb-bg-secondary rounded-lg">
                <Cpu size={18} className="text-wb-accent-light" />
                <div>
                  <div className="text-sm text-wb-text-secondary">Cores/Threads</div>
                  <div className="font-medium">{system_info.cpu.cores}C / {system_info.cpu.threads}T</div>
                </div>
              </span>
              <span className="inline-flex items-center gap-2 px-4 py-2 bg-wb-bg-secondary rounded-lg">
                <MemoryStick size={18} className="text-wb-accent-light" />
                <div>
                  <div className="text-sm text-wb-text-secondary">Memory</div>
                  <div className="font-medium">{(system_info.memory.total_bytes / 1073741824).toFixed(0)} GB</div>
                </div>
              </span>
              <span className="inline-flex items-center gap-2 px-4 py-2 bg-wb-bg-secondary rounded-lg">
                <Monitor size={18} className="text-wb-accent-light" />
                <div>
                  <div className="text-sm text-wb-text-secondary">OS</div>
                  <div className="font-medium">{system_info.os.name}</div>
                </div>
              </span>
              {system_info.storage?.[0] && (
                <span className="inline-flex items-center gap-2 px-4 py-2 bg-wb-bg-secondary rounded-lg">
                  <HardDrive size={18} className="text-wb-accent-light" />
                  <div>
                    <div className="text-sm text-wb-text-secondary">Storage</div>
                    <div className="font-medium">{system_info.storage[0].device_type}</div>
                  </div>
                </span>
              )}
            </div>
          </div>

          <Link
            to={`/compare?id=${id}`}
            className="btn-primary flex items-center gap-2 flex-shrink-0"
          >
            <GitCompare size={18} />
            Compare
          </Link>
        </div>
      </div>

      {/* Benchmark Results by Category */}
      {categories.map(({ key, name, color }) => {
        const categoryResults = results[key] || []
        if (categoryResults.length === 0) return null

        const chartData = categoryResults.map((r) => ({
          name: r.name.length > 20 ? r.name.substring(0, 20) + '...' : r.name,
          fullName: r.name,
          value: r.value,
          unit: r.unit,
        }))

        return (
          <div key={key} className="card mb-6">
            <h2 className="text-xl font-semibold mb-6 flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: color }}
              />
              {name}
              <span className="text-wb-text-secondary font-normal text-sm ml-2">
                ({categoryResults.length} tests)
              </span>
            </h2>

            {/* Chart */}
            <div className="h-64 mb-6">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart
                  data={chartData}
                  layout="vertical"
                  margin={{ top: 0, right: 30, left: 100, bottom: 0 }}
                >
                  <CartesianGrid strokeDasharray="3 3" stroke="#2c3e50" horizontal={false} />
                  <XAxis type="number" stroke="#95a5a6" fontSize={12} />
                  <YAxis
                    type="category"
                    dataKey="name"
                    stroke="#95a5a6"
                    fontSize={12}
                    width={100}
                  />
                  <Tooltip
                    contentStyle={{
                      backgroundColor: '#16213e',
                      border: '1px solid #2c3e50',
                      borderRadius: '8px',
                    }}
                    formatter={(value, name, props) => [
                      `${value.toFixed(2)} ${props.payload.unit}`,
                      props.payload.fullName,
                    ]}
                  />
                  <Bar dataKey="value" radius={[0, 4, 4, 0]}>
                    {chartData.map((entry, index) => (
                      <Cell key={index} fill={color} />
                    ))}
                  </Bar>
                </BarChart>
              </ResponsiveContainer>
            </div>

            {/* Details Table */}
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-wb-border">
                    <th className="text-left py-3 px-4 text-wb-text-secondary font-medium">Test</th>
                    <th className="text-right py-3 px-4 text-wb-text-secondary font-medium">Value</th>
                    <th className="text-right py-3 px-4 text-wb-text-secondary font-medium">Min</th>
                    <th className="text-right py-3 px-4 text-wb-text-secondary font-medium">Max</th>
                    <th className="text-right py-3 px-4 text-wb-text-secondary font-medium">Std Dev</th>
                  </tr>
                </thead>
                <tbody>
                  {categoryResults.map((r, i) => (
                    <tr key={i} className="border-b border-wb-border/50 hover:bg-wb-bg-secondary/30">
                      <td className="py-3 px-4">
                        <div className="font-medium">{r.name}</div>
                        <div className="text-wb-text-secondary text-xs">{r.description}</div>
                      </td>
                      <td className="text-right py-3 px-4 font-mono" style={{ color }}>
                        {r.value.toFixed(2)} {r.unit}
                      </td>
                      <td className="text-right py-3 px-4 font-mono text-wb-text-secondary">
                        {r.details.min.toFixed(2)}
                      </td>
                      <td className="text-right py-3 px-4 font-mono text-wb-text-secondary">
                        {r.details.max.toFixed(2)}
                      </td>
                      <td className="text-right py-3 px-4 font-mono text-wb-text-secondary">
                        {r.details.std_dev.toFixed(2)}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )
      })}
    </div>
  )
}
