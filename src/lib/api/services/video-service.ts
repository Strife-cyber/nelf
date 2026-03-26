// =============================================================================
// Project:        Nelf
// Description:    Video CRUD service
// =============================================================================

import { BaseCrudService } from '../base-crud';
import type { Video, CreateVideoRequest, UpdateVideoRequest } from '../../../types';

export class VideoService extends BaseCrudService<Video, CreateVideoRequest, UpdateVideoRequest> {
  constructor() {
    super('videos');
  }

  async findByEventTitle(eventTitle: string): Promise<{ data: Video[]; success: boolean }> {
    try {
      const response = await this.request<{ data: Video[]; success: boolean }>(
        `${this.getBaseUrl()}/event/${encodeURIComponent(eventTitle)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }

  async toggleActive(id: number): Promise<{ data: Video; success: boolean }> {
    return this.request<{ data: Video; success: boolean }>(
      `${this.getBaseUrl()}/${id}/toggle-active`,
      { method: 'PATCH' }
    );
  }

  async getActive(): Promise<{ data: Video[]; success: boolean }> {
    try {
      const response = await this.request<{ data: Video[]; success: boolean }>(
        `${this.getBaseUrl()}/active`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }
}

export const videoService = new VideoService();
