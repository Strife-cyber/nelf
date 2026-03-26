// =============================================================================
// Project:        Nelf
// Description:    TypeScript types based on database schema
// =============================================================================

// =============================================================================
// Base Types
// =============================================================================

export interface BaseEntity {
  id: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

// =============================================================================
// User Types
// =============================================================================

export interface User extends BaseEntity {
  name: string;
  email: string;
  phone?: string;
  role?: string;
  description?: string;
  avatar_url?: string;
  initials?: string;
  skills?: string[];
}

export interface CreateUserRequest {
  name: string;
  email: string;
  phone?: string;
  role?: string;
  description?: string;
  avatar_url?: string;
  initials?: string;
  skills?: string[];
}

export interface UpdateUserRequest extends Partial<CreateUserRequest> {
  id: number;
}

// =============================================================================
// Video Types
// =============================================================================

export interface Video extends BaseEntity {
  name: string;
  description?: string;
  event_title?: string;
  short_info?: string;
  thumbnail_url?: string;
  url: string;
}

export interface CreateVideoRequest {
  name: string;
  description?: string;
  event_title?: string;
  short_info?: string;
  thumbnail_url?: string;
  url: string;
}

export interface UpdateVideoRequest extends Partial<CreateVideoRequest> {
  id: number;
}

// =============================================================================
// Flyer Types
// =============================================================================

export interface Flyer extends BaseEntity {
  name: string;
  description?: string;
  event_title?: string;
  short_info?: string;
  url: string;
}

export interface CreateFlyerRequest {
  name: string;
  description?: string;
  event_title?: string;
  short_info?: string;
  url: string;
}

export interface UpdateFlyerRequest extends Partial<CreateFlyerRequest> {
  id: number;
}

// =============================================================================
// Website Preview Types
// =============================================================================

export interface WebsitePreview {
  id: number;
  url: string;
  title?: string;
  description?: string;
  short_info?: string;
  image_url?: string;
  favicon_url?: string;
  content_type: string;
  is_active: boolean;
  retrieved_at: string;
}

export interface CreateWebsitePreviewRequest {
  url: string;
  title?: string;
  description?: string;
  short_info?: string;
  image_url?: string;
  favicon_url?: string;
  content_type: string;
}

export interface UpdateWebsitePreviewRequest extends Partial<CreateWebsitePreviewRequest> {
  id: number;
}

// =============================================================================
// API Response Types
// =============================================================================

export interface ApiResponse<T> {
  data: T;
  message?: string;
  success: boolean;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  success: boolean;
}

export interface ApiError {
  message: string;
  status: number;
  details?: any;
}
