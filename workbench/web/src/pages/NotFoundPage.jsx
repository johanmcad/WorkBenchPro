import { Link } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'

export default function NotFoundPage() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-32 text-center">
      <Helmet>
        <title>Page Not Found | WorkBench-Pro</title>
        <meta name="robots" content="noindex" />
      </Helmet>
      <h1 className="text-6xl font-bold mb-4">404</h1>
      <p className="text-xl text-wb-text-secondary mb-8">
        Page not found.
      </p>
      <Link to="/" className="btn-primary inline-flex items-center gap-2">
        Back to Home
      </Link>
    </div>
  )
}
