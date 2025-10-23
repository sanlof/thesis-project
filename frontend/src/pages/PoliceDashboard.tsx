/**
 * src/pages/PoliceDashboard.tsx
 * ---------------------------------------------------------
 * Police System Dashboard
 *
 * Displays and manages suspects in the Police System.
 * Provides create, update, delete, and flagging functionality.
 * Automatically refreshes via usePoliceData polling.
 */

import React, { useState } from "react";
import { AppLayout } from "../components/layout/AppLayout";
import { DataTable, Column } from "../components/common/DataTable";
import { NotificationBanner } from "../components/common/NotificationBanner";
import { usePoliceData } from "../hooks/usePoliceData";
import { formatFullName, formatFlagStatus } from "../utils/formatting";
import {
  validateSwedishPersonalId,
  getValidationError,
} from "../utils/validation";
import type { Suspect, CreateSuspect, UpdateSuspect } from "../services/types";
import { Plus, Flag, Trash2 } from "lucide-react";

export default function PoliceDashboard() {
  const {
    suspects,
    loading,
    error,
    createSuspect,
    updateSuspect,
    deleteSuspect,
    updateFlag,
  } = usePoliceData();

  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState<CreateSuspect>({
    fullName: "",
    personalId: "",
    flag: false,
  });
  const [formError, setFormError] = useState<string | null>(null);
  const [successMsg, setSuccessMsg] = useState<string | null>(null);

  // ----------------------------
  // Form Handlers
  // ----------------------------
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === "checkbox" ? checked : value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const validationError = getValidationError(formData.personalId);
    if (validationError) {
      setFormError(validationError);
      return;
    }

    try {
      await createSuspect(formData);
      setShowForm(false);
      setFormData({ fullName: "", personalId: "", flag: false });
      setSuccessMsg("New suspect successfully added.");
    } catch {
      setFormError("Failed to create suspect.");
    }
  };

  // ----------------------------
  // Table Columns
  // ----------------------------
  const columns: Column<Suspect>[] = [
    { header: "Name", accessor: (s) => formatFullName(s.fullName) },
    { header: "Personal ID", accessor: (s) => s.personalId },
    {
      header: "Flag",
      accessor: (s) => formatFlagStatus(s.flag),
      className: "text-center",
    },
  ];

  return (
    <AppLayout
      title="Police Dashboard"
      subtitle="Manage suspects and flag statuses"
      system="police"
    >
      <div className="space-y-6">
        {/* Notifications */}
        {error && (
          <NotificationBanner
            message={error}
            variant="error"
            onClose={() => window.location.reload()}
          />
        )}
        {successMsg && (
          <NotificationBanner
            message={successMsg}
            variant="success"
            onClose={() => setSuccessMsg(null)}
          />
        )}

        {/* Create New Suspect */}
        <div className="flex justify-between items-center">
          <h2 className="text-lg font-semibold text-blue-700">
            Suspect Records
          </h2>
          <button
            onClick={() => setShowForm((prev) => !prev)}
            className="flex items-center space-x-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors"
          >
            <Plus className="w-4 h-4" />
            <span>{showForm ? "Cancel" : "Add Suspect"}</span>
          </button>
        </div>

        {/* Form Section */}
        {showForm && (
          <form
            onSubmit={handleSubmit}
            className="p-4 bg-blue-50 border border-blue-200 rounded-xl shadow-sm space-y-4"
          >
            {formError && (
              <NotificationBanner
                message={formError}
                variant="error"
                onClose={() => setFormError(null)}
              />
            )}
            <div className="grid md:grid-cols-3 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">
                  Full Name
                </label>
                <input
                  type="text"
                  name="fullName"
                  value={formData.fullName}
                  onChange={handleChange}
                  className="mt-1 block w-full border border-gray-300 rounded-lg px-3 py-2"
                  required
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700">
                  Personal ID
                </label>
                <input
                  type="text"
                  name="personalId"
                  value={formData.personalId}
                  onChange={handleChange}
                  placeholder="YYYYMMDD-XXXX"
                  className="mt-1 block w-full border border-gray-300 rounded-lg px-3 py-2"
                  required
                />
              </div>

              <div className="flex items-center space-x-2 pt-6">
                <input
                  id="flag"
                  name="flag"
                  type="checkbox"
                  checked={formData.flag}
                  onChange={handleChange}
                  className="h-4 w-4 text-blue-600 border-gray-300 rounded"
                />
                <label htmlFor="flag" className="text-sm text-gray-700">
                  Mark as flagged
                </label>
              </div>
            </div>

            <div className="flex justify-end space-x-2 pt-2">
              <button
                type="submit"
                className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors"
              >
                Save Suspect
              </button>
              <button
                type="button"
                onClick={() => setShowForm(false)}
                className="bg-gray-200 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-300 transition-colors"
              >
                Cancel
              </button>
            </div>
          </form>
        )}

        {/* Suspect Table */}
        <DataTable
          data={suspects}
          columns={columns}
          loading={loading}
          emptyMessage="No suspects found."
          actions={(suspect) => (
            <div className="flex items-center space-x-3">
              <button
                title="Toggle flag"
                onClick={() => updateFlag(suspect.personalId, !suspect.flag)}
                className="text-blue-600 hover:text-blue-800"
              >
                <Flag className="w-4 h-4" />
              </button>
              <button
                title="Delete suspect"
                onClick={() => deleteSuspect(suspect.id)}
                className="text-red-600 hover:text-red-800"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
          )}
        />
      </div>
    </AppLayout>
  );
}
