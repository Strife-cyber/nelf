import { getAuth, type Auth } from 'firebase/auth'
import { initializeApp, type FirebaseApp } from 'firebase/app'
import { getFirestore, type Firestore } from 'firebase/firestore'
import { getStorage, type FirebaseStorage } from 'firebase/storage'

const requiredEnv = <K extends keyof ImportMetaEnv>(key: K): string => {
  const value = import.meta.env[key]
  if (!value) {
    // Vite inlines env vars at dev-server start / build time. If this is missing,
    // it's usually because the server wasn't restarted after editing .env,
    // or the var name/format is wrong (must start with VITE_ and use KEY=value).
    throw new Error(
      `[firebase] Missing env var ${String(key)}. ` +
        `Check your .env/.env.local (project root), ensure it is named correctly and uses "${String(key)}=...". ` +
        `Then restart the Vite dev server (or rebuild for production).`,
    )
  }
  return value
}

// Configuration Firebase depuis les variables d'environnement
const firebaseConfig = {
  apiKey: requiredEnv('VITE_FIREBASE_API_KEY'),
  authDomain: requiredEnv('VITE_FIREBASE_AUTH_DOMAIN'),
  projectId: requiredEnv('VITE_FIREBASE_PROJECT_ID'),
  storageBucket: requiredEnv('VITE_FIREBASE_STORAGE_BUCKET'),
  messagingSenderId: requiredEnv('VITE_FIREBASE_MESSAGING_SENDER_ID'),
  appId: requiredEnv('VITE_FIREBASE_APP_ID'),
  measurementId: requiredEnv('VITE_FIREBASE_MEASUREMENT_ID'),
}

// Initialisation de Firebase
let app: FirebaseApp | null = null
let auth: Auth | null = null
let db: Firestore | null = null
let storage: FirebaseStorage | null = null

// Initialiser Firebase
export const initFirebase = (): FirebaseApp => {
  if (!app) {
    app = initializeApp(firebaseConfig)
  }
  return app
}

// Obtenir l'instance d'authentification
export const getFirebaseAuth = (): Auth => {
  if (!auth) {
    if (!app) {
      initFirebase()
    }
    auth = getAuth(app!)
  }
  return auth
}

// Obtenir l'instance Firestore
export const getFirebaseFirestore = (): Firestore => {
  if (!db) {
    if (!app) {
      initFirebase()
    }
    db = getFirestore(app!)
  }
  return db
}

// Obtenir l'instance Storage
export const getFirebaseStorage = (): FirebaseStorage => {
  if (!storage) {
    if (!app) {
      initFirebase()
    }
    storage = getStorage(app!)
  }
  return storage
}

// Exporter l'instance principale de l'application
export const getFirebaseApp = (): FirebaseApp => {
  if (!app) {
    initFirebase()
  }
  return app!
}

export default {
  app: getFirebaseApp,
  auth: getFirebaseAuth,
  firestore: getFirebaseFirestore,
  storage: getFirebaseStorage,
}
