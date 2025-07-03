import axios from 'axios';
import type { AuthResponse, LoginRequest, RegisterRequest, User, CreateUserRequest } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests if available
apiClient.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Auth API
export const authApi = {
  login: async (credentials: LoginRequest): Promise<AuthResponse> => {
    const response = await apiClient.post('/auth/login', credentials);
    return response.data;
  },

  register: async (userData: RegisterRequest): Promise<AuthResponse> => {
    const response = await apiClient.post('/auth/register', userData);
    return response.data;
  },
};

// Users API
export const usersApi = {
  getUsers: async (): Promise<User[]> => {
    const response = await apiClient.get('/users');
    return response.data;
  },

  getUser: async (id: string): Promise<User> => {
    const response = await apiClient.get(`/users/${id}`);
    return response.data;
  },

  createUser: async (userData: CreateUserRequest): Promise<User> => {
    const response = await apiClient.post('/users', userData);
    return response.data;
  },

  updateUser: async (id: string, userData: Partial<CreateUserRequest>): Promise<User> => {
    const response = await apiClient.put(`/users/${id}`, userData);
    return response.data;
  },

  deleteUser: async (id: string): Promise<void> => {
    await apiClient.delete(`/users/${id}`);
  },
};

// Health check
export const healthApi = {
  check: async () => {
    const response = await apiClient.get('/health');
    return response.data;
  },
}; 