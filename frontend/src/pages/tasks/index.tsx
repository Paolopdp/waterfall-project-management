import { useQuery } from "react-query";
import Head from "next/head";
import { useTranslation } from "next-i18next";
import { serverSideTranslations } from "next-i18next/serverSideTranslations";
import type { GetServerSideProps } from "next";
import TaskList from "@/components/tasks/TaskList";
import TaskBoard from "@/components/tasks/TaskBoard";
import { useState } from "react";

type ViewMode = "list" | "board" | "gantt";

export default function Tasks() {
  const { t } = useTranslation("common");
  const [viewMode, setViewMode] = useState<ViewMode>("list");

  const {
    data: tasks,
    isLoading,
    error,
  } = useQuery("tasks", async () => {
    const response = await fetch("http://127.0.0.1:3001/api/tasks");
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    return response.json();
  });

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error loading tasks</div>;

  return (
    <>
      <Head>
        <title>{t("tasks.title")} - Waterfall Resource Manager</title>
      </Head>
      <main className="p-4">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">{t("tasks.title")}</h1>
          <div className="flex gap-2">
            <button
              className={`px-4 py-2 rounded ${
                viewMode === "list" ? "bg-blue-500 text-white" : "bg-gray-200"
              }`}
              onClick={() => setViewMode("list")}
            >
              {t("tasks.viewModes.list")}
            </button>
            <button
              className={`px-4 py-2 rounded ${
                viewMode === "board" ? "bg-blue-500 text-white" : "bg-gray-200"
              }`}
              onClick={() => setViewMode("board")}
            >
              {t("tasks.viewModes.board")}
            </button>
            <button
              className={`px-4 py-2 rounded ${
                viewMode === "gantt" ? "bg-blue-500 text-white" : "bg-gray-200"
              }`}
              onClick={() => setViewMode("gantt")}
            >
              {t("tasks.viewModes.gantt")}
            </button>
          </div>
        </div>

        {viewMode === "list" && <TaskList tasks={tasks} />}
        {viewMode === "board" && <TaskBoard tasks={tasks} />}
        {viewMode === "gantt" && <div>Gantt view coming soon...</div>}
      </main>
    </>
  );
}

export const getServerSideProps: GetServerSideProps = async ({ locale }) => {
  return {
    props: {
      ...(await serverSideTranslations(locale ?? "en", ["common"])),
    },
  };
};
