import { TrendingUp, TrendingDown } from 'lucide-react'

export default function CompactComparisonChart({
  tests,
  title,
}) {
  if (!tests || tests.length === 0) {
    return (
      <div className="card">
        <h3 className="text-lg font-semibold mb-2">{title}</h3>
        <p className="text-wb-text-secondary text-sm">No data available</p>
      </div>
    )
  }

  return (
    <div className="card">
      {title && (
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-sm font-semibold text-wb-text-secondary">{title}</h3>
          {/* Legend */}
          <div className="flex gap-3 text-[10px] text-wb-text-secondary">
            <div className="flex items-center gap-1">
              <div className="w-2 h-2 rounded-full bg-green-400" />
              <span>Selected</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-2 h-2 rounded-full bg-yellow-400" />
              <span>Median</span>
            </div>
          </div>
        </div>
      )}

      <div className="space-y-1">
        {tests.map((test) => (
          <TestRow key={test.test_id} test={test} />
        ))}
      </div>
    </div>
  )
}

function TestRow({ test }) {
  const { test_name, min_value, max_value, p50, unit, percentile } = test
  const userValue = percentile?.user_value
  const percentileRank = percentile?.percentile_rank
  const isHigherBetter = percentile?.is_higher_better ?? true

  // Calculate positions as percentages
  const range = max_value - min_value
  const userPosition = range > 0 ? ((userValue - min_value) / range) * 100 : 50
  const medianPosition = range > 0 ? ((p50 - min_value) / range) * 100 : 50

  // Calculate top percent for badge
  const topPercent = percentileRank !== undefined ? 100 - percentileRank : null

  return (
    <div className="flex items-center gap-3 group">
      {/* Test name - fixed width */}
      <div className="w-44 shrink-0 flex items-center gap-1.5">
        <span className="text-xs truncate" title={test_name}>
          {test_name}
        </span>
        {!isHigherBetter && (
          <span className="text-[8px] text-yellow-400 bg-yellow-400/10 px-1 rounded shrink-0">
            ↓
          </span>
        )}
      </div>

      {/* Range bar */}
      <div className="flex-1 flex items-center gap-2">
        <span className="text-[9px] text-wb-text-secondary w-10 text-right shrink-0">
          {formatValue(min_value)}
        </span>

        {/* Bar */}
        <div className="flex-1 relative h-2">
          <div className="absolute inset-0 bg-wb-border rounded-full" />
          <div
            className="absolute inset-0 rounded-full opacity-50"
            style={{
              background: 'linear-gradient(90deg, rgba(59,130,246,0.3) 0%, rgba(59,130,246,0.5) 50%, rgba(59,130,246,0.3) 100%)'
            }}
          />

          {/* Median marker */}
          <div
            className="absolute top-1/2 -translate-y-1/2 w-0.5 h-3 bg-yellow-400 rounded-full z-10"
            style={{ left: `${Math.min(Math.max(medianPosition, 0), 100)}%` }}
            title={`Median: ${formatValue(p50)} ${unit}`}
          />

          {/* User marker */}
          {userValue !== undefined && (
            <div
              className="absolute top-1/2 -translate-y-1/2 z-20"
              style={{ left: `${Math.min(Math.max(userPosition, 0), 100)}%` }}
              title={`Your score: ${formatValue(userValue)} ${unit}`}
            >
              <div className="relative -translate-x-1/2">
                <div className="w-2.5 h-2.5 bg-green-400 rounded-full border border-wb-bg-card shadow" />
              </div>
            </div>
          )}
        </div>

        <span className="text-[9px] text-wb-text-secondary w-10 shrink-0">
          {formatValue(max_value)}
        </span>
      </div>

      {/* User value + Percentile */}
      <div className="flex items-center gap-2 shrink-0">
        {userValue !== undefined && (
          <span className="text-[10px] text-green-400 font-medium whitespace-nowrap">
            {formatValue(userValue)} {unit}
          </span>
        )}
        {topPercent !== null && (
          <PercentileBadge topPercent={topPercent} />
        )}
      </div>
    </div>
  )
}

function PercentileBadge({ topPercent }) {
  let bgColor, textColor, Icon

  if (topPercent <= 10) {
    bgColor = 'bg-green-500/20'
    textColor = 'text-green-400'
    Icon = TrendingUp
  } else if (topPercent <= 25) {
    bgColor = 'bg-green-500/15'
    textColor = 'text-green-300'
    Icon = TrendingUp
  } else if (topPercent <= 50) {
    bgColor = 'bg-yellow-500/15'
    textColor = 'text-yellow-400'
    Icon = TrendingUp
  } else if (topPercent <= 75) {
    bgColor = 'bg-orange-500/15'
    textColor = 'text-orange-400'
    Icon = TrendingDown
  } else {
    bgColor = 'bg-red-500/15'
    textColor = 'text-red-400'
    Icon = TrendingDown
  }

  return (
    <div className={`flex items-center gap-0.5 px-1.5 py-0.5 rounded ${bgColor} whitespace-nowrap`}>
      <Icon size={10} className={`${textColor} shrink-0`} />
      <span className={`text-[10px] font-medium ${textColor}`}>
        {topPercent.toFixed(0)}%
      </span>
    </div>
  )
}

function formatValue(value) {
  if (value === undefined || value === null) return '-'
  if (Math.abs(value) >= 10000) return value.toFixed(0)
  if (Math.abs(value) >= 100) return value.toFixed(1)
  if (Math.abs(value) >= 1) return value.toFixed(2)
  return value.toFixed(3)
}
