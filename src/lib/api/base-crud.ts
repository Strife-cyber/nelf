// =============================================================================
// Project:        Nelf
// Description:    Base CRUD service template
// =============================================================================

import { apiClient } from './config';
import type { ApiResponse, PaginatedResponse, ApiError } from '../../types';

export interface CrudService<T, CreateRequest, UpdateRequest> {
  getAll(params?: Record<string, any>): Promise<PaginatedResponse<T>>;
  getById(id: number): Promise<ApiResponse<T>>;
  create(data: CreateRequest): Promise<ApiResponse<T>>;
  update(id: number, data: UpdateRequest): Promise<ApiResponse<T>>;
  delete(id: number): Promise<ApiResponse<void>>;
}

export abstract class BaseCrudService<T, CreateRequest, UpdateRequest>
  implements CrudService<T, CreateRequest, UpdateRequest>
{
  protected readonly endpoint: string;

  constructor(endpoint: string) {
    this.endpoint = endpoint;
  }

  protected getBaseUrl(): string {
    return `${apiClient.getBaseUrl()}/${this.endpoint}`;
  }

  protected async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
    }
    return response.json();
  }

  protected async request<T>(
    url: string,
    options: RequestInit = {}
  ): Promise<T> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), apiClient.getTimeout());

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...apiClient.getHeaders(),
          ...options.headers,
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);
      return this.handleResponse<T>(response);
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof Error) {
        throw new Error(`API request failed: ${error.message}`);
      }
      throw new Error('Unknown API error occurred');
    }
  }

  async getAll(params?: Record<string, any>): Promise<PaginatedResponse<T>> {
    const url = new URL(this.getBaseUrl());
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          url.searchParams.append(key, String(value));
        }
      });
    }

    return this.request<PaginatedResponse<T>>(url.toString(), {
      method: 'GET',
    });
  }

  async getById(id: number): Promise<ApiResponse<T>> {
    return this.request<ApiResponse<T>>(`${this.getBaseUrl()}/${id}`, {
      method: 'GET',
    });
  }

  async create(data: CreateRequest): Promise<ApiResponse<T>> {
    return this.request<ApiResponse<T>>(this.getBaseUrl(), {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async update(id: number, data: UpdateRequest): Promise<ApiResponse<T>> {
    return this.request<ApiResponse<T>>(`${this.getBaseUrl()}/${id}`, {
      method: 'PUT' as RequestInit['method'],
      body: JSON.stringify(data),
    });
  }

  async delete(id: number): Promise<ApiResponse<void>> {
    return this.request<ApiResponse<void>>(`${this.getBaseUrl()}/${id}`, {
      method: 'DELETE',
    });
  }

  async patch(id: number, data: Partial<UpdateRequest>): Promise<ApiResponse<T>> {
    return this.request<ApiResponse<T>>(`${this.getBaseUrl()}/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(data),
    });
  }
}
