export type TaskStatus = 'PENDING' | 'IN_PROGRESS' | 'COMPLETED' | 'BLOCKED';

export interface Task {
  id: string;
  name: string;
  description: string;
  projectId: string;
  status: TaskStatus;
  startDate: string;
  endDate: string;
  progress: number;
  assignedTo: string | null;
  dependencies: string[];
  createdAt: string;
  updatedAt: string;
}

export interface TaskCreate {
  name: string;
  description: string;
  projectId: string;
  startDate: string;
  endDate: string;
  assignedTo?: string;
  dependencies?: string[];
}

export interface TaskUpdate {
  name?: string;
  description?: string;
  status?: TaskStatus;
  startDate?: string;
  endDate?: string;
  progress?: number;
  assignedTo?: string;
  dependencies?: string[];
}