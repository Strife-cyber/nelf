/**
 * Cloudinary Upload Service
 * Handles file uploads to Cloudinary
 */

const cloudName = import.meta.env.VITE_CLOUDINARY_CLOUD_NAME
const uploadPreset = import.meta.env.VITE_CLOUDINARY_UPLOAD_PRESET || 'nelf_uploads'

if (!cloudName) {
  console.warn('VITE_CLOUDINARY_CLOUD_NAME is missing. Uploads will not work.')
}

export interface UploadResult {
  url: string
  publicId: string
  secureUrl: string
  width?: number
  height?: number
  format?: string
  bytes?: number
}

/**
 * Upload a file to Cloudinary
 * @param file - File to upload
 * @param folder - Optional folder path in Cloudinary
 * @param resourceType - 'image', 'video', or 'auto'
 * @returns Promise with upload result
 */
export async function uploadToCloudinary(
  file: File,
  folder: string = 'nelf',
  resourceType: 'image' | 'video' | 'auto' = 'auto',
): Promise<UploadResult> {
  if (!cloudName) {
    throw new Error('Cloudinary cloud name is not configured')
  }

  const formData = new FormData()
  formData.append('file', file)
  formData.append('upload_preset', uploadPreset)
  formData.append('folder', folder)

  // Auto-detect resource type if not specified
  if (resourceType === 'auto') {
    if (file.type.startsWith('image/')) {
      resourceType = 'image'
    } else if (file.type.startsWith('video/')) {
      resourceType = 'video'
    } else {
      resourceType = 'image' // default
    }
  }

  const uploadUrl = `https://api.cloudinary.com/v1_1/${cloudName}/${resourceType}/upload`

  try {
    const response = await fetch(uploadUrl, {
      method: 'POST',
      body: formData,
    })

    if (!response.ok) {
      const error = await response.json()
      throw new Error(error.error?.message || 'Upload failed')
    }

    const data = await response.json()

    return {
      url: data.secure_url || data.url,
      publicId: data.public_id,
      secureUrl: data.secure_url,
      width: data.width,
      height: data.height,
      format: data.format,
      bytes: data.bytes,
    }
  } catch (error) {
    console.error('Cloudinary upload error:', error)
    throw error
  }
}

/**
 * Upload an image to Cloudinary
 */
export async function uploadImage(
  file: File,
  folder: string = 'nelf/flyers',
): Promise<UploadResult> {
  return uploadToCloudinary(file, folder, 'image')
}

/**
 * Upload a video to Cloudinary
 */
export async function uploadVideo(
  file: File,
  folder: string = 'nelf/videos',
): Promise<UploadResult> {
  return uploadToCloudinary(file, folder, 'video')
}

/**
 * Generate a thumbnail URL from a Cloudinary public ID
 */
export function getThumbnailUrl(publicId: string, width: number = 300): string {
  if (!cloudName) return ''
  return `https://res.cloudinary.com/${cloudName}/image/upload/w_${width},h_${width},c_fill/${publicId}`
}
