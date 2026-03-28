// =============================================================================
// Project:        Nelf
// Description:    Base CRUD service template
// =============================================================================

import { apiClient } from './config';
import { UnauthorizedError } from './errors';
import type { ApiResponse, PaginatedResponse } from '../../types';

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

  protected buildFetchHeaders(options: RequestInit): Record<string, string> {
    const headers: Record<string, string> = { ...apiClient.getHeaders() };
    if (options.headers) {
      new Headers(options.headers as HeadersInit).forEach((value, key) => {
        headers[key] = value;
      });
    }
    if (options.body instanceof FormData) {
      delete headers['Content-Type'];
    }
    return headers;
  }

  protected async handleResponse<T>(response: Response): Promise<T> {
    if (response.status === 401) {
      throw new UnauthorizedError();
    }
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      const msg =
        (errorData as { message?: string }).message || `HTTP error! status: ${response.status}`;
      throw new Error(msg);
    }
    const text = await response.text();
    if (!text.trim()) {
      return undefined as T;
    }
    return JSON.parse(text) as T;
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
        headers: this.buildFetchHeaders(options),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);
      return this.handleResponse<T>(response);
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof UnauthorizedError) {
        throw error;
      }
      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          throw new Error('API request failed: request timed out');
        }
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

  /** Bare JSON array from GET /{endpoint} (Rust/OpenAPI list). */
  async listAll(): Promise<T[]> {
    const raw = await this.request<unknown>(this.getBaseUrl(), { method: 'GET' });
    if (Array.isArray(raw)) {
      return raw as T[];
    }
    if (raw && typeof raw === 'object' && Array.isArray((raw as PaginatedResponse<T>).data)) {
      return (raw as PaginatedResponse<T>).data;
    }
    throw new Error('Unexpected list response shape');
  }

  async getOne(id: number): Promise<T> {
    return this.request<T>(`${this.getBaseUrl()}/${id}`, { method: 'GET' });
  }

  async createFormData(formData: FormData): Promise<T> {
    return this.request<T>(this.getBaseUrl(), { method: 'POST', body: formData });
  }

  async updateFormData(id: number, formData: FormData): Promise<T> {
    return this.request<T>(`${this.getBaseUrl()}/${id}`, { method: 'PUT', body: formData });
  }

  async removeOne(id: number): Promise<void> {
    await this.request<void>(`${this.getBaseUrl()}/${id}`, { method: 'DELETE' });
  }
}
