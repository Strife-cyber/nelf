import { Cloudinary } from '@cloudinary/url-gen'

// Configuration Cloudinary depuis les variables d'environnement
const apiKey = import.meta.env.VITE_CLOUDINARY_API_KEY
const cloudName = import.meta.env.VITE_CLOUDINARY_CLOUD_NAME
const apiSecret = import.meta.env.VITE_CLOUDINARY_API_SECRET

// Instance Cloudinary pour la génération d'URLs (côté client)
let cloudinaryInstance: Cloudinary | null = null

// Initialiser Cloudinary pour la génération d'URLs
export const initCloudinary = (): Cloudinary => {
  if (!cloudinaryInstance) {
    if (!cloudName) {
      throw new Error("VITE_CLOUDINARY_CLOUD_NAME est manquant dans les variables d'environnement")
    }
    cloudinaryInstance = new Cloudinary({
      cloud: {
        cloudName,
      },
    })
  }
  return cloudinaryInstance
}

// Obtenir l'instance Cloudinary
export const getCloudinary = (): Cloudinary => {
  if (!cloudinaryInstance) {
    return initCloudinary()
  }
  return cloudinaryInstance
}

// Configuration pour les opérations côté serveur (si nécessaire)
export const getCloudinaryConfig = () => {
  if (!cloudName || !apiKey || !apiSecret) {
    throw new Error("Les variables d'environnement Cloudinary sont manquantes")
  }
  return {
    cloudName,
    apiKey,
    apiSecret,
  }
}

export default {
  instance: getCloudinary,
  config: getCloudinaryConfig,
}
