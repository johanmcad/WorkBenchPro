import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
  Cell,
} from 'recharts'

export default function DistributionChart({
  statistics,
  percentileData,
  title,
}) {
  if (!statistics || !statistics.histogram_buckets || statistics.histogram_buckets.length === 0) {
    return (
      <div className="card">
        <h3 className="text-lg font-semibold mb-2">{title}</h3>
        <p className="text-wb-text-secondary text-sm">No data available</p>
      </div>
    )
  }

  const { histogram_buckets, min_value, max_value, p50, unit } = statistics
  const userValue = percentileData?.user_value
  const percentileRank = percentileData?.percentile_rank
  const isHigherBetter = percentileData?.is_higher_better ?? true

  // Prepare chart data
  const chartData = histogram_buckets.map((bucket, idx) => ({
    name: formatValue(bucket.bucket_start),
    range: `${formatValue(bucket.bucket_start)} - ${formatValue(bucket.bucket_end)}`,
    count: bucket.count,
    bucket_start: bucket.bucket_start,
    bucket_end: bucket.bucket_end,
    isUserBucket: userValue !== undefined &&
      userValue >= bucket.bucket_start &&
      userValue < bucket.bucket_end,
  }))

  // Mark the last bucket as user's if they're at the max
  if (userValue !== undefined && chartData.length > 0) {
    const lastBucket = chartData[chartData.length - 1]
    if (userValue >= lastBucket.bucket_start && userValue <= statistics.max_value) {
      lastBucket.isUserBucket = true
    }
  }

  // Percentile display
  const getPercentileLabel = () => {
    if (percentileRank === undefined) return null

    // For higher-is-better metrics: if you beat 85% of results, you're in the top 15%
    // For lower-is-better metrics: if you beat 85% of results, you're in the top 15%
    const topPercent = 100 - percentileRank

    if (topPercent <= 10) {
      return { text: `Top ${topPercent.toFixed(0)}%`, color: 'text-green-400' }
    } else if (topPercent <= 25) {
      return { text: `Top ${topPercent.toFixed(0)}%`, color: 'text-green-300' }
    } else if (topPercent <= 50) {
      return { text: `Top ${topPercent.toFixed(0)}%`, color: 'text-yellow-400' }
    } else {
      return { text: `Top ${topPercent.toFixed(0)}%`, color: 'text-orange-400' }
    }
  }

  const percentileLabel = getPercentileLabel()

  return (
    <div className="card">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-semibold">{title}</h3>
          <p className="text-wb-text-secondary text-sm">
            {statistics.sample_count} samples | {unit}
          </p>
        </div>
        {percentileLabel && (
          <div className="text-right">
            <span className={`text-lg font-bold ${percentileLabel.color}`}>
              {percentileLabel.text}
            </span>
            {userValue !== undefined && (
              <p className="text-wb-text-secondary text-sm">
                Your score: {formatValue(userValue)} {unit}
              </p>
            )}
          </div>
        )}
      </div>

      {/* Statistics summary */}
      <div className="flex gap-4 mb-4 text-sm">
        <div className="bg-wb-bg-secondary px-3 py-1 rounded">
          <span className="text-wb-text-secondary">Min:</span>{' '}
          <span className="text-white">{formatValue(min_value)}</span>
        </div>
        <div className="bg-wb-bg-secondary px-3 py-1 rounded">
          <span className="text-wb-text-secondary">Median:</span>{' '}
          <span className="text-white">{formatValue(p50)}</span>
        </div>
        <div className="bg-wb-bg-secondary px-3 py-1 rounded">
          <span className="text-wb-text-secondary">Max:</span>{' '}
          <span className="text-white">{formatValue(max_value)}</span>
        </div>
        {!isHigherBetter && (
          <div className="bg-yellow-500/20 px-3 py-1 rounded text-yellow-400">
            Lower is better
          </div>
        )}
      </div>

      {/* Distribution histogram */}
      <div className="h-48">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={chartData}
            margin={{ top: 10, right: 30, left: 0, bottom: 0 }}
          >
            <CartesianGrid strokeDasharray="3 3" stroke="#2c3e50" vertical={false} />
            <XAxis
              dataKey="name"
              stroke="#95a5a6"
              fontSize={11}
              tick={{ fill: '#95a5a6' }}
              interval="preserveStartEnd"
            />
            <YAxis
              stroke="#95a5a6"
              fontSize={11}
              tick={{ fill: '#95a5a6' }}
              allowDecimals={false}
            />
            <Tooltip
              contentStyle={{
                backgroundColor: '#16213e',
                border: '1px solid #2c3e50',
                borderRadius: '8px',
              }}
              formatter={(value, name, props) => [
                `${value} runs`,
                `Range: ${props.payload.range}`,
              ]}
              labelStyle={{ color: '#95a5a6' }}
            />
            {/* Median reference line */}
            <ReferenceLine
              x={findClosestBucketName(chartData, p50)}
              stroke="#f1c40f"
              strokeDasharray="5 5"
              label={{
                value: 'Median',
                position: 'top',
                fill: '#f1c40f',
                fontSize: 11,
              }}
            />
            {/* User's score reference line */}
            {userValue !== undefined && (
              <ReferenceLine
                x={findClosestBucketName(chartData, userValue)}
                stroke="#2ecc71"
                strokeWidth={2}
                label={{
                  value: 'You',
                  position: 'top',
                  fill: '#2ecc71',
                  fontSize: 11,
                  fontWeight: 'bold',
                }}
              />
            )}
            <Bar dataKey="count" radius={[4, 4, 0, 0]}>
              {chartData.map((entry, index) => (
                <Cell
                  key={`cell-${index}`}
                  fill={entry.isUserBucket ? '#2ecc71' : '#3b82f6'}
                />
              ))}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Legend */}
      <div className="flex gap-4 mt-2 text-xs text-wb-text-secondary justify-center">
        <div className="flex items-center gap-1">
          <div className="w-3 h-3 bg-[#3b82f6] rounded" />
          <span>Community</span>
        </div>
        {userValue !== undefined && (
          <div className="flex items-center gap-1">
            <div className="w-3 h-3 bg-[#2ecc71] rounded" />
            <span>Your Result</span>
          </div>
        )}
        <div className="flex items-center gap-1">
          <div className="w-3 h-0.5 bg-[#f1c40f]" style={{ marginTop: '2px' }} />
          <span>Median</span>
        </div>
      </div>
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

function findClosestBucketName(chartData, value) {
  if (!chartData.length || value === undefined) return null

  // Find the bucket that contains this value
  for (const bucket of chartData) {
    if (value >= bucket.bucket_start && value < bucket.bucket_end) {
      return bucket.name
    }
  }

  // If not found (value is at max), return last bucket
  return chartData[chartData.length - 1]?.name
}
