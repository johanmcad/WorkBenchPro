import { useState } from 'react'
import { ChevronDown, ChevronRight } from 'lucide-react'

export default function CompactComparisonChart({
  tests,
  title,
}) {
  const [expandedTest, setExpandedTest] = useState(null)

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
            <span className="text-red-400/70">← worse</span>
            <div className="flex items-center gap-1">
              <div className="w-2 h-2 rounded-full bg-green-400" />
              <span>You</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-2 h-2 rounded-full bg-yellow-400" />
              <span>Median</span>
            </div>
            <span className="text-green-400/70">better →</span>
          </div>
        </div>
      )}

      <div className="space-y-1">
        {tests.map((test) => (
          <TestRow
            key={test.test_id}
            test={test}
            isExpanded={expandedTest === test.test_id}
            onToggle={() => setExpandedTest(expandedTest === test.test_id ? null : test.test_id)}
          />
        ))}
      </div>
    </div>
  )
}

// Infer if higher is better based on unit
function inferHigherIsBetter(unit) {
  if (!unit) return true
  const lowerUnit = unit.toLowerCase()
  // Time/latency units - lower is better
  if (['sec', 's', 'ms', 'μs', 'us', 'ns', 'seconds', 'milliseconds'].some(u => lowerUnit.includes(u))) {
    return false
  }
  // Percentage (overhead) - lower is better
  if (lowerUnit.includes('percent') || lowerUnit.includes('%')) {
    return false
  }
  // Throughput/speed units - higher is better
  if (['mb/s', 'gb/s', '/s', 'ops', 'files'].some(u => lowerUnit.includes(u))) {
    return true
  }
  return true // default
}

function TestRow({ test, isExpanded, onToggle }) {
  const { test_name, min_value, max_value, p50, p25, p75, p90, p95, unit, percentile, sample_count } = test
  const userValue = percentile?.user_value
  // Use inferred value based on unit, fallback to database value
  const isHigherBetter = inferHigherIsBetter(unit)

  // Calculate positions as percentages (flip for lower-is-better so right = better)
  const range = max_value - min_value
  let userPosition = range > 0 ? ((userValue - min_value) / range) * 100 : 50
  let medianPosition = range > 0 ? ((p50 - min_value) / range) * 100 : 50

  // For lower-is-better, flip positions so right = better (lower values)
  if (!isHigherBetter) {
    userPosition = 100 - userPosition
    medianPosition = 100 - medianPosition
  }

  // Display values: left = worst, right = best
  const leftValue = isHigherBetter ? min_value : max_value
  const rightValue = isHigherBetter ? max_value : min_value

  return (
    <div>
      <div
        className="flex items-center gap-3 group cursor-pointer hover:bg-wb-bg-secondary/30 rounded px-1 -mx-1"
        onClick={onToggle}
      >
        {/* Expand icon */}
        <div className="w-4 shrink-0 text-wb-text-secondary">
          {isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />}
        </div>

        {/* Test name - fixed width */}
        <div className="w-44 shrink-0">
          <span className="text-xs truncate" title={test_name}>
            {test_name}
          </span>
        </div>

        {/* Range bar */}
        <div className="flex-1 flex items-center gap-2">
          <span className="text-[9px] text-wb-text-secondary w-10 text-right shrink-0">
            {formatValue(leftValue)}
          </span>

          {/* Bar */}
          <div className="flex-1 relative h-2">
            <div className="absolute inset-0 bg-wb-border rounded-full" />
            <div
              className="absolute inset-0 rounded-full opacity-50"
              style={{
                background: 'linear-gradient(90deg, rgba(239,68,68,0.3) 0%, rgba(59,130,246,0.3) 50%, rgba(16,185,129,0.3) 100%)'
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
            {formatValue(rightValue)}
          </span>
        </div>

        {/* User value */}
        <div className="flex items-center gap-2 shrink-0">
          {userValue !== undefined && (
            <span className="text-[10px] text-green-400 font-medium whitespace-nowrap">
              {formatValue(userValue)} {unit}
            </span>
          )}
        </div>
      </div>

      {/* Expanded details */}
      {isExpanded && (
        <div className="ml-5 mt-2 mb-3 p-3 bg-wb-bg-secondary/50 rounded-lg text-xs">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            <div>
              <div className="text-wb-text-secondary text-[10px]">Your Value</div>
              <div className="text-green-400 font-medium">
                {userValue !== undefined ? `${formatValue(userValue)} ${unit}` : '-'}
              </div>
            </div>
            <div>
              <div className="text-wb-text-secondary text-[10px]">Median (P50)</div>
              <div className="text-yellow-400 font-medium">{formatValue(p50)} {unit}</div>
            </div>
            <div>
              <div className="text-wb-text-secondary text-[10px]">Min</div>
              <div className="text-white">{formatValue(min_value)} {unit}</div>
            </div>
            <div>
              <div className="text-wb-text-secondary text-[10px]">Max</div>
              <div className="text-white">{formatValue(max_value)} {unit}</div>
            </div>
            {p25 !== undefined && (
              <div>
                <div className="text-wb-text-secondary text-[10px]">P25</div>
                <div className="text-white">{formatValue(p25)} {unit}</div>
              </div>
            )}
            {p75 !== undefined && (
              <div>
                <div className="text-wb-text-secondary text-[10px]">P75</div>
                <div className="text-white">{formatValue(p75)} {unit}</div>
              </div>
            )}
            {p90 !== undefined && (
              <div>
                <div className="text-wb-text-secondary text-[10px]">P90</div>
                <div className="text-white">{formatValue(p90)} {unit}</div>
              </div>
            )}
            {p95 !== undefined && (
              <div>
                <div className="text-wb-text-secondary text-[10px]">P95</div>
                <div className="text-white">{formatValue(p95)} {unit}</div>
              </div>
            )}
            {sample_count !== undefined && (
              <div>
                <div className="text-wb-text-secondary text-[10px]">Samples</div>
                <div className="text-white">{sample_count}</div>
              </div>
            )}
          </div>
        </div>
      )}
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
