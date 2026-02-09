const SUPABASE_URL = 'https://wqutewgfxtucshqwzecj.supabase.co'
const SUPABASE_KEY = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6IndxdXRld2dmeHR1Y3NocXd6ZWNqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjkzNzQ4MDgsImV4cCI6MjA4NDk1MDgwOH0.Wx2Yd5ONcorGqLTeNsMgeOrkBQBwN8Kjag_rBXX60O8'

export default async function handler(req, res) {
  const BASE = 'https://www.workbench-pro.com'

  // Static pages
  const staticPages = [
    { loc: '/', changefreq: 'weekly', priority: '1.0' },
    { loc: '/results', changefreq: 'daily', priority: '0.9' },
    { loc: '/compare', changefreq: 'weekly', priority: '0.7' },
    { loc: '/about', changefreq: 'monthly', priority: '0.6' },
    { loc: '/faq', changefreq: 'monthly', priority: '0.6' },
    { loc: '/changelog', changefreq: 'monthly', priority: '0.5' },
    { loc: '/privacy', changefreq: 'yearly', priority: '0.3' },
    { loc: '/terms', changefreq: 'yearly', priority: '0.3' },
  ]

  // Fetch all benchmark run IDs from Supabase
  let runs = []
  try {
    const response = await fetch(
      `${SUPABASE_URL}/rest/v1/benchmark_runs?select=id,uploaded_at&order=uploaded_at.desc&limit=5000`,
      {
        headers: {
          'apikey': SUPABASE_KEY,
          'Authorization': `Bearer ${SUPABASE_KEY}`,
        },
      }
    )
    if (response.ok) {
      runs = await response.json()
    }
  } catch (e) {
    // If Supabase is down, still return static pages
  }

  const today = new Date().toISOString().split('T')[0]

  let xml = '<?xml version="1.0" encoding="UTF-8"?>\n'
  xml += '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'

  // Static pages
  for (const page of staticPages) {
    xml += '  <url>\n'
    xml += `    <loc>${BASE}${page.loc}</loc>\n`
    xml += `    <lastmod>${today}</lastmod>\n`
    xml += `    <changefreq>${page.changefreq}</changefreq>\n`
    xml += `    <priority>${page.priority}</priority>\n`
    xml += '  </url>\n'
  }

  // Individual benchmark result pages
  for (const run of runs) {
    const lastmod = run.uploaded_at ? run.uploaded_at.split('T')[0] : today
    xml += '  <url>\n'
    xml += `    <loc>${BASE}/results/${run.id}</loc>\n`
    xml += `    <lastmod>${lastmod}</lastmod>\n`
    xml += `    <changefreq>monthly</changefreq>\n`
    xml += `    <priority>0.6</priority>\n`
    xml += '  </url>\n'
  }

  xml += '</urlset>\n'

  res.setHeader('Content-Type', 'application/xml')
  res.setHeader('Cache-Control', 's-maxage=3600, stale-while-revalidate=86400')
  res.status(200).send(xml)
}
