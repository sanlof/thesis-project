/**
 * src/components/layout/Header.tsx
 * ---------------------------------------------------------
 * Application Header
 *
 * Displays the app title, system indicator, and optional
 * navigation actions. Used across all major pages.
 */

import React from "react";
import { Shield, Stethoscope } from "lucide-react";

interface HeaderProps {
  title: string;
  system?: "police" | "hospital";
  subtitle?: string;
}

export const Header: React.FC<HeaderProps> = ({ title, system, subtitle }) => {
  const getSystemIcon = () => {
    switch (system) {
      case "police":
        return <Shield className="w-6 h-6 text-blue-600" />;
      case "hospital":
        return <Stethoscope className="w-6 h-6 text-red-600" />;
      default:
        return null;
    }
  };

  const getSystemColor = () => {
    switch (system) {
      case "police":
        return "text-blue-700";
      case "hospital":
        return "text-red-700";
      default:
        return "text-gray-700";
    }
  };

  return (
    <header className="flex items-center justify-between bg-gray-50 border-b border-gray-200 px-6 py-4 shadow-sm">
      <div className="flex items-center space-x-3">
        {getSystemIcon()}
        <div>
          <h1 className={`text-xl font-semibold ${getSystemColor()}`}>
            {title}
          </h1>
          {subtitle && (
            <p className="text-sm text-gray-500 leading-tight">{subtitle}</p>
          )}
        </div>
      </div>

      <div className="text-xs text-gray-500 font-mono">
        {system === "police" && "POLICE SYSTEM"}
        {system === "hospital" && "HOSPITAL SYSTEM"}
        {!system && "CROSS-SYSTEM"}
      </div>
    </header>
  );
};
