/**
 * Firestore Service
 * Handles all Firestore operations for the admin dashboard
 */

import {
  collection,
  doc,
  getDoc,
  getDocs,
  addDoc,
  updateDoc,
  deleteDoc,
  query,
  orderBy,
  where,
  type QueryConstraint,
  serverTimestamp,
} from 'firebase/firestore'
import { getFirebaseFirestore } from './firebase'
import type { Flyer, Video, WebsitePreview, TeamMember } from '@/types/models'

const db = getFirebaseFirestore()

// Collection names
const COLLECTIONS = {
  flyers: 'flyers',
  videos: 'videos',
  previews: 'websitePreviews',
  team: 'teamMembers',
} as const

/**
 * Generic CRUD operations
 */
async function create<T extends { id?: string }>(
  collectionName: string,
  data: Omit<T, 'id' | 'createdAt' | 'updatedAt'>,
): Promise<string> {
  const docRef = await addDoc(collection(db, collectionName), {
    ...data,
    createdAt: serverTimestamp(),
    updatedAt: serverTimestamp(),
  })
  return docRef.id
}

async function read<T>(collectionName: string, id: string): Promise<T | null> {
  const docRef = doc(db, collectionName, id)
  const docSnap = await getDoc(docRef)
  if (docSnap.exists()) {
    return { id: docSnap.id, ...docSnap.data() } as T
  }
  return null
}

async function readAll<T>(
  collectionName: string,
  constraints: QueryConstraint[] = [],
): Promise<T[]> {
  const q = query(collection(db, collectionName), ...constraints)
  const querySnapshot = await getDocs(q)
  return querySnapshot.docs.map((doc) => ({
    id: doc.id,
    ...doc.data(),
  })) as T[]
}

async function update<T extends { id: string }>(
  collectionName: string,
  id: string,
  data: Partial<Omit<T, 'id' | 'createdAt'>>,
): Promise<void> {
  const docRef = doc(db, collectionName, id)
  await updateDoc(docRef, {
    ...data,
    updatedAt: serverTimestamp(),
  })
}

async function remove(collectionName: string, id: string): Promise<void> {
  const docRef = doc(db, collectionName, id)
  await deleteDoc(docRef)
}

// Flyers
export const flyersService = {
  async create(data: Omit<Flyer, 'id' | 'createdAt' | 'updatedAt'>): Promise<string> {
    return create<Flyer>(COLLECTIONS.flyers, data)
  },

  async read(id: string): Promise<Flyer | null> {
    return read<Flyer>(COLLECTIONS.flyers, id)
  },

  async readAll(activeOnly: boolean = false): Promise<Flyer[]> {
    const constraints: QueryConstraint[] = [orderBy('uploadedAt', 'desc')]
    if (activeOnly) {
      constraints.unshift(where('isActive', '==', true))
    }
    return readAll<Flyer>(COLLECTIONS.flyers, constraints)
  },

  async update(id: string, data: Partial<Omit<Flyer, 'id' | 'createdAt'>>): Promise<void> {
    return update<Flyer>(COLLECTIONS.flyers, id, data)
  },

  async delete(id: string): Promise<void> {
    return remove(COLLECTIONS.flyers, id)
  },
}

// Videos
export const videosService = {
  async create(data: Omit<Video, 'id' | 'createdAt' | 'updatedAt'>): Promise<string> {
    return create<Video>(COLLECTIONS.videos, data)
  },

  async read(id: string): Promise<Video | null> {
    return read<Video>(COLLECTIONS.videos, id)
  },

  async readAll(activeOnly: boolean = false): Promise<Video[]> {
    const constraints: QueryConstraint[] = [orderBy('uploadedAt', 'desc')]
    if (activeOnly) {
      constraints.unshift(where('isActive', '==', true))
    }
    return readAll<Video>(COLLECTIONS.videos, constraints)
  },

  async update(id: string, data: Partial<Omit<Video, 'id' | 'createdAt'>>): Promise<void> {
    return update<Video>(COLLECTIONS.videos, id, data)
  },

  async delete(id: string): Promise<void> {
    return remove(COLLECTIONS.videos, id)
  },
}

// Website Previews
export const previewsService = {
  async create(data: Omit<WebsitePreview, 'id' | 'createdAt' | 'updatedAt'>): Promise<string> {
    return create<WebsitePreview>(COLLECTIONS.previews, data)
  },

  async read(id: string): Promise<WebsitePreview | null> {
    return read<WebsitePreview>(COLLECTIONS.previews, id)
  },

  async readAll(activeOnly: boolean = false): Promise<WebsitePreview[]> {
    const constraints: QueryConstraint[] = [orderBy('uploadedAt', 'desc')]
    if (activeOnly) {
      constraints.unshift(where('isActive', '==', true))
    }
    return readAll<WebsitePreview>(COLLECTIONS.previews, constraints)
  },

  async update(id: string, data: Partial<Omit<WebsitePreview, 'id' | 'createdAt'>>): Promise<void> {
    return update<WebsitePreview>(COLLECTIONS.previews, id, data)
  },

  async delete(id: string): Promise<void> {
    return remove(COLLECTIONS.previews, id)
  },
}

// Team Members
export const teamService = {
  async create(data: Omit<TeamMember, 'id' | 'createdAt' | 'updatedAt'>): Promise<string> {
    return create<TeamMember>(COLLECTIONS.team, data)
  },

  async read(id: string): Promise<TeamMember | null> {
    return read<TeamMember>(COLLECTIONS.team, id)
  },

  async readAll(activeOnly: boolean = false): Promise<TeamMember[]> {
    const constraints: QueryConstraint[] = []
    if (activeOnly) {
      constraints.push(where('isActive', '==', true))
    }
    // Order by name if no order field
    if (constraints.length === 0) {
      constraints.push(orderBy('name', 'asc'))
    }
    return readAll<TeamMember>(COLLECTIONS.team, constraints)
  },

  async update(id: string, data: Partial<Omit<TeamMember, 'id' | 'createdAt'>>): Promise<void> {
    return update<TeamMember>(COLLECTIONS.team, id, data)
  },

  async delete(id: string): Promise<void> {
    return remove(COLLECTIONS.team, id)
  },
}
