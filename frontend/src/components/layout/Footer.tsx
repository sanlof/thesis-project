/**
 * src/components/layout/Footer.tsx
 * ---------------------------------------------------------
 * Application Footer
 *
 * Displays environment info, version, and developer attribution.
 * Stays consistent across all pages.
 */

import React from "react";

interface FooterProps {
  version?: string;
  environment?: string;
}

export const Footer: React.FC<FooterProps> = ({
  version = "v1.0.0",
  environment = import.meta.env.MODE,
}) => {
  const year = new Date().getFullYear();

  return (
    <footer className="w-full bg-gray-100 border-t border-gray-200 text-gray-600 text-sm py-3 px-6 flex justify-between items-center">
      <div className="flex items-center space-x-2">
        <span className="font-medium">Interlink Systems</span>
        <span>© {year}</span>
      </div>

      <div className="text-xs text-gray-500">
        <span>{environment.toUpperCase()}</span>
        <span className="mx-1">•</span>
        <span>{version}</span>
      </div>
    </footer>
  );
};
