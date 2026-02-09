import { useState } from 'react'
import { Link } from 'react-router-dom'
import { Github, Mail } from 'lucide-react'
import ContactForm from './ContactForm'

export default function Footer() {
  const [isContactFormOpen, setIsContactFormOpen] = useState(false)

  return (
    <>
      <footer className="bg-wb-bg-card border-t border-wb-border py-8 mt-auto">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-6">
            {/* Brand */}
            <div className="flex items-start gap-3">
              <div className="w-8 h-8 bg-wb-accent rounded-lg flex items-center justify-center shrink-0">
                <span className="text-white font-bold">W</span>
              </div>
              <div className="flex flex-col">
                <span className="text-wb-text-secondary">
                  WorkBench-Pro
                </span>
                <span className="text-wb-text-secondary text-xs">
                  Real-World Workstation Benchmark
                </span>
              </div>
            </div>

            {/* Navigation Links */}
            <div className="flex flex-wrap gap-x-6 gap-y-2 text-sm">
              <Link to="/results" className="text-wb-text-secondary hover:text-white transition-colors">Results</Link>
              <Link to="/compare" className="text-wb-text-secondary hover:text-white transition-colors">Compare</Link>
              <Link to="/about" className="text-wb-text-secondary hover:text-white transition-colors">About</Link>
              <Link to="/faq" className="text-wb-text-secondary hover:text-white transition-colors">FAQ</Link>
              <Link to="/changelog" className="text-wb-text-secondary hover:text-white transition-colors">Changelog</Link>
            </div>

            {/* External Links */}
            <div className="flex items-center gap-6 md:justify-end">
              <a
                href="https://github.com/johanmcad/WorkBenchPro"
                target="_blank"
                rel="noopener noreferrer"
                className="text-wb-text-secondary hover:text-white transition-colors"
              >
                <Github size={20} />
              </a>
              <button
                onClick={() => setIsContactFormOpen(true)}
                className="text-wb-text-secondary hover:text-white transition-colors flex items-center gap-2"
              >
                <Mail size={18} />
                <span className="text-sm">Contact</span>
              </button>
            </div>
          </div>

          <div className="border-t border-wb-border pt-4 flex flex-col sm:flex-row justify-between items-center gap-2 text-xs text-wb-text-secondary">
            <span>© 2025 P&T AB · Johan Moreau</span>
            <div className="flex gap-4">
              <Link to="/privacy" className="hover:text-white transition-colors">Privacy</Link>
              <Link to="/terms" className="hover:text-white transition-colors">Terms</Link>
            </div>
          </div>
        </div>
      </footer>

      <ContactForm
        isOpen={isContactFormOpen}
        onClose={() => setIsContactFormOpen(false)}
      />
    </>
  )
}
