// Supabase API client for fetching benchmark data
const SUPABASE_URL = 'https://wqutewgfxtucshqwzecj.supabase.co'
const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndxdXRld2dmeHR1Y3NocXd6ZWNqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjkzNzQ4MDgsImV4cCI6MjA4NDk1MDgwOH0.Wx2Yd5ONcorGqLTeNsMgeOrkBQBwN8Kjag_rBXX60O8'

const headers = {
  'apikey': SUPABASE_KEY,
  'Authorization': `Bearer ${SUPABASE_KEY}`,
  'Content-Type': 'application/json',
}

export async function fetchBenchmarkRuns({ cpuFilter, osFilter, minMemory, limit = 50 } = {}) {
  let url = `${SUPABASE_URL}/rest/v1/benchmark_runs?select=id,display_name,cpu_name,cpu_cores,cpu_threads,memory_gb,os_name,storage_type,uploaded_at&order=uploaded_at.desc&limit=${limit}`

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
