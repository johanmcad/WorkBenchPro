import { useState } from 'react'
import { Github, Mail } from 'lucide-react'
import ContactForm from './ContactForm'

export default function Footer() {
  const [isContactFormOpen, setIsContactFormOpen] = useState(false)

  return (
    <>
      <footer className="bg-wb-bg-card border-t border-wb-border py-8 mt-auto">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 bg-wb-accent rounded-lg flex items-center justify-center">
                <span className="text-white font-bold">W</span>
              </div>
              <div className="flex flex-col">
                <span className="text-wb-text-secondary">
                  WorkBench-Pro - Real-World Workstation Benchmark
                </span>
                <span className="text-wb-text-secondary text-xs">
                  © 2025 P&T AB · Johan Moreau
                </span>
              </div>
            </div>

            <div className="flex items-center gap-6">
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
                <span className="text-sm">Contact Us</span>
              </button>
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
