import { useState } from "react";
import { useForm } from "react-hook-form";
import { CreateProjectDto } from "@/services/projects";
import { useTranslation } from "next-i18next";

interface ProjectFormProps {
  onSubmit: (data: CreateProjectDto) => Promise<void>;
}

export const ProjectForm = ({ onSubmit }: ProjectFormProps) => {
  const { t } = useTranslation("common");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<CreateProjectDto>();

  const handleFormSubmit = async (formData: CreateProjectDto) => {
    try {
      setIsSubmitting(true);
      // Convert dates to ISO string with time (UTC midnight)
      const data = {
        ...formData,
        start_date: new Date(formData.start_date + 'T00:00:00Z').toISOString(),
        end_date: new Date(formData.end_date + 'T00:00:00Z').toISOString(),
      };
      await onSubmit(data);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700">
          {t("project.name")}
        </label>
        <input
          type="text"
          {...register("name", { required: t("validation.required") })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
        />
        {errors.name && (
          <p className="mt-1 text-sm text-red-600">{errors.name.message}</p>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700">
          {t("project.description")}
        </label>
        <textarea
          {...register("description")}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            {t("project.startDate")}
          </label>
          <input
            type="date"
            {...register("start_date", { required: t("validation.required") })}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          />
          {errors.start_date && (
            <p className="mt-1 text-sm text-red-600">
              {errors.start_date.message}
            </p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            {t("project.endDate")}
          </label>
          <input
            type="date"
            {...register("end_date", { required: t("validation.required") })}
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
          />
          {errors.end_date && (
            <p className="mt-1 text-sm text-red-600">
              {errors.end_date.message}
            </p>
          )}
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700">
          {t("project.budget")}
        </label>
        <input
          type="number"
          {...register("budget", {
            required: t("validation.required"),
            min: 0,
          })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
        />
        {errors.budget && (
          <p className="mt-1 text-sm text-red-600">{errors.budget.message}</p>
        )}
      </div>

      <button
        type="submit"
        disabled={isSubmitting}
        className="inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50"
      >
        {isSubmitting ? t("common.submitting") : t("common.submit")}
      </button>
    </form>
  );
};
