import { Helmet } from 'react-helmet-async'

export default function TermsPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <Helmet>
        <title>Terms of Use | WorkBench-Pro</title>
        <meta name="description" content="WorkBench-Pro terms of use. Understand the terms for using the benchmark tool and community features." />
        <link rel="canonical" href="https://www.workbench-pro.com/terms" />
        <meta property="og:url" content="https://www.workbench-pro.com/terms" />
        <meta property="og:title" content="Terms of Use | WorkBench-Pro" />
        <meta property="og:description" content="WorkBench-Pro terms of use. Understand the terms for using the benchmark tool and community features." />
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "Terms of Use" }
          ]
        })}</script>
      </Helmet>

      <h1 className="text-4xl font-bold mb-8">Terms of Use</h1>
      <p className="text-wb-text-secondary text-sm mb-8">Last updated: February 9, 2026</p>

      <div className="space-y-8 text-wb-text-secondary leading-relaxed">
        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Acceptance of Terms</h2>
          <p>
            By downloading, installing, or using WorkBench-Pro, you agree to these terms. If you do not agree,
            do not use the software.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Use of the Software</h2>
          <p>
            WorkBench-Pro is provided free of charge for personal and commercial use. You may run the benchmark
            on any systems you are authorized to use. The software is provided "as is" without warranty of any kind.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Community Features</h2>
          <p>When uploading benchmark results to the community database, you agree to:</p>
          <ul className="list-disc list-inside mt-2 space-y-1">
            <li>Only upload results from benchmarks you actually ran</li>
            <li>Not manipulate or falsify benchmark results</li>
            <li>Not upload offensive or inappropriate display names or descriptions</li>
            <li>Not abuse the upload system (e.g., spam, excessive uploads)</li>
          </ul>
          <p className="mt-3">
            We reserve the right to remove any uploaded results that violate these terms.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Disclaimer</h2>
          <p>
            Benchmark results are influenced by many factors including background processes, system configuration,
            thermal conditions, and driver versions. Results should be used for general comparison purposes and
            may not be perfectly reproducible. WorkBench-Pro and its creators are not responsible for any decisions
            made based on benchmark results.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Open Source</h2>
          <p>
            WorkBench-Pro is open source software. The source code is available on{' '}
            <a
              href="https://github.com/johanmcad/WorkBenchPro"
              className="text-wb-accent-light hover:underline"
              target="_blank"
              rel="noopener noreferrer"
            >
              GitHub
            </a>
            . Contributions are welcome under the project's license terms.
          </p>
        </section>

        <section>
          <h2 className="text-xl font-semibold text-wb-text-primary mb-3">Changes to Terms</h2>
          <p>
            We may update these terms from time to time. Changes will be posted on this page with an
            updated date. Continued use of the software after changes constitutes acceptance of the new terms.
          </p>
        </section>
      </div>
    </div>
  )
}
