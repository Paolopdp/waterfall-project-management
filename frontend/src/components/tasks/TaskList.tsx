import { useState } from 'react';
import { Task } from '@/types/task';
import { useTranslation } from 'next-i18next';

interface TaskListProps {
  tasks: Task[];
}

export default function TaskList({ tasks }: TaskListProps) {
  const { t } = useTranslation('common');
  const [sortField, setSortField] = useState<keyof Task>('startDate');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');

  const sortedTasks = [...tasks].sort((a, b) => {
    if (sortDirection === 'asc') {
      return a[sortField] > b[sortField] ? 1 : -1;
    }
    return a[sortField] < b[sortField] ? 1 : -1;
  });

  const handleSort = (field: keyof Task) => {
    if (field === sortField) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(field);
      setSortDirection('asc');
    }
  };

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full bg-white">
        <thead className="bg-gray-50">
          <tr>
            <th
              className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              onClick={() => handleSort('name')}
            >
              {t('tasks.fields.name')}
            </th>
            <th
              className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              onClick={() => handleSort('status')}
            >
              {t('tasks.fields.status')}
            </th>
            <th
              className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              onClick={() => handleSort('startDate')}
            >
              {t('tasks.fields.startDate')}
            </th>
            <th
              className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              onClick={() => handleSort('endDate')}
            >
              {t('tasks.fields.endDate')}
            </th>
            <th
              className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              onClick={() => handleSort('progress')}
            >
              {t('tasks.fields.progress')}
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              {t('common.actions')}
            </th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {sortedTasks.map((task) => (
            <tr key={task.id}>
              <td className="px-6 py-4 whitespace-nowrap">{task.name}</td>
              <td className="px-6 py-4 whitespace-nowrap">
                <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                  task.status === 'COMPLETED' ? 'bg-green-100 text-green-800' :
                  task.status === 'IN_PROGRESS' ? 'bg-blue-100 text-blue-800' :
                  'bg-gray-100 text-gray-800'
                }`}>
                  {t(`tasks.status.${task.status.toLowerCase()}`)}
                </span>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                {new Date(task.startDate).toLocaleDateString()}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                {new Date(task.endDate).toLocaleDateString()}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="w-full bg-gray-200 rounded-full h-2.5">
                  <div
                    className="bg-blue-600 h-2.5 rounded-full"
                    style={{ width: `${task.progress}%` }}
                  ></div>
                </div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <button className="text-indigo-600 hover:text-indigo-900">
                  {t('common.edit')}
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}