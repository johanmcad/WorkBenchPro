import { Link, useLocation } from 'react-router-dom'
import { Menu, X, Gauge, BarChart3, Download } from 'lucide-react'
import { useState } from 'react'

export default function Navbar() {
  const location = useLocation()
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)

  const links = [
    { path: '/', label: 'Home', icon: Gauge },
    { path: '/results', label: 'Results', icon: BarChart3 },
  ]

  const isActive = (path) => location.pathname === path

  return (
    <nav className="bg-wb-bg-card border-b border-wb-border sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-3">
            <div className="w-9 h-9 bg-wb-accent rounded-lg flex items-center justify-center">
              <span className="text-white font-bold text-lg">W</span>
            </div>
            <span className="font-semibold text-xl hidden sm:block">WorkBench-Pro</span>
          </Link>

          {/* Desktop Nav */}
          <div className="hidden md:flex items-center gap-1">
            {links.map(({ path, label, icon: Icon }) => (
              <Link
                key={path}
                to={path}
                className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${
                  isActive(path)
                    ? 'bg-wb-accent text-white'
                    : 'text-wb-text-secondary hover:text-white hover:bg-wb-bg-secondary'
                }`}
              >
                <Icon size={18} />
                <span>{label}</span>
              </Link>
            ))}
            <a
              href="#download"
              className="flex items-center gap-2 ml-4 btn-primary"
            >
              <Download size={18} />
              <span>Download</span>
            </a>
          </div>

          {/* Mobile menu button */}
          <div className="md:hidden flex items-center">
            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="p-2 rounded-lg text-wb-text-secondary hover:text-white hover:bg-wb-bg-secondary"
            >
              {mobileMenuOpen ? <X size={24} /> : <Menu size={24} />}
            </button>
          </div>
        </div>
      </div>

      {/* Mobile menu */}
      {mobileMenuOpen && (
        <div className="md:hidden border-t border-wb-border">
          <div className="px-4 py-3 space-y-2">
            {links.map(({ path, label, icon: Icon }) => (
              <Link
                key={path}
                to={path}
                onClick={() => setMobileMenuOpen(false)}
                className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
                  isActive(path)
                    ? 'bg-wb-accent text-white'
                    : 'text-wb-text-secondary hover:text-white hover:bg-wb-bg-secondary'
                }`}
              >
                <Icon size={20} />
                <span>{label}</span>
              </Link>
            ))}
            <a
              href="#download"
              onClick={() => setMobileMenuOpen(false)}
              className="flex items-center gap-3 px-4 py-3 btn-primary w-full justify-center"
            >
              <Download size={20} />
              <span>Download</span>
            </a>
          </div>
        </div>
      )}
    </nav>
  )
}
