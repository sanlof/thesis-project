/**
 * src/components/common/DataForm.tsx
 * ---------------------------------------------------------
 * Reusable Data Form Component
 *
 * A dynamic, type-safe form for creating or editing records.
 * Supports:
 *  - Custom field configurations
 *  - Inline validation errors
 *  - Submit and reset actions
 *  - Loading and disabled states
 *
 * Used across both systems (Police and Hospital).
 */

import React, { useState } from "react";
import { Loader2 } from "lucide-react";

export interface FormField<T> {
  /** Field label */
  label: string;
  /** Property name on the data model */
  name: keyof T;
  /** Input type (text, number, etc.) */
  type?: string;
  /** Optional placeholder text */
  placeholder?: string;
  /** Whether field is required */
  required?: boolean;
  /** Optional validation function */
  validate?: (value: any) => string | null;
}

interface DataFormProps<T> {
  /** Initial form values */
  initialValues: T;
  /** Field configuration */
  fields: FormField<T>[];
  /** Submit handler */
  onSubmit: (values: T) => Promise<void> | void;
  /** Optional reset handler */
  onReset?: () => void;
  /** Optional submit button label */
  submitLabel?: string;
  /** Whether the form is currently submitting */
  loading?: boolean;
}

export function DataForm<T extends Record<string, any>>({
  initialValues,
  fields,
  onSubmit,
  onReset,
  submitLabel = "Submit",
  loading = false,
}: DataFormProps<T>) {
  const [values, setValues] = useState<T>(initialValues);
  const [errors, setErrors] = useState<Record<keyof T, string | null>>(
    {} as any
  );

  /**
   * Handle field value changes.
   */
  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setValues((prev) => ({ ...prev, [name]: value }));
  };

  /**
   * Run validations for all fields.
   * Returns true if form is valid.
   */
  const validateForm = (): boolean => {
    const newErrors: Record<string, string | null> = {};
    fields.forEach((field) => {
      const val = values[field.name];
      if (field.required && !val) {
        newErrors[field.name as string] = "This field is required.";
      } else if (field.validate) {
        const err = field.validate(val);
        if (err) newErrors[field.name as string] = err;
      }
    });
    setErrors(newErrors as any);
    return Object.values(newErrors).every((e) => !e);
  };

  /**
   * Handle form submission.
   */
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    try {
      await onSubmit(values);
      setErrors({} as any);
    } catch (err: any) {
      console.error("Form submission failed:", err);
    }
  };

  /**
   * Handle form reset (if provided).
   */
  const handleReset = () => {
    setValues(initialValues);
    setErrors({} as any);
    if (onReset) onReset();
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="bg-white border border-gray-200 rounded-2xl p-6 shadow-sm space-y-5"
    >
      {fields.map((field) => (
        <div key={String(field.name)}>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            {field.label}
            {field.required && <span className="text-red-500 ml-1">*</span>}
          </label>
          <input
            type={field.type || "text"}
            name={String(field.name)}
            placeholder={field.placeholder}
            value={values[field.name] as string}
            onChange={handleChange}
            className={`w-full border rounded-lg px-3 py-2 focus:outline-none focus:ring-2 ${
              errors[field.name]
                ? "border-red-400 focus:ring-red-300"
                : "border-gray-300 focus:ring-blue-200"
            }`}
            disabled={loading}
          />
          {errors[field.name] && (
            <p className="text-xs text-red-500 mt-1">{errors[field.name]}</p>
          )}
        </div>
      ))}

      <div className="flex justify-end space-x-3 pt-4 border-t border-gray-100">
        {onReset && (
          <button
            type="button"
            onClick={handleReset}
            disabled={loading}
            className="px-4 py-2 text-sm text-gray-600 border rounded-lg hover:bg-gray-50 disabled:opacity-50"
          >
            Reset
          </button>
        )}

        <button
          type="submit"
          disabled={loading}
          className="px-4 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 flex items-center space-x-2"
        >
          {loading && <Loader2 className="w-4 h-4 animate-spin" />}
          <span>{submitLabel}</span>
        </button>
      </div>
    </form>
  );
}
