/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Police System Colors (Blue Tones)
        police: {
          50: '#eff6ff',   // Lightest blue
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',  // Primary police blue
          600: '#2563eb',
          700: '#1d4ed8',  // Darker police blue
          800: '#1e40af',
          900: '#1e3a8a',  // Darkest blue
        },
        
        // Hospital System Colors (Red/Medical Tones)
        hospital: {
          50: '#fef2f2',   // Lightest red
          100: '#fee2e2',
          200: '#fecaca',
          300: '#fca5a5',
          400: '#f87171',
          500: '#ef4444',  // Primary hospital red
          600: '#dc2626',
          700: '#b91c1c',  // Darker hospital red
          800: '#991b1b',
          900: '#7f1d1d',  // Darkest red
        },
        
        // Shared/Neutral Colors
        neutral: {
          50: '#fafafa',
          100: '#f5f5f5',
          200: '#e5e5e5',
          300: '#d4d4d4',
          400: '#a3a3a3',
          500: '#737373',
          600: '#525252',
          700: '#404040',
          800: '#262626',
          900: '#171717',
        },
        
        // Status Colors
        success: '#10b981',  // Green for success states
        warning: '#f59e0b',  // Amber for warnings
        error: '#ef4444',    // Red for errors
        info: '#3b82f6',     // Blue for info
        
        // Flag Status
        flagged: '#ef4444',      // Red for flagged status
        unflagged: '#6b7280',    // Gray for unflagged status
      },
      
      // Custom spacing for consistent layout
      spacing: {
        '128': '32rem',
        '144': '36rem',
      },
      
      // Custom border radius
      borderRadius: {
        'xl': '1rem',
        '2xl': '1.5rem',
      },
      
      // Custom box shadows
      boxShadow: {
        'police': '0 4px 6px -1px rgba(59, 130, 246, 0.1), 0 2px 4px -1px rgba(59, 130, 246, 0.06)',
        'hospital': '0 4px 6px -1px rgba(239, 68, 68, 0.1), 0 2px 4px -1px rgba(239, 68, 68, 0.06)',
        'card': '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
      },
      
      // Custom animations
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'bounce-slow': 'bounce 2s infinite',
        'spin-slow': 'spin 3s linear infinite',
      },
      
      // Custom font families (optional - using system fonts for now)
      fontFamily: {
        sans: [
          'Inter var',
          'ui-sans-serif',
          'system-ui',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'Roboto',
          'Helvetica Neue',
          'Arial',
          'sans-serif',
        ],
        mono: [
          'ui-monospace',
          'SFMono-Regular',
          'Menlo',
          'Monaco',
          'Consolas',
          'Liberation Mono',
          'Courier New',
          'monospace',
        ],
      },
    },
  },
  plugins: [],
}
