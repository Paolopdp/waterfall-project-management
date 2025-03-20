import { DragEvent, useState } from 'react';
import { Task, TaskStatus } from '@/types/task';
import { useTranslation } from 'next-i18next';

interface TaskBoardProps {
  tasks: Task[];
}

interface TaskCardProps {
  task: Task;
  onDragStart: (e: DragEvent<HTMLDivElement>, task: Task) => void;
}

const TaskCard = ({ task, onDragStart }: TaskCardProps) => {
  return (
    <div
      draggable
      onDragStart={(e) => onDragStart(e, task)}
      className="bg-white p-4 rounded-lg shadow mb-3 cursor-move hover:shadow-md transition-shadow"
    >
      <h3 className="font-medium text-gray-900 mb-1">{task.name}</h3>
      <div className="text-sm text-gray-500 mb-2 line-clamp-2">
        {task.description}
      </div>
      <div className="flex items-center justify-between">
        <div className="w-full max-w-[150px]">
          <div className="text-xs text-gray-500 mb-1">Progress</div>
          <div className="w-full bg-gray-200 rounded-full h-1.5">
            <div
              className="bg-blue-600 h-1.5 rounded-full"
              style={{ width: `${task.progress}%` }}
            />
          </div>
        </div>
        <div className="text-xs text-gray-500">
          {new Date(task.endDate).toLocaleDateString()}
        </div>
      </div>
    </div>
  );
};

const COLUMNS: TaskStatus[] = ['PENDING', 'IN_PROGRESS', 'BLOCKED', 'COMPLETED'];

export default function TaskBoard({ tasks }: TaskBoardProps) {
  const { t } = useTranslation('common');
  const [draggedTask, setDraggedTask] = useState<Task | null>(null);

  const handleDragStart = (e: DragEvent<HTMLDivElement>, task: Task) => {
    setDraggedTask(task);
  };

  const handleDragOver = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
  };

  const handleDrop = (e: DragEvent<HTMLDivElement>, status: TaskStatus) => {
    e.preventDefault();
    if (!draggedTask) return;

    // Here you would typically make an API call to update the task status
    console.log(`Moving task ${draggedTask.id} to ${status}`);
    setDraggedTask(null);
  };

  const getColumnTasks = (status: TaskStatus) => {
    return tasks.filter((task) => task.status === status);
  };

  return (
    <div className="flex gap-4 h-[calc(100vh-12rem)] overflow-hidden">
      {COLUMNS.map((status) => (
        <div
          key={status}
          className="flex-1 bg-gray-100 rounded-lg p-4"
          onDragOver={handleDragOver}
          onDrop={(e) => handleDrop(e, status)}
        >
          <div className="flex items-center justify-between mb-4">
            <h2 className="font-medium text-gray-900">
              {t(`tasks.status.${status.toLowerCase()}`)}
            </h2>
            <span className="bg-gray-200 text-gray-700 text-xs font-medium px-2.5 py-0.5 rounded">
              {getColumnTasks(status).length}
            </span>
          </div>
          <div className="overflow-y-auto h-full">
            {getColumnTasks(status).map((task) => (
              <TaskCard
                key={task.id}
                task={task}
                onDragStart={handleDragStart}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}