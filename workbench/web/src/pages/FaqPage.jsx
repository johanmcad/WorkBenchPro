import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { ChevronDown } from 'lucide-react'

const faqs = [
  {
    question: 'What does WorkBench-Pro measure?',
    answer: 'WorkBench-Pro measures real-world workstation performance across four categories: project operations (file I/O, directory traversal), build performance (CPU single/multi-thread, compression), responsiveness (process spawn, storage latency, thread wake), and Windows system tools (PowerShell, registry, services). Every test simulates an actual workflow rather than a synthetic loop.',
  },
  {
    question: 'Is WorkBench-Pro free?',
    answer: 'Yes, WorkBench-Pro is completely free and open source. The full source code is available on GitHub. There are no paid tiers, ads, or data collection beyond the benchmark scores you choose to share.',
  },
  {
    question: 'What operating systems are supported?',
    answer: 'WorkBench-Pro currently supports Windows 10 and Windows 11 (x64). Some tests use Windows-specific APIs like PowerShell, registry queries, and Windows services, so it is designed specifically for the Windows platform.',
  },
  {
    question: 'Do I need to install anything?',
    answer: 'No. WorkBench-Pro is a portable executable — just download the .exe and run it. No installation, no admin privileges required, and no files are saved to your computer.',
  },
  {
    question: 'How does the scoring system work?',
    answer: 'Each of your test results is compared against the community database using percentile ranks. Your score (e.g., 20/25) shows how many of your tests performed above the community median. This gives a practical overview of where your machine stands without reducing everything to a single abstract number.',
  },
  {
    question: 'Will WorkBench-Pro affect my system or files?',
    answer: 'No. All benchmark operations use temporary files that are cleaned up after each test. The tool does not modify your system, write to the registry, or install any services. It only reads system information (CPU, memory, OS) to include in your benchmark results.',
  },
  {
    question: 'How do I share or compare my results?',
    answer: 'After running a benchmark, you can upload your results to the community database. Each result gets a unique URL you can share. You can also use the Compare page to view two results side-by-side, or select up to three results on the Results page to overlay them on the community charts.',
  },
  {
    question: 'Why are my antivirus or Windows Defender slowing down the benchmark?',
    answer: 'Real-time antivirus scanning can significantly impact file I/O benchmarks because the scanner inspects every file operation. WorkBench-Pro includes a "Safe Mode" that uses operations less likely to trigger scanning. For the most accurate results, you can temporarily exclude the benchmark\'s temp directory from real-time scanning.',
  },
  {
    question: 'How is this different from Cinebench, Geekbench, or PassMark?',
    answer: 'Traditional benchmarks focus on peak theoretical performance — tight CPU loops, sequential storage throughput, and synthetic graphics tests. WorkBench-Pro measures the mixed, real-world workloads that determine how responsive your system feels during actual work: file enumeration patterns like git status, random I/O like opening source files, process spawn latency, and actual tool execution.',
  },
]

function FaqItem({ question, answer }) {
  const [open, setOpen] = useState(false)

  return (
    <div className="border border-wb-border rounded-lg overflow-hidden">
      <button
        onClick={() => setOpen(!open)}
        className="w-full text-left p-4 flex items-center justify-between gap-4 hover:bg-wb-bg-card/50 transition-colors"
      >
        <span className="font-medium">{question}</span>
        <ChevronDown
          size={20}
          className={`text-wb-text-secondary shrink-0 transition-transform duration-200 ${open ? 'rotate-180' : ''}`}
        />
      </button>
      {open && (
        <div className="px-4 pb-4 text-wb-text-secondary text-sm leading-relaxed">
          {answer}
        </div>
      )}
    </div>
  )
}

export default function FaqPage() {
  const faqSchema = {
    "@context": "https://schema.org",
    "@type": "FAQPage",
    "mainEntity": faqs.map(faq => ({
      "@type": "Question",
      "name": faq.question,
      "acceptedAnswer": {
        "@type": "Answer",
        "text": faq.answer,
      },
    })),
  }

  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
      <Helmet>
        <title>FAQ - Frequently Asked Questions | WorkBench-Pro</title>
        <meta name="description" content="Common questions about WorkBench-Pro: what it measures, how scoring works, system requirements, privacy, and how it compares to other benchmarks." />
        <link rel="canonical" href="https://www.workbench-pro.com/faq" />
        <meta property="og:url" content="https://www.workbench-pro.com/faq" />
        <meta property="og:title" content="FAQ - Frequently Asked Questions | WorkBench-Pro" />
        <meta property="og:description" content="Common questions about WorkBench-Pro: what it measures, how scoring works, and how it compares to other benchmarks." />
        <meta property="twitter:url" content="https://www.workbench-pro.com/faq" />
        <meta property="twitter:title" content="FAQ - Frequently Asked Questions | WorkBench-Pro" />
        <meta property="twitter:description" content="Common questions about WorkBench-Pro: what it measures, how scoring works, and how it compares to other benchmarks." />
        <script type="application/ld+json">{JSON.stringify(faqSchema)}</script>
        <script type="application/ld+json">{JSON.stringify({
          "@context": "https://schema.org",
          "@type": "BreadcrumbList",
          "itemListElement": [
            { "@type": "ListItem", "position": 1, "name": "Home", "item": "https://www.workbench-pro.com/" },
            { "@type": "ListItem", "position": 2, "name": "FAQ" }
          ]
        })}</script>
      </Helmet>

      <h1 className="text-4xl font-bold mb-8">Frequently Asked Questions</h1>

      <div className="space-y-3">
        {faqs.map((faq, i) => (
          <FaqItem key={i} question={faq.question} answer={faq.answer} />
        ))}
      </div>

      <div className="mt-12 p-6 rounded-lg border border-wb-border bg-wb-bg-card/50 text-center">
        <p className="text-wb-text-secondary mb-4">
          Still have questions?
        </p>
        <div className="flex gap-4 justify-center">
          <Link to="/" className="btn-primary">
            Back to Home
          </Link>
          <a
            href="https://github.com/johanmcad/WorkBenchPro/issues"
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary"
          >
            Ask on GitHub
          </a>
        </div>
      </div>
    </div>
  )
}
