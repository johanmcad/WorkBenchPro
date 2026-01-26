/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Match the desktop app theme
        'wb-bg': '#1a1a2e',
        'wb-bg-card': '#16213e',
        'wb-bg-secondary': '#0f3460',
        'wb-accent': '#1a5a8a',
        'wb-accent-light': '#2980b9',
        'wb-text': '#ecf0f1',
        'wb-text-secondary': '#95a5a6',
        'wb-success': '#27ae60',
        'wb-error': '#e74c3c',
        'wb-border': '#2c3e50',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
}
