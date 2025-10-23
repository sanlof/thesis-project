/**
 * src/main.tsx
 * ---------------------------------------------------------
 * React Entry Point
 *
 * Initializes the React app, mounts it to the DOM,
 * and loads global styles and environment configurations.
 */

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css"; // TailwindCSS base styles

// Root application mount point
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
