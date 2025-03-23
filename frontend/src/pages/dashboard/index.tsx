import { useState } from 'react';
import { GetServerSideProps, NextPage } from 'next';
import { useTranslation } from 'next-i18next';
import { serverSideTranslations } from 'next-i18next/serverSideTranslations';
import { useQuery, useMutation, useQueryClient } from 'react-query';
import { Project, projectService, CreateProjectDto } from '@/services/projects';
import { ProjectForm } from '@/components/projects/ProjectForm';
import Card from '@/widgets/card';
import Head from 'next/head';
import Link from 'next/link';

const Dashboard: NextPage = () => {
  const { t } = useTranslation('common');
  const [showNewProjectForm, setShowNewProjectForm] = useState(false);
  const queryClient = useQueryClient();

  const { data: projects, isLoading, error } = useQuery<Project[]>(
    'projects',
    projectService.getAll,
    {
      staleTime: 30000, // Consider data fresh for 30 seconds
      retry: 2, // Retry failed requests twice
    }
  );

  const createProject = useMutation(
    (newProject: CreateProjectDto) => projectService.create(newProject),
    {
      onSuccess: () => {
        queryClient.invalidateQueries('projects');
        setShowNewProjectForm(false);
      },
    }
  );

  const getStatusColor = (status: string) => {
    const colors = {
      ACTIVE: 'bg-green-100 text-green-800',
      COMPLETED: 'bg-blue-100 text-blue-800',
      ON_HOLD: 'bg-yellow-100 text-yellow-800',
    };
    return colors[status] || 'bg-gray-100 text-gray-800';
  };

  const getProjectStats = () => {
    if (!projects) return { active: 0, completed: 0, onHold: 0, total: 0 };
    return {
      active: projects.filter(p => p.status === 'ACTIVE').length,
      completed: projects.filter(p => p.status === 'COMPLETED').length,
      onHold: projects.filter(p => p.status === 'ON_HOLD').length,
      total: projects.length,
    };
  };

  const stats = getProjectStats();

  if (error) {
    return (
      <div className="flex justify-center items-center h-screen">
        <div className="text-center">
          <h2 className="text-xl font-semibold text-red-600 mb-2">
            {t('common.error')}
          </h2>
          <p className="text-gray-600">{t('common.tryAgain')}</p>
        </div>
      </div>
    );
  }

  return (
    <>
      <Head>
        <title>{t('dashboard.title')} - Waterfall Resource Manager</title>
      </Head>
      <div className="min-h-screen bg-gray-50">
        <div className="container mx-auto px-4 py-8">
          <div className="flex justify-between items-center mb-8">
            <h1 className="text-3xl font-bold text-gray-900">{t('dashboard.title')}</h1>
            <button
              onClick={() => setShowNewProjectForm(!showNewProjectForm)}
              className="bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-2 rounded-md shadow-sm transition duration-150 ease-in-out flex items-center"
            >
              <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
              </svg>
              {showNewProjectForm ? t('common.cancel') : t('project.create')}
            </button>
          </div>

          {/* Stats Overview */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
            <Card className="bg-white">
              <div className="text-center">
                <h3 className="text-lg font-medium text-gray-900">{t('dashboard.totalProjects')}</h3>
                <p className="text-3xl font-bold text-indigo-600">{stats.total}</p>
              </div>
            </Card>
            <Card className="bg-white">
              <div className="text-center">
                <h3 className="text-lg font-medium text-gray-900">{t('dashboard.activeProjects')}</h3>
                <p className="text-3xl font-bold text-green-600">{stats.active}</p>
              </div>
            </Card>
            <Card className="bg-white">
              <div className="text-center">
                <h3 className="text-lg font-medium text-gray-900">{t('dashboard.completedProjects')}</h3>
                <p className="text-3xl font-bold text-blue-600">{stats.completed}</p>
              </div>
            </Card>
            <Card className="bg-white">
              <div className="text-center">
                <h3 className="text-lg font-medium text-gray-900">{t('dashboard.onHoldProjects')}</h3>
                <p className="text-3xl font-bold text-yellow-600">{stats.onHold}</p>
              </div>
            </Card>
          </div>

          {showNewProjectForm && (
            <Card className="mb-8 bg-white">
              <ProjectForm
                onSubmit={createProject.mutate}
                isLoading={createProject.isLoading}
                error={createProject.error}
              />
            </Card>
          )}

          {isLoading ? (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {[1, 2, 3].map((n) => (
                <Card key={n} className="animate-pulse bg-white">
                  <div className="h-4 bg-gray-200 rounded w-3/4 mb-4"></div>
                  <div className="h-4 bg-gray-200 rounded w-1/2 mb-4"></div>
                  <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                </Card>
              ))}
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {projects?.map((project) => (
                <Link href={`/projects/${project.id}`} key={project.id}>
                  <Card className="bg-white hover:shadow-lg transition-shadow duration-200 cursor-pointer">
                    <div className="flex justify-between items-start mb-4">
                      <h3 className="text-xl font-semibold text-gray-900">{project.name}</h3>
                      <span className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(project.status)}`}>
                        {project.status.toLowerCase()}
                      </span>
                    </div>
                    <p className="text-gray-600 mb-4 line-clamp-2">{project.description || t('common.noDescription')}</p>
                    <div className="flex justify-between items-center text-sm text-gray-500">
                      <div className="flex items-center">
                        <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>
                        {new Date(project.start_date).toLocaleDateString()}
                      </div>
                      <div className="flex items-center">
                        <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        ${project.budget.toLocaleString()}
                      </div>
                    </div>
                  </Card>
                </Link>
              ))}
            </div>
          )}
        </div>
      </div>
    </>
  );
};

export const getServerSideProps: GetServerSideProps = async ({ locale = 'en' }) => {
  return {
    props: {
      ...(await serverSideTranslations(locale, ['common'])),
    },
  };
};

export default Dashboard;
