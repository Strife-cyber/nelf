import { getAuth, type Auth } from 'firebase/auth'
import { initializeApp, type FirebaseApp } from 'firebase/app'
import { getFirestore, type Firestore } from 'firebase/firestore'
import { getStorage, type FirebaseStorage } from 'firebase/storage'

// Configuration Firebase depuis les variables d'environnement
const firebaseConfig = {
  apiKey: import.meta.env.VITE_FIREBASE_API_KEY,
  authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN,
  projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID,
  storageBucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID,
  appId: import.meta.env.VITE_FIREBASE_APP_ID,
  measurementId: import.meta.env.VITE_FIREBASE_MEASUREMENT_ID,
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
