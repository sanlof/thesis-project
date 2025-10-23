/**
 * src/components/common/DataTable.tsx
 * ---------------------------------------------------------
 * Reusable Data Table Component
 *
 * Provides a generic, type-safe table component for displaying
 * structured data across the Police and Hospital systems.
 *
 * Features:
 *  - Customizable columns
 *  - Optional actions per row
 *  - Empty and loading states
 *  - Tailwind-styled layout
 */

import React from "react";
import { Loader2 } from "lucide-react";

export interface Column<T> {
  /** Header label for the column */
  header: string;
  /** Function to render the cell content */
  accessor: (row: T) => React.ReactNode;
  /** Optional column width or alignment classes */
  className?: string;
}

interface DataTableProps<T> {
  /** Data rows to display */
  data: T[];
  /** Column definitions */
  columns: Column<T>[];
  /** Optional row key generator (defaults to index) */
  rowKey?: (row: T, index: number) => string | number;
  /** Optional action buttons (e.g., Edit/Delete) */
  actions?: (row: T) => React.ReactNode;
  /** Loading indicator */
  loading?: boolean;
  /** Message when no data */
  emptyMessage?: string;
  /** Max height for scrollable table body */
  maxHeight?: string;
}

export function DataTable<T>({
  data,
  columns,
  rowKey,
  actions,
  loading = false,
  emptyMessage = "No data available.",
  maxHeight = "500px",
}: DataTableProps<T>) {
  return (
    <div className="w-full bg-white border border-gray-200 rounded-2xl shadow-sm overflow-hidden">
      <div className="overflow-x-auto">
        <table className="min-w-full text-sm text-left text-gray-700">
          <thead className="bg-gray-100 text-gray-600 uppercase text-xs font-semibold">
            <tr>
              {columns.map((col, idx) => (
                <th
                  key={idx}
                  className={`px-4 py-3 whitespace-nowrap ${
                    col.className || ""
                  }`}
                >
                  {col.header}
                </th>
              ))}
              {actions && <th className="px-4 py-3 text-right">Actions</th>}
            </tr>
          </thead>

          <tbody
            className="divide-y divide-gray-100"
            style={{ maxHeight, overflowY: "auto" }}
          >
            {loading ? (
              <tr>
                <td
                  colSpan={columns.length + (actions ? 1 : 0)}
                  className="py-8 text-center"
                >
                  <div className="flex justify-center items-center space-x-2 text-gray-500">
                    <Loader2 className="w-5 h-5 animate-spin" />
                    <span>Loading...</span>
                  </div>
                </td>
              </tr>
            ) : data.length === 0 ? (
              <tr>
                <td
                  colSpan={columns.length + (actions ? 1 : 0)}
                  className="py-8 text-center text-gray-500"
                >
                  {emptyMessage}
                </td>
              </tr>
            ) : (
              data.map((row, idx) => (
                <tr
                  key={rowKey ? rowKey(row, idx) : idx}
                  className="hover:bg-gray-50 transition-colors"
                >
                  {columns.map((col, cIdx) => (
                    <td
                      key={cIdx}
                      className={`px-4 py-2 whitespace-nowrap ${
                        col.className || ""
                      }`}
                    >
                      {col.accessor(row)}
                    </td>
                  ))}
                  {actions && (
                    <td className="px-4 py-2 text-right space-x-2">
                      {actions(row)}
                    </td>
                  )}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
