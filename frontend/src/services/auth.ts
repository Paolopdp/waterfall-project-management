import { useMutation } from "react-query";
import { LoginCredentials, AuthResponse, AuthError } from "@/types/auth";
import api from "./api";
import Cookies from "js-cookie";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001/api";

export const useAuth = () => {
  const loginMutation = useMutation<AuthResponse, AuthError, LoginCredentials>(
    async (credentials) => {
      const { data } = await api.post<AuthResponse>("/auth/login", credentials);
      return data;
    },
    {
      onSuccess: (data) => {
        // Store token in both cookie and localStorage (cookie for middleware, localStorage for client-side checks)
        Cookies.set("auth_token", data.token, {
          expires: 1, // 1 day
          path: "/",
          sameSite: "strict",
        });
        localStorage.setItem("auth_token", data.token);
      },
    }
  );

  const logout = () => {
    Cookies.remove("auth_token");
    localStorage.removeItem("auth_token");
    window.location.href = "/login";
  };

  const getToken = (): string | null => {
    if (typeof window !== "undefined") {
      return Cookies.get("auth_token") || null;
    }
    return null;
  };

  const isAuthenticated = (): boolean => {
    const token = getToken();
    return !!token;
  };

  return {
    login: loginMutation.mutateAsync,
    logout,
    isAuthenticated,
    getToken,
    isLoading: loginMutation.isLoading,
    error: loginMutation.error,
  };
};
