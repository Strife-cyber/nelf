// =============================================================================
// Project:        Nelf
// Description:    User CRUD service
// =============================================================================

import { BaseCrudService } from '../base-crud';
import type { User, CreateUserRequest, UpdateUserRequest } from '../../../types';

export class UserService extends BaseCrudService<User, CreateUserRequest, UpdateUserRequest> {
  constructor() {
    super('users');
  }

  async findByEmail(email: string): Promise<{ data: User | null; success: boolean }> {
    try {
      const response = await this.request<{ data: User | null; success: boolean }>(
        `${this.getBaseUrl()}/email/${encodeURIComponent(email)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: null, success: false };
    }
  }

  async findByRole(role: string): Promise<{ data: User[]; success: boolean }> {
    try {
      const response = await this.request<{ data: User[]; success: boolean }>(
        `${this.getBaseUrl()}/role/${encodeURIComponent(role)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }

  async toggleActive(id: number): Promise<{ data: User; success: boolean }> {
    return this.request<{ data: User; success: boolean }>(
      `${this.getBaseUrl()}/${id}/toggle-active`,
      { method: 'PATCH' }
    );
  }
}

export const userService = new UserService();
