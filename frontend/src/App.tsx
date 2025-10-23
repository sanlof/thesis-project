/**
 * src/App.tsx
 * ---------------------------------------------------------
 * Application Entry Point
 *
 * Sets up client-side routing between major system dashboards:
 *  - Police System
 *  - Hospital System
 *  - Cross-System Dashboard
 *
 * Provides navigation, route guards, and default redirects.
 */

import React from "react";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
  Link,
} from "react-router-dom";
import PoliceDashboard from "./pages/PoliceDashboard";
import HospitalDashboard from "./pages/HospitalDashboard";
import CrossSystemDashboard from "./pages/CrossSystemDashboard";
import { Shield, Stethoscope, Link2, Home } from "lucide-react";

export default function App() {
  return (
    <Router>
      <div className="min-h-screen flex flex-col bg-gray-50">
        {/* Navigation Bar */}
        <nav className="bg-white border-b border-gray-200 shadow-sm px-6 py-3 flex justify-between items-center">
          <Link
            to="/"
            className="flex items-center space-x-2 text-gray-700 hover:text-gray-900"
          >
            <Home className="w-5 h-5" />
            <span className="font-semibold">Home</span>
          </Link>

          <div className="flex items-center space-x-6">
            <Link
              to="/police"
              className="flex items-center space-x-1 text-blue-700 hover:text-blue-900 transition-colors"
            >
              <Shield className="w-4 h-4" />
              <span>Police</span>
            </Link>
            <Link
              to="/hospital"
              className="flex items-center space-x-1 text-red-700 hover:text-red-900 transition-colors"
            >
              <Stethoscope className="w-4 h-4" />
              <span>Hospital</span>
            </Link>
            <Link
              to="/cross-system"
              className="flex items-center space-x-1 text-gray-700 hover:text-gray-900 transition-colors"
            >
              <Link2 className="w-4 h-4" />
              <span>Cross-System</span>
            </Link>
          </div>
        </nav>

        {/* Page Content */}
        <main className="flex-1">
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/police" element={<PoliceDashboard />} />
            <Route path="/hospital" element={<HospitalDashboard />} />
            <Route path="/cross-system" element={<CrossSystemDashboard />} />
            <Route path="*" element={<NotFoundPage />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

// -----------------------------------------------------------------------------
// Supporting Pages
// -----------------------------------------------------------------------------

/**
 * Home landing page providing quick navigation.
 */
const HomePage: React.FC = () => (
  <div className="flex flex-col items-center justify-center py-20 px-4 text-center">
    <h1 className="text-3xl font-bold text-gray-800 mb-4">
      Welcome to the Interlink Systems Portal
    </h1>
    <p className="text-gray-600 max-w-xl mb-8">
      This portal allows secure, real-time coordination between Police and
      Hospital systems. Choose a system below to begin managing records or
      reviewing cross-system data.
    </p>
    <div className="flex flex-wrap justify-center gap-4">
      <Link
        to="/police"
        className="flex items-center space-x-2 bg-blue-600 text-white px-5 py-3 rounded-xl hover:bg-blue-700 transition-colors"
      >
        <Shield className="w-5 h-5" />
        <span>Police System</span>
      </Link>
      <Link
        to="/hospital"
        className="flex items-center space-x-2 bg-red-600 text-white px-5 py-3 rounded-xl hover:bg-red-700 transition-colors"
      >
        <Stethoscope className="w-5 h-5" />
        <span>Hospital System</span>
      </Link>
      <Link
        to="/cross-system"
        className="flex items-center space-x-2 bg-gray-700 text-white px-5 py-3 rounded-xl hover:bg-gray-800 transition-colors"
      >
        <Link2 className="w-5 h-5" />
        <span>Cross-System Dashboard</span>
      </Link>
    </div>
  </div>
);

/**
 * Simple 404 fallback page.
 */
const NotFoundPage: React.FC = () => (
  <div className="flex flex-col items-center justify-center h-[80vh] text-center space-y-3">
    <h1 className="text-4xl font-bold text-gray-800">404</h1>
    <p className="text-gray-600">The page you’re looking for doesn’t exist.</p>
    <Link
      to="/"
      className="mt-3 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors"
    >
      Go Home
    </Link>
  </div>
);
