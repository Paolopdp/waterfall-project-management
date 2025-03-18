import { useMutation, useQueryClient } from 'react-query';
import { Task, TaskUpdate } from '@/types/task';

export function useTasks() {
  const queryClient = useQueryClient();

  const updateTask = useMutation(
    async ({ taskId, updates }: { taskId: string; updates: TaskUpdate }) => {
      const response = await fetch(`http://localhost:3001/tasks/${taskId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      });

      if (!response.ok) {
        throw new Error('Failed to update task');
      }

      return response.json();
    },
    {
      onSuccess: () => {
        queryClient.invalidateQueries('tasks');
      },
    }
  );

  return {
    updateTask,
  };
}