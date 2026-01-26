import { useState } from 'react'
import { ChevronDown, ChevronRight } from 'lucide-react'

// Test descriptions from benchmark specifications
const TEST_DESCRIPTIONS = {
  // Project Operations
  'File Enumeration': {
    what: 'Tests how fast the system can list files and directories.',
    how: 'Creates 30,000 small text files across 500 directories, then recursively enumerates all files 5 times.',
    why: 'Affects IDE project loading, git status, file explorer browsing.',
  },
  'Small File Random Read': {
    what: 'Tests random access read performance on storage.',
    how: 'Creates a 1GB file with random data, then performs 10,000 random 4KB reads at random offsets.',
    why: 'Affects loading multiple source files, database queries, IDE responsiveness.',
  },
  'Metadata Operations': {
    what: 'Tests file metadata access speed (attributes, size, timestamps).',
    how: 'Creates test files then repeatedly queries file metadata using filesystem APIs.',
    why: 'Affects file browsers, backup tools, build systems checking timestamps.',
  },
  'Directory Traversal with Content': {
    what: 'Tests combined enumeration and partial file reading.',
    how: 'Creates 30,000 files across 500 directories, then traverses all directories reading the first 1KB of each file.',
    why: 'Simulates grep/ripgrep searching through source code, IDE indexing.',
  },
  'Large File Sequential Read': {
    what: 'Tests sequential read throughput for large files.',
    how: 'Creates a 2GB file with random data, then reads it sequentially in 1MB chunks.',
    why: 'Affects opening large CAD files, video editing, database operations.',
  },
  'Git Operations': {
    what: 'Tests real git command performance.',
    how: 'Creates a git repository with 5,000 files, then benchmarks git status, diff, log, and add commands.',
    why: 'Directly measures developer workflow with version control.',
  },
  'Robocopy File Copy': {
    what: 'Tests Windows robust file copy performance.',
    how: 'Creates 1,200+ files across 20 directories, then runs robocopy copy and mirror operations.',
    why: 'Measures backup, deployment, and file synchronization speed.',
  },
  'File Search': {
    what: 'Tests Windows Search indexing service performance.',
    how: 'Queries the Windows Search index for common file patterns using the Windows Search API.',
    why: 'Affects Start menu search, File Explorer search, Outlook search.',
  },
  'Antivirus Impact': {
    what: 'Measures Windows Defender real-time scanning overhead.',
    how: 'Performs file operations and measures the time difference when Defender is actively scanning.',
    why: 'Shows antivirus impact on build times and file operations.',
  },
  'Symlink Operations': {
    what: 'Tests symbolic link creation and resolution.',
    how: 'Creates directory and file symbolic links, then resolves them to their targets.',
    why: 'Affects npm/pnpm (uses symlinks), development workflows, junctions.',
  },
  // Build Performance
  'Single-Thread Compute': {
    what: 'Tests single-core computational performance.',
    how: 'Generates 256MB of random data, then compresses it using LZ4 algorithm on a single CPU core.',
    why: 'Affects single-threaded build steps, script execution, startup time.',
  },
  'Multi-Thread Compute': {
    what: 'Tests parallel computational performance across all cores.',
    how: 'Generates data chunks for each CPU thread, then compresses all chunks in parallel using LZ4.',
    why: 'Affects parallel compilation, video encoding, data processing.',
  },
  'Mixed Read-Compute-Write': {
    what: 'Tests realistic mixed CPU operations.',
    how: 'Runs compression, hashing, and sorting operations concurrently across multiple threads.',
    why: 'Represents typical development workloads with varied operations.',
  },
  'Sustained Write Performance': {
    what: 'Tests continuous write performance with periodic sync.',
    how: 'Writes 4GB of data in 4MB chunks, calling fsync every 256MB to simulate realistic build output.',
    why: 'Affects build artifact generation, log writing, database commits.',
  },
  'Windows Compression': {
    what: 'Tests Windows native C# compilation performance.',
    how: 'Generates C# source files with classes and functions, then compiles with csc.exe.',
    why: 'Measures build performance using Windows built-in compiler.',
  },
  'Archive Operations': {
    what: 'Tests archive compression and extraction speed.',
    how: 'Creates 250 text files across 16 directories, then compresses and extracts with tar.',
    why: 'Affects npm install, artifact packaging, backup operations.',
  },
  'PowerShell Scripts': {
    what: 'Tests PowerShell script execution performance.',
    how: 'Executes scripts for compute, file operations, object manipulation, and string processing.',
    why: 'Affects build scripts, automation, deployment pipelines.',
  },
  // Responsiveness
  'Storage Latency Distribution': {
    what: 'Measures storage I/O latency distribution.',
    how: 'Creates 1GB test file, performs 10,000 random 4KB reads measuring individual operation times.',
    why: 'Shows SSD/HDD responsiveness, affects perceived system snappiness.',
  },
  'Process Spawn Time': {
    what: 'Tests process creation overhead.',
    how: 'Spawns cmd.exe 100 times, measuring time from spawn to process completion.',
    why: 'Affects build tools that spawn many processes (make, npm, cargo).',
  },
  'Thread Wake Latency': {
    what: 'Tests thread scheduler responsiveness.',
    how: 'Creates worker threads that sleep, then measures time to wake them using condition variables.',
    why: 'Affects async runtime performance, UI responsiveness, server latency.',
  },
  'Memory Latency': {
    what: 'Tests memory subsystem access latency.',
    how: 'Creates a large array with pointer-chasing pattern, then follows the chain measuring access time.',
    why: 'Affects cache-unfriendly workloads, large data structure traversal.',
  },
  'Memory Bandwidth': {
    what: 'Tests memory throughput across all cores.',
    how: 'Allocates large buffers per thread (~1GB total), performs memcpy operations in parallel.',
    why: 'Affects data processing, video editing, scientific computing.',
  },
  // System Tools
  'Registry Operations': {
    what: 'Tests Windows Registry access speed.',
    how: 'Opens registry keys, reads and writes test values, enumerates subkeys.',
    why: 'Affects app startup (reading settings), installers, system tools.',
  },
  'Event Log Query': {
    what: 'Tests Windows Event Log query performance.',
    how: 'Queries the Application, System, and Security event logs using Windows Event Log API.',
    why: 'Affects monitoring tools, debugging, security auditing.',
  },
  'Task Scheduler': {
    what: 'Tests Task Scheduler API performance.',
    how: 'Queries Windows Task Scheduler for all scheduled tasks, enumerates task folders and properties.',
    why: 'Affects system administration tools, backup schedulers.',
  },
  'Application Launch': {
    what: 'Tests built-in Windows application startup time.',
    how: 'Launches Notepad, WordPad, Calculator, MSPaint, and cmd.exe, measuring time until running.',
    why: 'Indicates overall system responsiveness for starting applications.',
  },
  'Windows Services': {
    what: 'Tests Windows Service Manager performance.',
    how: 'Enumerates all Windows services, retrieves service status and configuration.',
    why: 'Affects services.msc, system administration, monitoring tools.',
  },
  'Network Info': {
    what: 'Tests network configuration query speed.',
    how: 'Queries network adapters, IP configuration, routing tables, and DNS settings.',
    why: 'Affects network troubleshooting tools, VPN clients, system info.',
  },
  'WMI Query': {
    what: 'Tests Windows Management Instrumentation performance.',
    how: 'Executes WMI queries for OS details, CPU info, disk drives, and memory.',
    why: 'Affects system monitoring, inventory tools, PowerShell scripts.',
  },
  'Process Management': {
    what: 'Tests process enumeration speed.',
    how: 'Enumerates all running processes, retrieves process names, PIDs, and memory usage.',
    why: 'Affects Task Manager, process monitors, debugging tools.',
  },
  'Environment Variables': {
    what: 'Tests environment variable access speed.',
    how: 'Reads and writes environment variables, enumerates all variables.',
    why: 'Affects process startup, build scripts, configuration loading.',
  },
}

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
  const lowerUnit = unit.toLowerCase().trim()

  // Check for units with "/" (rates)
  if (lowerUnit.includes('/')) {
    const beforeSlash = lowerUnit.split('/')[0]
    // If time unit is BEFORE the slash (like "ms/op", "sec/iter"), lower is better
    // These measure "time per operation" - less time = better
    if (['ms', 's', 'sec', 'μs', 'us', 'ns'].some(t => beforeSlash === t || beforeSlash.endsWith(t))) {
      return false
    }
    // Otherwise it's "X per time" (like files/sec, MB/s) - higher is better
    return true
  }

  // Throughput keywords without slash
  if (lowerUnit.includes('throughput') || lowerUnit.includes('bandwidth')) {
    return true
  }

  // Exact matches for short time units (e.g., "s", "ms", "ns")
  if (['s', 'ms', 'ns', 'μs', 'us'].includes(lowerUnit)) {
    return false
  }

  // Time/latency/duration units - lower is better
  if (['sec', 'second', 'ms', 'millisec', 'μs', 'us', 'ns', 'nanosec', 'latency', 'time'].some(u => lowerUnit.includes(u))) {
    return false
  }

  // Percentage (overhead) - lower is better
  if (lowerUnit.includes('percent') || lowerUnit.includes('%')) {
    return false
  }

  return true // default higher is better
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
          {/* Test description */}
          {TEST_DESCRIPTIONS[test_name] && (
            <div className="mb-3 pb-3 border-b border-wb-border/50">
              <div className="text-white mb-1">{TEST_DESCRIPTIONS[test_name].what}</div>
              <div className="text-wb-text-secondary text-[10px] mb-1">
                <span className="text-wb-accent">How:</span> {TEST_DESCRIPTIONS[test_name].how}
              </div>
              <div className="text-wb-text-secondary text-[10px]">
                <span className="text-wb-accent">Why it matters:</span> {TEST_DESCRIPTIONS[test_name].why}
              </div>
            </div>
          )}
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
