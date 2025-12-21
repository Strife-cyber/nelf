/**
 * Flyer Model
 * Represents a promotional flyer/poster
 */
export interface Flyer {
  id: string
  name: string
  url: string
  thumbnailUrl?: string
  uploadedAt: string
  uploadedBy?: string
  eventTitle?: string
  description?: string
  shortInfo?: string
  tags?: string[]
  order?: number
  isActive?: boolean
  createdAt?: string
  updatedAt?: string
}

/**
 * Video Model
 * Represents a video content
 */
export interface Video {
  id: string
  name: string
  url: string
  thumbnailUrl?: string
  duration?: number // in seconds
  uploadedAt: string
  uploadedBy?: string
  eventTitle?: string
  description?: string
  shortInfo?: string
  tags?: string[]
  order?: number
  isActive?: boolean
  videoType?: 'youtube' | 'vimeo' | 'direct' | 'embedded'
  videoId?: string // for YouTube/Vimeo
  createdAt?: string
  updatedAt?: string
}

/**
 * Website Preview Model
 * Represents a website preview/showcase
 */
export interface WebsitePreview {
  id: string
  name: string
  url: string
  thumbnailUrl: string
  uploadedAt: string
  uploadedBy?: string
  description?: string
  tags?: string[]
  order?: number
  isActive?: boolean
  technologies?: string[] // e.g., ['Vue.js', 'Node.js', 'Firebase']
  clientName?: string
  projectType?: 'portfolio' | 'ecommerce' | 'corporate' | 'landing' | 'other'
  createdAt?: string
  updatedAt?: string
}

/**
 * Team Member Model
 * Represents a team member
 */
export interface TeamMember {
  id: string
  name: string
  role: string
  description: string
  avatar?: string // URL to avatar image
  avatarInitials?: string // Fallback initials if no avatar
  email?: string
  phone?: string
  socialLinks?: {
    linkedin?: string
    github?: string
    twitter?: string
    portfolio?: string
  }
  order?: number
  isActive?: boolean
  skills?: string[]
  createdAt?: string
  updatedAt?: string
}

/**
 * Base model with common fields
 */
export interface BaseModel {
  id: string
  createdAt: string
  updatedAt: string
  createdBy?: string
  updatedBy?: string
}

/**
 * Firestore document reference
 */
export interface FirestoreDocument<T> {
  id: string
  data: T
  path: string
}
