import api from "./api";

export interface Project {
  id: string;
  name: string;
  description?: string;
  start_date: string;
  end_date: string;
  budget: number;
  status: "ACTIVE" | "COMPLETED" | "ON_HOLD";
}

export interface CreateProjectDto {
  name: string;
  description?: string;
  start_date: string;
  end_date: string;
  budget: number;
  client_id?: string;
}

export const projectService = {
  getAll: async () => {
    const { data } = await api.get<Project[]>("/projects");
    return data;
  },

  create: async (project: CreateProjectDto) => {
    const { data } = await api.post<Project>("/projects", project);
    return data;
  },

  getById: async (id: string) => {
    const { data } = await api.get<Project>(`/projects/${id}`);
    return data;
  },
};
