// =============================================================================
// Project:        Nelf
// Description:    Flyer CRUD service
// =============================================================================

import { BaseCrudService } from '../base-crud';
import type { Flyer, CreateFlyerRequest, UpdateFlyerRequest } from '../../../types';

export class FlyerService extends BaseCrudService<Flyer, CreateFlyerRequest, UpdateFlyerRequest> {
  constructor() {
    super('flyers');
  }

  async findByEventTitle(eventTitle: string): Promise<{ data: Flyer[]; success: boolean }> {
    try {
      const response = await this.request<{ data: Flyer[]; success: boolean }>(
        `${this.getBaseUrl()}/event/${encodeURIComponent(eventTitle)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }

  async toggleActive(id: number): Promise<{ data: Flyer; success: boolean }> {
    return this.request<{ data: Flyer; success: boolean }>(
      `${this.getBaseUrl()}/${id}/toggle-active`,
      { method: 'PATCH' }
    );
  }

  async getActive(): Promise<{ data: Flyer[]; success: boolean }> {
    try {
      const response = await this.request<{ data: Flyer[]; success: boolean }>(
        `${this.getBaseUrl()}/active`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }
}

export const flyerService = new FlyerService();
