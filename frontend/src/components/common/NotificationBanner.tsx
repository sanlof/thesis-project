/**
 * src/components/common/NotificationBanner.tsx
 * ---------------------------------------------------------
 * Reusable Notification Banner
 *
 * Displays alert messages for success, error, or info states.
 * Used across Police and Hospital systems for user feedback.
 *
 * Supports:
 *  - Variant-based styling (success, error, warning, info)
 *  - Dismiss button
 *  - Optional auto-hide after timeout
 */

import React, { useEffect } from "react";
import { AlertCircle, CheckCircle2, Info, XCircle } from "lucide-react";

export type NotificationVariant = "success" | "error" | "warning" | "info";

interface NotificationBannerProps {
  /** Message text to display */
  message: string;
  /** Visual variant of the banner */
  variant?: NotificationVariant;
  /** Called when the banner is dismissed */
  onClose?: () => void;
  /** Automatically hide after timeout (ms) */
  autoHideDuration?: number;
}

export const NotificationBanner: React.FC<NotificationBannerProps> = ({
  message,
  variant = "info",
  onClose,
  autoHideDuration = 4000,
}) => {
  // Automatically hide after a set time
  useEffect(() => {
    if (autoHideDuration && onClose) {
      const timer = setTimeout(onClose, autoHideDuration);
      return () => clearTimeout(timer);
    }
  }, [autoHideDuration, onClose]);

  const getVariantStyles = () => {
    switch (variant) {
      case "success":
        return "bg-green-50 border-green-300 text-green-800";
      case "error":
        return "bg-red-50 border-red-300 text-red-800";
      case "warning":
        return "bg-yellow-50 border-yellow-300 text-yellow-800";
      case "info":
      default:
        return "bg-blue-50 border-blue-300 text-blue-800";
    }
  };

  const getVariantIcon = () => {
    switch (variant) {
      case "success":
        return <CheckCircle2 className="w-5 h-5 text-green-600" />;
      case "error":
        return <XCircle className="w-5 h-5 text-red-600" />;
      case "warning":
        return <AlertCircle className="w-5 h-5 text-yellow-600" />;
      case "info":
      default:
        return <Info className="w-5 h-5 text-blue-600" />;
    }
  };

  return (
    <div
      className={`flex items-center justify-between px-4 py-3 border rounded-lg shadow-sm ${getVariantStyles()} transition-opacity`}
      role="alert"
    >
      <div className="flex items-center space-x-3">
        {getVariantIcon()}
        <span className="text-sm font-medium">{message}</span>
      </div>
      {onClose && (
        <button
          onClick={onClose}
          className="text-sm font-semibold text-gray-500 hover:text-gray-700 focus:outline-none"
          aria-label="Close"
        >
          ✕
        </button>
      )}
    </div>
  );
};
