// =============================================================================
// Project:        Nelf
// Description:    Website Preview CRUD service
// =============================================================================

import { BaseCrudService } from '../base-crud';
import type { 
  WebsitePreview, 
  CreateWebsitePreviewRequest, 
  UpdateWebsitePreviewRequest 
} from '../../../types';

export class WebsitePreviewService extends BaseCrudService<
  WebsitePreview, 
  CreateWebsitePreviewRequest, 
  UpdateWebsitePreviewRequest
> {
  constructor() {
    super('website-previews');
  }

  async findByUrl(url: string): Promise<{ data: WebsitePreview | null; success: boolean }> {
    try {
      const response = await this.request<{ data: WebsitePreview | null; success: boolean }>(
        `${this.getBaseUrl()}/url/${encodeURIComponent(url)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: null, success: false };
    }
  }

  async findByContentType(contentType: string): Promise<{ data: WebsitePreview[]; success: boolean }> {
    try {
      const response = await this.request<{ data: WebsitePreview[]; success: boolean }>(
        `${this.getBaseUrl()}/content-type/${encodeURIComponent(contentType)}`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }

  async getActive(): Promise<{ data: WebsitePreview[]; success: boolean }> {
    try {
      const response = await this.request<{ data: WebsitePreview[]; success: boolean }>(
        `${this.getBaseUrl()}/active`,
        { method: 'GET' }
      );
      return response;
    } catch (error) {
      return { data: [], success: false };
    }
  }

  async refreshPreview(id: number): Promise<{ data: WebsitePreview; success: boolean }> {
    return this.request<{ data: WebsitePreview; success: boolean }>(
      `${this.getBaseUrl()}/${id}/refresh`,
      { method: 'POST' }
    );
  }

  async toggleActive(id: number): Promise<{ data: WebsitePreview; success: boolean }> {
    return this.request<{ data: WebsitePreview; success: boolean }>(
      `${this.getBaseUrl()}/${id}/toggle-active`,
      { method: 'PATCH' }
    );
  }
}

export const websitePreviewService = new WebsitePreviewService();
