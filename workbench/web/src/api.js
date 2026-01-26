// Supabase API client for fetching benchmark data
const SUPABASE_URL = 'https://wqutewgfxtucshqwzecj.supabase.co'
const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndxdXRld2dmeHR1Y3NocXd6ZWNqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjkzNzQ4MDgsImV4cCI6MjA4NDk1MDgwOH0.Wx2Yd5ONcorGqLTeNsMgeOrkBQBwN8Kjag_rBXX60O8'

const headers = {
  'apikey': SUPABASE_KEY,
  'Authorization': `Bearer ${SUPABASE_KEY}`,
  'Content-Type': 'application/json',
}

export async function fetchBenchmarkRuns({ cpuFilter, osFilter, minMemory, limit = 50 } = {}) {
  let url = `${SUPABASE_URL}/rest/v1/benchmark_runs?select=id,display_name,description,cpu_name,cpu_cores,cpu_threads,memory_gb,os_name,storage_type,uploaded_at&order=uploaded_at.desc&limit=${limit}`

  if (cpuFilter) {
    url += `&cpu_name=ilike.*${encodeURIComponent(cpuFilter)}*`
  }
  if (osFilter) {
    url += `&os_name=ilike.*${encodeURIComponent(osFilter)}*`
  }
  if (minMemory) {
    url += `&memory_gb=gte.${minMemory}`
  }

  const response = await fetch(url, { headers })
  if (!response.ok) {
    throw new Error(`Failed to fetch: ${response.status}`)
  }
  return response.json()
}

export async function fetchBenchmarkRun(id) {
  const url = `${SUPABASE_URL}/rest/v1/benchmark_runs?id=eq.${id}&select=*`
  const response = await fetch(url, { headers })
  if (!response.ok) {
    throw new Error(`Failed to fetch: ${response.status}`)
  }
  const runs = await response.json()
  return runs[0] || null
}

export async function fetchStats() {
  // Get total count
  const countUrl = `${SUPABASE_URL}/rest/v1/benchmark_runs?select=id`
  const countResponse = await fetch(countUrl, {
    headers: { ...headers, 'Prefer': 'count=exact' }
  })

  const totalCount = parseInt(countResponse.headers.get('content-range')?.split('/')[1] || '0')

  // Get unique CPUs count (approximate by fetching distinct)
  const cpuUrl = `${SUPABASE_URL}/rest/v1/benchmark_runs?select=cpu_name`
  const cpuResponse = await fetch(cpuUrl, { headers })
  const cpuData = await cpuResponse.json()
  const uniqueCpus = new Set(cpuData.map(r => r.cpu_name)).size

  return {
    totalRuns: totalCount,
    uniqueCpus,
  }
}

// Admin password hash (SHA-256)
const ADMIN_PASSWORD_HASH = '92f71e72f53a12f3851825f1caf01587679bc8333ecf07c9df745b0c4386eec0'

async function hashPassword(password) {
  const encoder = new TextEncoder()
  const data = encoder.encode(password)
  const hashBuffer = await crypto.subtle.digest('SHA-256', data)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
}

export async function deleteBenchmarkRun(id, password) {
  // Verify password
  const passwordHash = await hashPassword(password)
  if (passwordHash !== ADMIN_PASSWORD_HASH) {
    throw new Error('Invalid admin password')
  }

  const url = `${SUPABASE_URL}/rest/v1/benchmark_runs?id=eq.${id}`
  const response = await fetch(url, {
    method: 'DELETE',
    headers,
  })

  if (!response.ok) {
    throw new Error(`Failed to delete: ${response.status}`)
  }
}

export async function updateBenchmarkRun(id, password, { display_name, description }) {
  // Verify password
  const passwordHash = await hashPassword(password)
  if (passwordHash !== ADMIN_PASSWORD_HASH) {
    throw new Error('Invalid admin password')
  }

  const url = `${SUPABASE_URL}/rest/v1/benchmark_runs?id=eq.${id}`
  const response = await fetch(url, {
    method: 'PATCH',
    headers,
    body: JSON.stringify({ display_name, description: description || null }),
  })

  if (!response.ok) {
    throw new Error(`Failed to update: ${response.status}`)
  }
}

// Fetch test statistics with histogram buckets for community comparison
export async function fetchTestStatistics() {
  const url = `${SUPABASE_URL}/rest/v1/rpc/get_test_statistics`
  const response = await fetch(url, {
    method: 'POST',
    headers,
    body: JSON.stringify({})
  })
  if (!response.ok) {
    throw new Error(`Failed to fetch statistics: ${response.status}`)
  }
  return response.json()
}

// Fetch percentile rank for a specific run
export async function fetchPercentileRank(runId) {
  const url = `${SUPABASE_URL}/rest/v1/rpc/get_percentile_rank`
  const response = await fetch(url, {
    method: 'POST',
    headers,
    body: JSON.stringify({ run_id: runId })
  })
  if (!response.ok) {
    throw new Error(`Failed to fetch percentile rank: ${response.status}`)
  }
  return response.json()
}

// GitHub Releases API for download links
export async function fetchLatestRelease() {
  try {
    // Replace with your actual GitHub repo
    const response = await fetch('https://api.github.com/repos/johanmcad/WorkBenchPro/releases/latest')
    if (!response.ok) return null
    return response.json()
  } catch {
    return null
  }
}
