import { Thread, Tag, Reply } from "@/types/forum";
import api from "./api";

interface ThreadSearchParams {
  query?: string;
  tags?: string[];
  authorId?: string;
  fromDate?: string;
  toDate?: string;
  limit?: number;
  offset?: number;
}

export const forumService = {
  async searchThreads(params: ThreadSearchParams): Promise<Thread[]> {
    const { data } = await api.get("/forum/threads/search", { params });
    return data;
  },

  async getThread(id: string): Promise<Thread> {
    const { data } = await api.get(`/forum/threads/${id}`);
    return data;
  },

  async getReplies(threadId: string): Promise<Reply[]> {
    const { data } = await api.get(`/forum/threads/${threadId}/replies`);
    return data;
  },

  async createReply(threadId: string, content: string): Promise<Reply> {
    const { data } = await api.post(`/forum/threads/${threadId}/replies`, { content });
    return data;
  },

  async getTags(): Promise<Tag[]> {
    const { data } = await api.get("/forum/tags");
    return data;
  },
};
