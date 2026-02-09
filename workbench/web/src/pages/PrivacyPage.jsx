import { Helmet } from 'react-helmet-async'

export default function PrivacyPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <Helmet>
        <title>Privacy Policy | WorkBench-Pro</title>
        <meta name="description" content="WorkBench-Pro privacy policy. Learn what data is collected when you run benchmarks and how your information is handled." />
        <link rel="canonical" href="https://www.workbench-pro.com/privacy" />
        <meta property="og:url" content="https://www.workbench-pro.com/privacy" />
        <meta property="og:title" content="Privacy Policy | WorkBench-Pro" />
        <meta property="og:description" content="WorkBench-Pro privacy policy. Learn what data is collected and how your information is handled." />
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "Privacy Policy" }
          ]
        })}</script>
      </Helmet>

      <h1 className="text-4xl font-bold mb-8">Privacy Policy</h1>
      <p className="text-wb-text-secondary text-sm mb-8">Last updated: February 9, 2026</p>

      <div className="space-y-8 text-wb-text-secondary leading-relaxed">
        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Overview</h2>
          <p>
            WorkBench-Pro is designed with privacy in mind. We collect the minimum amount of information
            necessary to provide benchmark comparison features. No personal data is collected, and all data
            sharing is optional.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Desktop Application</h2>
          <p>The WorkBench-Pro desktop application:</p>
          <ul className="list-disc list-inside mt-2 space-y-1">
            <li>Does not save any files to your computer</li>
            <li>Does not install any services, drivers, or registry entries</li>
            <li>Does not collect or transmit data without your explicit action</li>
            <li>Does not require administrator privileges</li>
            <li>Uses only temporary files during benchmarks, which are cleaned up immediately after</li>
          </ul>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Data Collected When You Upload Results</h2>
          <p>
            If you choose to upload your benchmark results to the community database, the following information
            is sent:
          </p>
          <ul className="list-disc list-inside mt-2 space-y-1">
            <li>CPU model name, core count, and thread count</li>
            <li>Total system memory (RAM)</li>
            <li>Operating system name and version</li>
            <li>Storage type (SSD/HDD) if detected</li>
            <li>Benchmark test results (scores and timings)</li>
            <li>Display name (you choose this — it can be anything)</li>
          </ul>
          <p className="mt-3">
            We do <strong className="text-wb-text-primary">not</strong> collect: your IP address (beyond what's
            inherent in HTTP requests), machine hostname, username, file paths, serial numbers, MAC addresses,
            or any other personally identifiable information.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Website</h2>
          <p>
            The WorkBench-Pro website (workbench-pro.com) does not use cookies, tracking pixels, or analytics
            services. The site is hosted on Vercel, which may collect standard server logs (IP address,
            user agent, timestamps) as part of normal web hosting operations.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Data Storage</h2>
          <p>
            Uploaded benchmark results are stored in a Supabase database. Results are publicly visible on
            the website. You can request deletion of your uploaded results by using the delete function on
            the results page.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Contact</h2>
          <p>
            If you have questions about this privacy policy or want to request data deletion, you can reach
            us through the contact form on the website or by opening an issue on{' '}
            <a
              href="https://github.com/johanmcad/WorkBenchPro"
              className="text-wb-accent-light hover:underline"
              target="_blank"
              rel="noopener noreferrer"
            >
              GitHub
            </a>.
          </p>
        </section>
      </div>
    </div>
  )
}
