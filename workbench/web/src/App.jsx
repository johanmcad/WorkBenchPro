import { Routes, Route } from 'react-router-dom'
import Navbar from './components/Navbar'
import Footer from './components/Footer'
import HomePage from './pages/HomePage'
import ResultsPage from './pages/ResultsPage'
import ComparePage from './pages/ComparePage'

function App() {
  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />
      <main className="flex-1">
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/results" element={<ResultsPage />} />
          <Route path="/compare" element={<ComparePage />} />
        </Routes>
      </main>
      <Footer />
    </div>
  )
}

export default App
