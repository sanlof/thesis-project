/**
 * src/pages/HospitalDashboard.tsx
 * ---------------------------------------------------------
 * Hospital System Dashboard
 *
 * Displays and manages patients in the Hospital System.
 * Provides create, update, and delete functionality.
 * Also lists flagged patients for security synchronization.
 */

import React, { useState } from "react";
import { AppLayout } from "../components/layout/AppLayout";
import { DataTable, Column } from "../components/common/DataTable";
import { NotificationBanner } from "../components/common/NotificationBanner";
import { useHospitalData } from "../hooks/useHospitalData";
import { formatFullName, formatFlagStatus } from "../utils/formatting";
import { getValidationError } from "../utils/validation";
import type { Patient, CreatePatient, UpdatePatient } from "../services/types";
import { Plus, Edit, Trash2 } from "lucide-react";

export default function HospitalDashboard() {
  const {
    patients,
    flaggedPatients,
    loading,
    flaggedLoading,
    error,
    createPatient,
    updatePatient,
    deletePatient,
  } = useHospitalData();

  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState<CreatePatient>({
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
      await createPatient(formData);
      setShowForm(false);
      setFormData({ fullName: "", personalId: "", flag: false });
      setSuccessMsg("New patient successfully added.");
    } catch {
      setFormError("Failed to create patient.");
    }
  };

  // ----------------------------
  // Table Columns
  // ----------------------------
  const patientColumns: Column<Patient>[] = [
    { header: "Name", accessor: (p) => formatFullName(p.fullName) },
    { header: "Personal ID", accessor: (p) => p.personalId },
    {
      header: "Flag",
      accessor: (p) => formatFlagStatus(p.flag),
      className: "text-center",
    },
  ];

  const flaggedColumns: Column<Patient>[] = [
    { header: "Name", accessor: (p) => formatFullName(p.fullName) },
    { header: "Personal ID", accessor: (p) => p.personalId },
    {
      header: "Flag",
      accessor: (p) => formatFlagStatus(p.flag),
      className: "text-center",
    },
  ];

  return (
    <AppLayout
      title="Hospital Dashboard"
      subtitle="Manage patients and flagged records"
      system="hospital"
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
        {formError && (
          <NotificationBanner
            message={formError}
            variant="error"
            onClose={() => setFormError(null)}
          />
        )}
        {successMsg && (
          <NotificationBanner
            message={successMsg}
            variant="success"
            onClose={() => setSuccessMsg(null)}
          />
        )}

        {/* Create New Patient */}
        <div className="flex justify-between items-center">
          <h2 className="text-lg font-semibold text-red-700">
            Patient Records
          </h2>
          <button
            onClick={() => setShowForm((prev) => !prev)}
            className="flex items-center space-x-2 bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition-colors"
          >
            <Plus className="w-4 h-4" />
            <span>{showForm ? "Cancel" : "Add Patient"}</span>
          </button>
        </div>

        {/* Form Section */}
        {showForm && (
          <form
            onSubmit={handleSubmit}
            className="p-4 bg-red-50 border border-red-200 rounded-xl shadow-sm space-y-4"
          >
            <div className="grid md:grid-cols-3 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">
                  Full Name
                </label>
                <input
                  type="text"
                  name="fullName"
                  value={formData.full_name}
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
                  className="h-4 w-4 text-red-600 border-gray-300 rounded"
                />
                <label htmlFor="flag" className="text-sm text-gray-700">
                  Mark as flagged
                </label>
              </div>
            </div>

            <div className="flex justify-end space-x-2 pt-2">
              <button
                type="submit"
                className="bg-red-600 text-white px-4 py-2 rounded-lg hover:bg-red-700 transition-colors"
              >
                Save Patient
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

        {/* Patient Table */}
        <DataTable
          data={patients}
          columns={patientColumns}
          loading={loading}
          emptyMessage="No patients found."
          actions={(patient) => (
            <div className="flex items-center space-x-3">
              <button
                title="Edit patient"
                onClick={() =>
                  updatePatient(patient.id, { flag: !patient.flag })
                }
                className="text-red-600 hover:text-red-800"
              >
                <Edit className="w-4 h-4" />
              </button>
              <button
                title="Delete patient"
                onClick={() => deletePatient(patient.id)}
                className="text-gray-500 hover:text-red-600"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
          )}
        />

        {/* Flagged Patients Table */}
        <section>
          <h2 className="text-lg font-semibold text-red-700 mt-6 mb-2">
            Flagged Patients
          </h2>
          <DataTable
            data={flaggedPatients}
            columns={flaggedColumns}
            loading={flaggedLoading}
            emptyMessage="No flagged patients found."
          />
        </section>
      </div>
    </AppLayout>
  );
}
