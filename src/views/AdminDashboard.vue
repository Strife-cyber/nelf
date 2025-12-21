<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { onAuthStateChanged, signOut, type User } from 'firebase/auth'
import { getFirebaseAuth } from '@/services/firebase'
import type { Flyer, Video, WebsitePreview, TeamMember } from '@/types/models'
import { flyersService, videosService, previewsService, teamService } from '@/services/firestore'
import { uploadImage, uploadVideo, getThumbnailUrl } from '@/services/cloudinary-upload'
import { trimVideoTo10MB } from '@/services/video-trimmer'

const router = useRouter()
const auth = getFirebaseAuth()

const user = ref<User | null>(null)
const isLoading = ref(false)
const isUploading = ref(false)
const isProcessingVideo = ref(false)
const activeTab = ref<'flyers' | 'videos' | 'previews' | 'team'>('flyers')
const errorMsg = ref<string | null>(null)

// Data from Firestore
const flyers = ref<Flyer[]>([])
const videos = ref<Video[]>([])
const previews = ref<WebsitePreview[]>([])
const teamMembers = ref<TeamMember[]>([])

// Modal states
const showFlyerModal = ref(false)
const showVideoModal = ref(false)
const showPreviewModal = ref(false)
const showTeamModal = ref(false)
const showDeleteConfirm = ref(false)
const deleteItemType = ref<'flyers' | 'videos' | 'previews' | 'team' | null>(null)
const deleteItemId = ref<string | null>(null)

// Form data
const flyerForm = ref({
  name: '',
  eventTitle: '',
  description: '',
  shortInfo: '',
  tags: '',
  file: null as File | null,
})

const videoForm = ref({
  name: '',
  eventTitle: '',
  description: '',
  shortInfo: '',
  tags: '',
  file: null as File | null,
})

const previewForm = ref({
  name: '',
  url: '',
  description: '',
  technologies: '',
  thumbnailFile: null as File | null,
})

const teamForm = ref({
  name: '',
  role: '',
  description: '',
  email: '',
  phone: '',
  linkedin: '',
  github: '',
  portfolio: '',
  skills: '',
  avatarFile: null as File | null,
})

let unsubscribe: (() => void) | null = null

onMounted(async () => {
  unsubscribe = onAuthStateChanged(auth, async (u) => {
    user.value = u
    if (!u) {
      router.push({ name: 'admin-login' })
    } else {
      // Load all data when authenticated
      await loadAllData()
    }
  })
})

onUnmounted(() => {
  unsubscribe?.()
  unsubscribe = null
})

async function loadAllData() {
  isLoading.value = true
  errorMsg.value = null
  try {
    await Promise.all([loadFlyers(), loadVideos(), loadPreviews(), loadTeamMembers()])
  } catch (error) {
    console.error('Error loading data:', error)
    errorMsg.value = 'Erreur lors du chargement des données'
  } finally {
    isLoading.value = false
  }
}

async function loadFlyers() {
  try {
    flyers.value = await flyersService.readAll()
  } catch (error) {
    console.error('Error loading flyers:', error)
    throw error
  }
}

async function loadVideos() {
  try {
    videos.value = await videosService.readAll()
  } catch (error) {
    console.error('Error loading videos:', error)
    throw error
  }
}

async function loadPreviews() {
  try {
    previews.value = await previewsService.readAll()
  } catch (error) {
    console.error('Error loading previews:', error)
    throw error
  }
}

async function loadTeamMembers() {
  try {
    teamMembers.value = await teamService.readAll()
  } catch (error) {
    console.error('Error loading team members:', error)
    throw error
  }
}

async function handleLogout() {
  isLoading.value = true
  try {
    await signOut(auth)
    router.push({ name: 'admin-login' })
  } catch (error) {
    console.error('Logout error:', error)
    errorMsg.value = 'Erreur lors de la déconnexion'
  } finally {
    isLoading.value = false
  }
}

// Helper function to remove undefined values from objects (Firestore doesn't support undefined)
function removeUndefined<T extends Record<string, unknown>>(obj: T): Partial<T> {
  const cleaned: Partial<T> = {}
  for (const key in obj) {
    if (obj[key] !== undefined) {
      cleaned[key] = obj[key]
    }
  }
  return cleaned
}

// File upload handlers
function handleFlyerUpload() {
  // Reset form
  flyerForm.value = {
    name: '',
    eventTitle: '',
    description: '',
    shortInfo: '',
    tags: '',
    file: null,
  }
  showFlyerModal.value = true
}

async function handleFlyerSubmit() {
  if (!flyerForm.value.name || !flyerForm.value.file) {
    errorMsg.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  isUploading.value = true
  errorMsg.value = null
  showFlyerModal.value = false

  try {
    // Upload to Cloudinary
    const uploadResult = await uploadImage(flyerForm.value.file, 'nelf/flyers')
    const thumbnailUrl = getThumbnailUrl(uploadResult.publicId, 300)

    // Parse tags
    const tags = flyerForm.value.tags
      ? flyerForm.value.tags
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean)
      : undefined

    // Save to Firestore - remove undefined values
    const flyerData = removeUndefined({
      name: flyerForm.value.name,
      url: uploadResult.secureUrl,
      thumbnailUrl,
      uploadedAt: new Date().toISOString(),
      uploadedBy: user.value?.email || user.value?.uid,
      eventTitle: flyerForm.value.eventTitle || undefined,
      description: flyerForm.value.description || undefined,
      shortInfo: flyerForm.value.shortInfo || undefined,
      tags,
      isActive: true,
    })

    await flyersService.create(flyerData as Omit<Flyer, 'id' | 'createdAt' | 'updatedAt'>)
    await loadFlyers()
  } catch (error) {
    console.error('Error uploading flyer:', error)
    errorMsg.value = "Erreur lors de l'upload de l'affiche"
  } finally {
    isUploading.value = false
  }
}

function handleVideoUpload() {
  // Reset form
  videoForm.value = {
    name: '',
    eventTitle: '',
    description: '',
    shortInfo: '',
    tags: '',
    file: null,
  }
  showVideoModal.value = true
}

async function handleVideoFileChange(e: Event) {
  const target = e.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    isProcessingVideo.value = true
    errorMsg.value = null
    try {
      // Check if file is larger than 10MB and trim if necessary
      if (file.size > 10 * 1024 * 1024) {
        errorMsg.value = 'Vidéo trop grande, traitement en cours pour optimiser...'
        videoForm.value.file = await trimVideoTo10MB(file)
        errorMsg.value = null
      } else {
        videoForm.value.file = file
      }
    } catch (error) {
      console.error('Error processing video:', error)
      errorMsg.value = 'Erreur lors du traitement de la vidéo. Veuillez réessayer.'
      videoForm.value.file = null
    } finally {
      isProcessingVideo.value = false
    }
  }
}

async function handleVideoSubmit() {
  if (!videoForm.value.name || !videoForm.value.file) {
    errorMsg.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  isUploading.value = true
  errorMsg.value = null
  showVideoModal.value = false

  try {
    // Upload to Cloudinary
    const uploadResult = await uploadVideo(videoForm.value.file, 'nelf/videos')
    const thumbnailUrl = getThumbnailUrl(uploadResult.publicId, 300)

    // Parse tags
    const tags = videoForm.value.tags
      ? videoForm.value.tags
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean)
      : undefined

    // Save to Firestore - remove undefined values
    const videoData = removeUndefined({
      name: videoForm.value.name,
      url: uploadResult.secureUrl,
      thumbnailUrl,
      uploadedAt: new Date().toISOString(),
      uploadedBy: user.value?.email || user.value?.uid,
      eventTitle: videoForm.value.eventTitle || undefined,
      description: videoForm.value.description || undefined,
      shortInfo: videoForm.value.shortInfo || undefined,
      tags,
      videoType: 'direct',
      isActive: true,
    })

    await videosService.create(videoData as Omit<Video, 'id' | 'createdAt' | 'updatedAt'>)
    await loadVideos()
  } catch (error) {
    console.error('Error uploading video:', error)
    errorMsg.value = "Erreur lors de l'upload de la vidéo"
  } finally {
    isUploading.value = false
  }
}

function handlePreviewUpload() {
  // Reset form
  previewForm.value = {
    name: '',
    url: '',
    description: '',
    technologies: '',
    thumbnailFile: null,
  }
  showPreviewModal.value = true
}

async function handlePreviewSubmit() {
  if (!previewForm.value.name || !previewForm.value.url || !previewForm.value.thumbnailFile) {
    errorMsg.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  isUploading.value = true
  errorMsg.value = null
  showPreviewModal.value = false

  try {
    // Upload thumbnail to Cloudinary
    const uploadResult = await uploadImage(previewForm.value.thumbnailFile, 'nelf/previews')
    const thumbnailUrl = uploadResult.secureUrl

    // Parse technologies
    const technologies = previewForm.value.technologies
      ? previewForm.value.technologies
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean)
      : undefined

    // Save to Firestore - remove undefined values
    const previewData = removeUndefined({
      name: previewForm.value.name,
      url: previewForm.value.url,
      thumbnailUrl,
      uploadedAt: new Date().toISOString(),
      uploadedBy: user.value?.email || user.value?.uid,
      description: previewForm.value.description || undefined,
      technologies,
      isActive: true,
    })

    await previewsService.create(
      previewData as Omit<WebsitePreview, 'id' | 'createdAt' | 'updatedAt'>,
    )
    await loadPreviews()
  } catch (error) {
    console.error('Error uploading preview:', error)
    errorMsg.value = "Erreur lors de l'upload de l'aperçu"
  } finally {
    isUploading.value = false
  }
}

function handleAddTeamMember() {
  // Reset form
  teamForm.value = {
    name: '',
    role: '',
    description: '',
    email: '',
    phone: '',
    linkedin: '',
    github: '',
    portfolio: '',
    skills: '',
    avatarFile: null,
  }
  showTeamModal.value = true
}

async function handleTeamSubmit() {
  if (!teamForm.value.name || !teamForm.value.role || !teamForm.value.description) {
    errorMsg.value = 'Veuillez remplir tous les champs obligatoires'
    return
  }

  isUploading.value = true
  errorMsg.value = null
  showTeamModal.value = false

  try {
    // Upload avatar if provided
    let avatarUrl: string | undefined
    if (teamForm.value.avatarFile) {
      const uploadResult = await uploadImage(teamForm.value.avatarFile, 'nelf/team')
      avatarUrl = uploadResult.secureUrl
    }

    // Generate initials
    const initials = teamForm.value.name
      .split(' ')
      .map((n) => n[0])
      .join('')
      .toUpperCase()
      .slice(0, 2)

    // Parse skills
    const skills = teamForm.value.skills
      ? teamForm.value.skills
          .split(',')
          .map((s) => s.trim())
          .filter(Boolean)
      : undefined

    // Build social links
    const socialLinks: TeamMember['socialLinks'] = {}
    if (teamForm.value.linkedin) socialLinks.linkedin = teamForm.value.linkedin
    if (teamForm.value.github) socialLinks.github = teamForm.value.github
    if (teamForm.value.portfolio) socialLinks.portfolio = teamForm.value.portfolio

    // Save to Firestore - remove undefined values
    const memberData = removeUndefined({
      name: teamForm.value.name,
      role: teamForm.value.role,
      description: teamForm.value.description,
      email: teamForm.value.email || undefined,
      phone: teamForm.value.phone || undefined,
      avatar: avatarUrl,
      avatarInitials: initials,
      socialLinks: Object.keys(socialLinks).length > 0 ? socialLinks : undefined,
      skills,
      isActive: true,
    })

    await teamService.create(memberData as Omit<TeamMember, 'id' | 'createdAt' | 'updatedAt'>)
    await loadTeamMembers()
  } catch (error) {
    console.error('Error creating team member:', error)
    errorMsg.value = 'Erreur lors de la création du membre'
  } finally {
    isUploading.value = false
  }
}

function handleDeleteItem(type: 'flyers' | 'videos' | 'previews' | 'team', id: string) {
  deleteItemType.value = type
  deleteItemId.value = id
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  if (!deleteItemType.value || !deleteItemId.value) return

  isLoading.value = true
  errorMsg.value = null
  showDeleteConfirm.value = false

  try {
    if (deleteItemType.value === 'flyers') {
      await flyersService.delete(deleteItemId.value)
      await loadFlyers()
    } else if (deleteItemType.value === 'videos') {
      await videosService.delete(deleteItemId.value)
      await loadVideos()
    } else if (deleteItemType.value === 'previews') {
      await previewsService.delete(deleteItemId.value)
      await loadPreviews()
    } else if (deleteItemType.value === 'team') {
      await teamService.delete(deleteItemId.value)
      await loadTeamMembers()
    }
  } catch (error) {
    console.error('Error deleting item:', error)
    errorMsg.value = 'Erreur lors de la suppression'
  } finally {
    isLoading.value = false
    deleteItemType.value = null
    deleteItemId.value = null
  }
}
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-[#2b2d75] via-[#4c2e6c] to-[#2b2d75]">
    <!-- Header -->
    <header class="border-b border-white/20 bg-white/5 backdrop-blur-md px-6 py-4">
      <div class="max-w-7xl mx-auto flex items-center justify-between">
        <div class="flex items-center gap-4">
          <h1 class="text-2xl font-bold text-white">Tableau de Bord Admin</h1>
          <span class="text-sm text-white/60">Bienvenue, {{ user?.email }}</span>
        </div>
        <div class="flex items-center gap-4">
          <button
            class="px-4 py-2 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all text-sm font-semibold"
            @click="router.push({ name: 'home' })"
          >
            Voir le site
          </button>
          <button
            class="px-4 py-2 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all text-sm font-semibold disabled:opacity-60"
            :disabled="isLoading"
            @click="handleLogout"
          >
            Déconnexion
          </button>
        </div>
      </div>
    </header>

    <!-- Error Message -->
    <div v-if="errorMsg" class="max-w-7xl mx-auto px-6 pt-4">
      <div class="rounded-xl bg-red-500/20 border border-red-500/30 p-4">
        <p class="text-sm text-red-200 font-medium">{{ errorMsg }}</p>
      </div>
    </div>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-6 py-8">
      <!-- Loading State -->
      <div v-if="isLoading && flyers.length === 0" class="flex items-center justify-center py-20">
        <div class="text-white/60">Chargement...</div>
      </div>

      <!-- Tabs -->
      <div
        v-else
        class="mb-8 flex items-center gap-2 p-1 rounded-xl bg-white/5 border border-white/10 w-fit"
      >
        <button
          :class="[
            'px-6 py-2.5 rounded-lg text-sm font-semibold transition-all',
            activeTab === 'flyers'
              ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
              : 'text-white/60 hover:text-white/80',
          ]"
          @click="activeTab = 'flyers'"
        >
          Affiches
        </button>
        <button
          :class="[
            'px-6 py-2.5 rounded-lg text-sm font-semibold transition-all',
            activeTab === 'videos'
              ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
              : 'text-white/60 hover:text-white/80',
          ]"
          @click="activeTab = 'videos'"
        >
          Vidéos
        </button>
        <button
          :class="[
            'px-6 py-2.5 rounded-lg text-sm font-semibold transition-all',
            activeTab === 'previews'
              ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
              : 'text-white/60 hover:text-white/80',
          ]"
          @click="activeTab = 'previews'"
        >
          Aperçus Web
        </button>
        <button
          :class="[
            'px-6 py-2.5 rounded-lg text-sm font-semibold transition-all',
            activeTab === 'team'
              ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
              : 'text-white/60 hover:text-white/80',
          ]"
          @click="activeTab = 'team'"
        >
          Équipe
        </button>
      </div>

      <!-- Flyers Tab -->
      <div v-if="activeTab === 'flyers'" class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-white">Gestion des Affiches ({{ flyers.length }})</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
            :disabled="isUploading"
            @click="handleFlyerUpload"
          >
            {{ isUploading ? 'Upload en cours...' : '+ Ajouter une affiche' }}
          </button>
        </div>
        <div v-if="flyers.length === 0" class="text-center py-12 text-white/60">
          Aucune affiche pour le moment
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="flyer in flyers"
            :key="flyer.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-4 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="aspect-[3/4] rounded-xl bg-white/5 mb-4 overflow-hidden">
              <img
                :src="flyer.thumbnailUrl || flyer.url"
                :alt="flyer.name"
                class="w-full h-full object-cover"
              />
            </div>
            <div class="flex items-center justify-between">
              <div class="flex-1">
                <h3 class="font-semibold text-white">{{ flyer.name }}</h3>
                <p v-if="flyer.eventTitle" class="text-sm text-[#c62d6a] mt-1 font-medium">
                  {{ flyer.eventTitle }}
                </p>
                <p class="text-xs text-white/60 mt-1">
                  {{ new Date(flyer.uploadedAt).toLocaleDateString('fr-FR') }}
                </p>
                <p v-if="flyer.shortInfo" class="text-xs text-white/70 mt-1">
                  {{ flyer.shortInfo }}
                </p>
                <p v-if="flyer.description" class="text-xs text-white/50 mt-1 line-clamp-2">
                  {{ flyer.description }}
                </p>
              </div>
              <button
                class="px-3 py-1.5 rounded-lg bg-red-500/20 border border-red-500/30 text-red-200 hover:bg-red-500/30 text-sm transition-all"
                @click="handleDeleteItem('flyers', flyer.id)"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Videos Tab -->
      <div v-if="activeTab === 'videos'" class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-white">Gestion des Vidéos ({{ videos.length }})</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
            :disabled="isUploading"
            @click="handleVideoUpload"
          >
            {{ isUploading ? 'Upload en cours...' : '+ Ajouter une vidéo' }}
          </button>
        </div>
        <div v-if="videos.length === 0" class="text-center py-12 text-white/60">
          Aucune vidéo pour le moment
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div
            v-for="video in videos"
            :key="video.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div
              class="aspect-video rounded-xl bg-white/5 mb-4 flex items-center justify-center overflow-hidden"
            >
              <video
                v-if="video.thumbnailUrl"
                :src="video.url"
                :poster="video.thumbnailUrl"
                class="w-full h-full object-cover"
                controls
              ></video>
              <svg v-else class="w-16 h-16 text-white/40" fill="currentColor" viewBox="0 0 24 24">
                <path d="M8 5v14l11-7z" />
              </svg>
            </div>
            <div class="flex items-center justify-between">
              <div class="flex-1">
                <h3 class="font-semibold text-white">{{ video.name }}</h3>
                <p v-if="video.eventTitle" class="text-sm text-[#c62d6a] mt-1 font-medium">
                  {{ video.eventTitle }}
                </p>
                <p class="text-xs text-white/60 mt-1">
                  {{ new Date(video.uploadedAt).toLocaleDateString('fr-FR') }}
                </p>
                <p v-if="video.shortInfo" class="text-xs text-white/70 mt-1">
                  {{ video.shortInfo }}
                </p>
                <p v-if="video.description" class="text-sm text-white/70 mt-2 line-clamp-2">
                  {{ video.description }}
                </p>
              </div>
              <button
                class="px-3 py-1.5 rounded-lg bg-red-500/20 border border-red-500/30 text-red-200 hover:bg-red-500/30 text-sm transition-all"
                @click="handleDeleteItem('videos', video.id)"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Previews Tab -->
      <div v-if="activeTab === 'previews'" class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-white">
            Gestion des Aperçus Web ({{ previews.length }})
          </h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
            :disabled="isUploading"
            @click="handlePreviewUpload"
          >
            {{ isUploading ? 'Upload en cours...' : '+ Ajouter un aperçu' }}
          </button>
        </div>
        <div v-if="previews.length === 0" class="text-center py-12 text-white/60">
          Aucun aperçu web pour le moment
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div
            v-for="preview in previews"
            :key="preview.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="aspect-video rounded-xl bg-white/5 mb-4 overflow-hidden">
              <img
                :src="preview.thumbnailUrl"
                :alt="preview.name"
                class="w-full h-full object-cover"
              />
            </div>
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-white">{{ preview.name }}</h3>
                <p class="text-xs text-white/60 mt-1">
                  {{ new Date(preview.uploadedAt).toLocaleDateString('fr-FR') }}
                </p>
                <a
                  :href="preview.url"
                  target="_blank"
                  class="text-sm text-[#c62d6a] hover:underline mt-2 block"
                >
                  {{ preview.url }}
                </a>
                <div
                  v-if="preview.technologies && preview.technologies.length > 0"
                  class="flex flex-wrap gap-1 mt-2"
                >
                  <span
                    v-for="tech in preview.technologies"
                    :key="tech"
                    class="px-2 py-0.5 rounded text-xs bg-white/10 text-white/80"
                  >
                    {{ tech }}
                  </span>
                </div>
              </div>
              <button
                class="px-3 py-1.5 rounded-lg bg-red-500/20 border border-red-500/30 text-red-200 hover:bg-red-500/30 text-sm transition-all"
                @click="handleDeleteItem('previews', preview.id)"
              >
                Supprimer
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Team Tab -->
      <div v-if="activeTab === 'team'" class="space-y-6">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-white">
            Gestion de l'Équipe ({{ teamMembers.length }})
          </h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
            :disabled="isUploading"
            @click="handleAddTeamMember"
          >
            {{ isUploading ? 'Traitement...' : '+ Ajouter un membre' }}
          </button>
        </div>
        <div v-if="teamMembers.length === 0" class="text-center py-12 text-white/60">
          Aucun membre d'équipe pour le moment
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="member in teamMembers"
            :key="member.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="flex items-center gap-4 mb-4">
              <div
                v-if="member.avatar"
                class="w-16 h-16 rounded-full bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] flex items-center justify-center text-white font-bold text-xl overflow-hidden"
              >
                <img :src="member.avatar" :alt="member.name" class="w-full h-full object-cover" />
              </div>
              <div
                v-else
                class="w-16 h-16 rounded-full bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] flex items-center justify-center text-white font-bold text-xl"
              >
                {{ member.avatarInitials || member.name.charAt(0).toUpperCase() }}
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-white">{{ member.name }}</h3>
                <p class="text-sm text-[#c62d6a] mt-1">{{ member.role }}</p>
              </div>
            </div>
            <p class="text-sm text-white/80 mb-4">{{ member.description }}</p>
            <div v-if="member.skills && member.skills.length > 0" class="flex flex-wrap gap-1 mb-4">
              <span
                v-for="skill in member.skills"
                :key="skill"
                class="px-2 py-0.5 rounded text-xs bg-white/10 text-white/80"
              >
                {{ skill }}
              </span>
            </div>
            <button
              class="w-full px-4 py-2 rounded-lg bg-red-500/20 border border-red-500/30 text-red-200 hover:bg-red-500/30 text-sm transition-all"
              @click="handleDeleteItem('team', member.id)"
            >
              Supprimer
            </button>
          </div>
        </div>
      </div>
    </main>

    <!-- Flyer Modal -->
    <div
      v-if="showFlyerModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="showFlyerModal = false"
    >
      <div
        class="w-full max-w-2xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#2b2d75] to-[#4c2e6c] shadow-2xl flex flex-col max-h-[80vh]"
      >
        <div class="flex items-center justify-between p-8 pb-4 border-b border-white/10">
          <h2 class="text-2xl font-bold text-white">Ajouter une Affiche</h2>
          <button
            class="text-white/60 hover:text-white transition-colors"
            @click="showFlyerModal = false"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <form @submit.prevent="handleFlyerSubmit" class="flex-1 overflow-y-auto p-8 space-y-4">
          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Nom *</label>
            <input
              v-model="flyerForm.name"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              required
              placeholder="Nom de l'affiche"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Titre de l'événement</label
            >
            <input
              v-model="flyerForm.eventTitle"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Titre de l'événement"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Fichier *</label>
            <input
              type="file"
              accept="image/*"
              required
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-semibold file:bg-[#c62d6a] file:text-white hover:file:bg-[#d63d7a] transition-all"
              @change="
                (e) => {
                  const file = (e.target as HTMLInputElement).files?.[0]
                  if (file) flyerForm.file = file
                }
              "
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Description</label>
            <textarea
              v-model="flyerForm.description"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all resize-none"
              rows="3"
              placeholder="Description de l'affiche..."
            ></textarea>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Informations courtes</label
            >
            <input
              v-model="flyerForm.shortInfo"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Informations courtes..."
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Tags (séparés par des virgules)</label
            >
            <input
              v-model="flyerForm.tags"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="tag1, tag2, tag3"
            />
          </div>

          <div class="flex items-center gap-4 pt-4 border-t border-white/10">
            <button
              type="button"
              class="flex-1 px-6 py-3 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all font-semibold"
              @click="showFlyerModal = false"
            >
              Annuler
            </button>
            <button
              type="submit"
              class="flex-1 px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
              :disabled="isUploading"
            >
              {{ isUploading ? 'Enregistrement...' : 'Enregistrer' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Video Modal -->
    <div
      v-if="showVideoModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="showVideoModal = false"
    >
      <div
        class="w-full max-w-2xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#2b2d75] to-[#4c2e6c] shadow-2xl flex flex-col max-h-[80vh]"
      >
        <div class="flex items-center justify-between p-8 pb-4 border-b border-white/10">
          <h2 class="text-2xl font-bold text-white">Ajouter une Vidéo</h2>
          <button
            class="text-white/60 hover:text-white transition-colors"
            @click="showVideoModal = false"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <form @submit.prevent="handleVideoSubmit" class="flex-1 overflow-y-auto p-8 space-y-4">
          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Nom *</label>
            <input
              v-model="videoForm.name"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              required
              placeholder="Nom de la vidéo"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Titre de l'événement</label
            >
            <input
              v-model="videoForm.eventTitle"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Titre de l'événement"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Fichier *</label>
            <input
              type="file"
              accept="video/*"
              required
              :disabled="isProcessingVideo"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-semibold file:bg-[#c62d6a] file:text-white hover:file:bg-[#d63d7a] transition-all disabled:opacity-50"
              @change="handleVideoFileChange"
            />
            <p v-if="isProcessingVideo" class="mt-2 text-sm text-white/60">
              Traitement de la vidéo en cours... Cela peut prendre quelques instants.
            </p>
            <p v-if="videoForm.file" class="mt-2 text-xs text-white/50">
              Fichier sélectionné: {{ videoForm.file.name }} 
              ({{ (videoForm.file.size / 1024 / 1024).toFixed(2) }} MB)
            </p>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Description</label>
            <textarea
              v-model="videoForm.description"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all resize-none"
              rows="3"
              placeholder="Description de la vidéo..."
            ></textarea>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Informations courtes</label
            >
            <input
              v-model="videoForm.shortInfo"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Informations courtes..."
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Tags (séparés par des virgules)</label
            >
            <input
              v-model="videoForm.tags"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="tag1, tag2, tag3"
            />
          </div>

          <div class="flex items-center gap-4 pt-4 border-t border-white/10">
            <button
              type="button"
              class="flex-1 px-6 py-3 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all font-semibold"
              @click="showVideoModal = false"
            >
              Annuler
            </button>
            <button
              type="submit"
              class="flex-1 px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
              :disabled="isUploading || isProcessingVideo"
            >
              {{ isUploading ? 'Enregistrement...' : isProcessingVideo ? 'Traitement...' : 'Enregistrer' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Preview Modal -->
    <div
      v-if="showPreviewModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="showPreviewModal = false"
    >
      <div
        class="w-full max-w-2xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#2b2d75] to-[#4c2e6c] shadow-2xl flex flex-col max-h-[80vh]"
      >
        <div class="flex items-center justify-between p-8 pb-4 border-b border-white/10">
          <h2 class="text-2xl font-bold text-white">Ajouter un Aperçu Web</h2>
          <button
            class="text-white/60 hover:text-white transition-colors"
            @click="showPreviewModal = false"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <form @submit.prevent="handlePreviewSubmit" class="flex-1 overflow-y-auto p-8 space-y-4">
          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Nom du site web *</label>
            <input
              v-model="previewForm.name"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              required
              placeholder="Ex: Site Web Principal"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">URL *</label>
            <input
              v-model="previewForm.url"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="url"
              required
              placeholder="https://example.com"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Miniature *</label>
            <input
              type="file"
              accept="image/*"
              required
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-semibold file:bg-[#c62d6a] file:text-white hover:file:bg-[#d63d7a] transition-all"
              @change="
                (e) => {
                  const file = (e.target as HTMLInputElement).files?.[0]
                  if (file) previewForm.thumbnailFile = file
                }
              "
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Description</label>
            <textarea
              v-model="previewForm.description"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all resize-none"
              rows="3"
              placeholder="Description du projet..."
            ></textarea>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Technologies (séparées par des virgules)</label
            >
            <input
              v-model="previewForm.technologies"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Vue.js, Node.js, Firebase"
            />
          </div>

          <div class="flex items-center gap-4 pt-4 border-t border-white/10">
            <button
              type="button"
              class="flex-1 px-6 py-3 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all font-semibold"
              @click="showPreviewModal = false"
            >
              Annuler
            </button>
            <button
              type="submit"
              class="flex-1 px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
              :disabled="isUploading"
            >
              {{ isUploading ? 'Enregistrement...' : 'Enregistrer' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Team Member Modal -->
    <div
      v-if="showTeamModal"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="showTeamModal = false"
    >
      <div
        class="w-full max-w-2xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#2b2d75] to-[#4c2e6c] shadow-2xl flex flex-col max-h-[80vh]"
      >
        <div class="flex items-center justify-between p-8 pb-4 border-b border-white/10">
          <h2 class="text-2xl font-bold text-white">Ajouter un Membre d'Équipe</h2>
          <button
            class="text-white/60 hover:text-white transition-colors"
            @click="showTeamModal = false"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <form @submit.prevent="handleTeamSubmit" class="flex-1 overflow-y-auto p-8 space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-white/90 mb-2">Nom *</label>
              <input
                v-model="teamForm.name"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="text"
                required
                placeholder="Jean Dupont"
              />
            </div>

            <div>
              <label class="block text-sm font-semibold text-white/90 mb-2">Rôle *</label>
              <input
                v-model="teamForm.role"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="text"
                required
                placeholder="Directeur Créatif"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Description *</label>
            <textarea
              v-model="teamForm.description"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all resize-none"
              rows="3"
              required
              placeholder="Description du membre..."
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-white/90 mb-2">Email</label>
              <input
                v-model="teamForm.email"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="email"
                placeholder="jean@nelf.com"
              />
            </div>

            <div>
              <label class="block text-sm font-semibold text-white/90 mb-2">Téléphone</label>
              <input
                v-model="teamForm.phone"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="tel"
                placeholder="+33 6 12 34 56 78"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2">Photo de profil</label>
            <input
              type="file"
              accept="image/*"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-semibold file:bg-[#c62d6a] file:text-white hover:file:bg-[#d63d7a] transition-all"
              @change="
                (e) => {
                  const file = (e.target as HTMLInputElement).files?.[0]
                  if (file) teamForm.avatarFile = file
                }
              "
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-white/90 mb-2"
              >Compétences (séparées par des virgules)</label
            >
            <input
              v-model="teamForm.skills"
              class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
              type="text"
              placeholder="Vue.js, Design, Management"
            />
          </div>

          <div class="space-y-2">
            <label class="block text-sm font-semibold text-white/90">Liens sociaux</label>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <input
                  v-model="teamForm.linkedin"
                  class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                  type="url"
                  placeholder="LinkedIn"
                />
              </div>
              <div>
                <input
                  v-model="teamForm.github"
                  class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                  type="url"
                  placeholder="GitHub"
                />
              </div>
              <div>
                <input
                  v-model="teamForm.portfolio"
                  class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                  type="url"
                  placeholder="Portfolio"
                />
              </div>
            </div>
          </div>

          <div class="flex items-center gap-4 pt-4 border-t border-white/10">
            <button
              type="button"
              class="flex-1 px-6 py-3 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all font-semibold"
              @click="showTeamModal = false"
            >
              Annuler
            </button>
            <button
              type="submit"
              class="flex-1 px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg disabled:opacity-60"
              :disabled="isUploading"
            >
              {{ isUploading ? 'Enregistrement...' : 'Enregistrer' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div
      v-if="showDeleteConfirm"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
      @click.self="showDeleteConfirm = false"
    >
      <div
        class="w-full max-w-md rounded-3xl border border-white/20 bg-gradient-to-br from-[#2b2d75] to-[#4c2e6c] p-8 shadow-2xl"
      >
        <h2 class="text-2xl font-bold text-white mb-4">Confirmer la suppression</h2>
        <p class="text-white/80 mb-6">
          Êtes-vous sûr de vouloir supprimer cet élément ? Cette action est irréversible.
        </p>
        <div class="flex items-center gap-4">
          <button
            class="flex-1 px-6 py-3 rounded-xl bg-white/10 border border-white/20 text-white hover:bg-white/15 transition-all font-semibold"
            @click="showDeleteConfirm = false"
          >
            Annuler
          </button>
          <button
            class="flex-1 px-6 py-3 rounded-xl bg-red-500/20 border border-red-500/30 text-red-200 hover:bg-red-500/30 transition-all font-semibold disabled:opacity-60"
            :disabled="isLoading"
            @click="confirmDelete"
          >
            {{ isLoading ? 'Suppression...' : 'Supprimer' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Additional custom styles if needed */
</style>
