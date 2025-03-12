// src/pages/dashboard.tsx
import Card from "@/widgets/card";
import { GetServerSideProps, NextPage } from "next";
import { useTranslation } from "next-i18next";
import { serverSideTranslations } from "next-i18next/serverSideTranslations";
import Link from "next/link";

interface Project {
  id: string;
  name: string;
  // Include any other properties that your project may have
}

interface DashboardProps {
  projects: Project[];
}
//TODO create a new component for the recent projects
const Dashboard: NextPage<DashboardProps> = ({ projects }) => {
  const { t } = useTranslation("common");
  console.log(t);
  console.log(t("db"));

  return (
    <div className="min-h-screen p-4">
      <header className="mb-8">
        <h1 className="text-3xl font-bold">{t("dashboardTitle")}</h1>
        <nav className="mt-4">
          <Link href="/" className="text-blue-500 hover:underline">
            Home
          </Link>
        </nav>
      </header>
      <main>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <Card title={t("activeProjects")} value={projects.length} />
          <Card title={t("assignedResources")} value={"TODO"} />
          <Card title={t("pendingTasks")} value={"TODO"} />
        </div>

        <div className="mt-8 border p-4 rounded-lg">
          <h2 className="text-xl font-bold mb-4">{t("recentProjects")}</h2>
          {projects.length === 0 ? (
            <p className="text-gray-500">{t("noProjectAvailable")}</p>
          ) : (
            <ul>
              {projects.map((project) => (
                <li key={project.id} className="py-1 border-b">
                  {project.name}
                </li>
              ))}
            </ul>
          )}
        </div>
      </main>
    </div>
  );
};

export default Dashboard;

export const getServerSideProps: GetServerSideProps<DashboardProps> = async ({
  locale = "en",
}) => {
  try {
    const res = await fetch("http://localhost:3001/api/projects");
    if (!res.ok) {
      throw new Error("Failed to fetch projects");
    }
    const projects: Project[] = await res.json();
    return {
      props: {
        projects,
        ...(await serverSideTranslations(locale, ["common"])),
      },
    };
  } catch (error) {
    console.error(error);
    return {
      props: {
        projects: [],
        ...(await serverSideTranslations(locale, ["common"])),
      },
    };
  }
};
