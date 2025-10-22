/**
 * src/components/layout/AppLayout.tsx
 * ---------------------------------------------------------
 * Application Layout Wrapper
 *
 * Provides a consistent layout structure for all pages,
 * including the Header, Footer, and a responsive content area.
 *
 * Supports system-specific theming (Police, Hospital, or Cross-System)
 * and ensures proper spacing and alignment across viewports.
 */

import React, { ReactNode } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";

interface AppLayoutProps {
  title: string;
  subtitle?: string;
  system?: "police" | "hospital";
  version?: string;
  environment?: string;
  children: ReactNode;
}

export const AppLayout: React.FC<AppLayoutProps> = ({
  title,
  subtitle,
  system,
  version,
  environment,
  children,
}) => {
  // System-specific background accents
  const getBackground = () => {
    switch (system) {
      case "police":
        return "bg-blue-50";
      case "hospital":
        return "bg-red-50";
      default:
        return "bg-gray-50";
    }
  };

  return (
    <div
      className={`flex flex-col min-h-screen ${getBackground()} transition-colors`}
    >
      {/* Header */}
      <Header title={title} subtitle={subtitle} system={system} />

      {/* Main Content Area */}
      <main className="flex-1 p-6 overflow-y-auto">
        <div className="max-w-7xl mx-auto">{children}</div>
      </main>

      {/* Footer */}
      <Footer version={version} environment={environment} />
    </div>
  );
};
