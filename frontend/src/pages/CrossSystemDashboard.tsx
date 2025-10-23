/**
 * src/pages/CrossSystemDashboard.tsx
 * ---------------------------------------------------------
 * Cross-System Dashboard
 *
 * Displays synchronized data between Police and Hospital systems.
 * Shows flagged entities from both, allows cross-system lookups,
 * and provides a snapshot of data consistency between systems.
 */

import React, { useEffect } from "react";
import { AppLayout } from "../components/layout/AppLayout";
import { DataTable, Column } from "../components/common/DataTable";
import { NotificationBanner } from "../components/common/NotificationBanner";
import { useCrossSystemQuery } from "../hooks/useCrossSystemQuery";
import { usePoliceData } from "../hooks/usePoliceData";
import { useHospitalData } from "../hooks/useHospitalData";
import { formatFullName, formatFlagStatus } from "../utils/formatting";
import type { Suspect, Patient } from "../services/types";
import { RefreshCcw } from "lucide-react";

export default function CrossSystemDashboard() {
  // Data hooks
  const {
    suspects,
    loading: suspectsLoading,
    error: suspectsError,
    refetch: refetchSuspects,
  } = usePoliceData();
  const {
    flaggedPatients,
    flaggedLoading,
    error: hospitalError,
    refetchFlagged,
  } = useHospitalData();

  const {
    queryPatientFromPolice,
    querySuspectFromHospital,
    policeQueryResult,
    hospitalQueryResult,
    policeQueryError,
    hospitalQueryError,
    clearErrors,
  } = useCrossSystemQuery();

  useEffect(() => {
    clearErrors();
  }, [clearErrors]);

  // Columns for police flagged suspects
  const suspectColumns: Column<Suspect>[] = [
    { header: "Name", accessor: (s: Suspect) => formatFullName(s.full_name) },
    { header: "Personal ID", accessor: (s: Suspect) => s.personal_id },
    {
      header: "Flag",
      accessor: (s: Suspect) => formatFlagStatus(s.flag),
      className: "text-center",
    },
  ];

  // Columns for hospital flagged patients
  const patientColumns: Column<Patient>[] = [
    { header: "Name", accessor: (p: Patient) => formatFullName(p.fullName) },
    { header: "Personal ID", accessor: (p: Patient) => p.personalId },
    {
      header: "Flag",
      accessor: (p: Patient) => formatFlagStatus(p.flag),
      className: "text-center",
    },
  ];

  const anyError =
    suspectsError || hospitalError || policeQueryError || hospitalQueryError;

  return (
    <AppLayout
      title="Cross-System Dashboard"
      subtitle="Real-time view of flagged records across systems"
    >
      <div className="space-y-6">
        {/* Error Banner */}
        {anyError && (
          <NotificationBanner
            message={anyError || "An error occurred while syncing systems."}
            variant="error"
            onClose={clearErrors}
          />
        )}

        {/* Refresh Controls */}
        <div className="flex justify-end">
          <button
            onClick={() => {
              refetchSuspects();
              refetchFlagged();
            }}
            className="flex items-center space-x-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors"
          >
            <RefreshCcw className="w-4 h-4" />
            <span>Refresh Data</span>
          </button>
        </div>

        {/* Police Flagged Records */}
        <section>
          <h2 className="text-lg font-semibold text-blue-700 mb-2">
            Flagged Suspects (Police)
          </h2>
          <DataTable
            data={suspects.filter((s) => s.flag)}
            columns={suspectColumns}
            loading={suspectsLoading}
            emptyMessage="No flagged suspects found."
          />
        </section>

        {/* Hospital Flagged Records */}
        <section>
          <h2 className="text-lg font-semibold text-red-700 mb-2">
            Flagged Patients (Hospital)
          </h2>
          <DataTable
            data={flaggedPatients}
            columns={patientColumns}
            loading={flaggedLoading}
            emptyMessage="No flagged patients found."
          />
        </section>

        {/* Cross-System Query Results */}
        <section>
          <h2 className="text-lg font-semibold text-gray-700 mb-3">
            Cross-System Lookup Results
          </h2>
          <div className="grid md:grid-cols-2 gap-4">
            <div className="p-4 bg-blue-50 rounded-xl border border-blue-200">
              <h3 className="font-semibold text-blue-700 mb-2">
                Police → Hospital
              </h3>
              {policeQueryResult ? (
                <p className="text-sm">
                  Found patient: <strong>{policeQueryResult.full_name}</strong>{" "}
                  ({policeQueryResult.personal_id})
                </p>
              ) : (
                <p className="text-sm text-gray-500 italic">
                  No patient data queried or record not found.
                </p>
              )}
            </div>

            <div className="p-4 bg-red-50 rounded-xl border border-red-200">
              <h3 className="font-semibold text-red-700 mb-2">
                Hospital → Police
              </h3>
              {hospitalQueryResult ? (
                <p className="text-sm">
                  Found suspect:{" "}
                  <strong>{hospitalQueryResult.full_name}</strong> (
                  {hospitalQueryResult.personal_id})
                </p>
              ) : (
                <p className="text-sm text-gray-500 italic">
                  No suspect data queried or record not found.
                </p>
              )}
            </div>
          </div>
        </section>
      </div>
    </AppLayout>
  );
}
