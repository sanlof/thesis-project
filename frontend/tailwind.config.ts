/**
 * tailwind.config.ts
 * ---------------------------------------------------------
 * Tailwind Configuration
 *
 * Extends the default theme with custom color palettes
 * and font settings to support both Police and Hospital
 * system branding.
 */

import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        police: {
          light: "#ebf4ff",
          DEFAULT: "#2563eb", // blue-600
          dark: "#1e3a8a",
        },
        hospital: {
          light: "#fee2e2",
          DEFAULT: "#dc2626", // red-600
          dark: "#991b1b",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "Avenir", "Helvetica", "Arial", "sans-serif"],
      },
      borderRadius: {
        xl: "1rem",
        "2xl": "1.5rem",
      },
      boxShadow: {
        soft: "0 2px 8px rgba(0, 0, 0, 0.05)",
      },
    },
  },
  plugins: [],
} satisfies Config;
