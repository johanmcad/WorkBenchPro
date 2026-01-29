import { useState } from 'react'
import { X, Send, CheckCircle, AlertCircle } from 'lucide-react'
import { submitContactForm } from '../api'

export default function ContactForm({ isOpen, onClose }) {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    message: '',
  })
  const [status, setStatus] = useState('') // 'sending', 'success', 'error'
  const [errorMessage, setErrorMessage] = useState('')

  const handleSubmit = async (e) => {
    e.preventDefault()
    setStatus('sending')
    setErrorMessage('')

    try {
      await submitContactForm({
        name: formData.name,
        email: formData.email,
        message: formData.message,
      })

      setStatus('success')
      setFormData({ name: '', email: '', message: '' })
      setTimeout(() => {
        onClose()
        setStatus('')
      }, 2000)
    } catch (error) {
      setStatus('error')
      setErrorMessage('Something went wrong. Try again in a bit.')
      console.error('Form submission error:', error)
    }
  }

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    })
  }

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
      <div className="relative w-full max-w-lg bg-wb-bg-card border border-wb-border rounded-2xl shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-wb-border">
          <h2 className="text-2xl font-bold">Contact Us</h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-wb-bg-secondary rounded-lg transition-colors"
            aria-label="Close"
          >
            <X size={20} />
          </button>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          {/* Success Message */}
          {status === 'success' && (
            <div className="flex items-center gap-3 p-4 bg-wb-success/20 border border-wb-success/50 rounded-lg">
              <CheckCircle size={20} className="text-wb-success" />
              <p className="text-sm text-wb-success">Got it! We'll get back to you soon.</p>
            </div>
          )}

          {/* Error Message */}
          {status === 'error' && (
            <div className="flex items-center gap-3 p-4 bg-wb-error/20 border border-wb-error/50 rounded-lg">
              <AlertCircle size={20} className="text-wb-error" />
              <p className="text-sm text-wb-error">{errorMessage}</p>
            </div>
          )}

          {/* Name Field */}
          <div>
            <label htmlFor="name" className="block text-sm font-medium mb-2">
              Name <span className="text-wb-error">*</span>
            </label>
            <input
              type="text"
              id="name"
              name="name"
              value={formData.name}
              onChange={handleChange}
              required
              className="input w-full"
              placeholder="Your name"
              disabled={status === 'sending'}
            />
          </div>

          {/* Email Field */}
          <div>
            <label htmlFor="email" className="block text-sm font-medium mb-2">
              Email <span className="text-wb-error">*</span>
            </label>
            <input
              type="email"
              id="email"
              name="email"
              value={formData.email}
              onChange={handleChange}
              required
              className="input w-full"
              placeholder="your@email.com"
              disabled={status === 'sending'}
            />
          </div>

          {/* Message Field */}
          <div>
            <label htmlFor="message" className="block text-sm font-medium mb-2">
              Message <span className="text-wb-error">*</span>
            </label>
            <textarea
              id="message"
              name="message"
              value={formData.message}
              onChange={handleChange}
              required
              rows={5}
              className="input w-full resize-none"
              placeholder="What's up?"
              disabled={status === 'sending'}
            />
          </div>

          {/* Submit Button */}
          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="btn-secondary flex-1"
              disabled={status === 'sending'}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="btn-primary flex-1 flex items-center justify-center gap-2"
              disabled={status === 'sending'}
            >
              {status === 'sending' ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Sending...
                </>
              ) : (
                <>
                  <Send size={18} />
                  Send Message
                </>
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
