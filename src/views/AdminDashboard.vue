<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { onAuthStateChanged, signOut, type User } from 'firebase/auth'
import { getFirebaseAuth } from '@/services/firebase'

const router = useRouter()
const auth = getFirebaseAuth()

const user = ref<User | null>(null)
const isLoading = ref(false)
const activeTab = ref<'flyers' | 'videos' | 'previews' | 'team'>('flyers')

// Mock data
const flyers = ref([
  { id: 1, name: 'Affiche 1', url: '/assets/affiche.jpeg', uploadedAt: '2024-01-15' },
  { id: 2, name: 'Affiche 2', url: '/assets/affiche2.jpeg', uploadedAt: '2024-01-20' },
  { id: 3, name: 'Affiche 3', url: '/assets/affiche3.jpeg', uploadedAt: '2024-01-25' },
])

const videos = ref([
  { id: 1, name: 'Présentation NELF', url: 'https://example.com/video1.mp4', uploadedAt: '2024-01-10' },
  { id: 2, name: 'Témoignage Client', url: 'https://example.com/video2.mp4', uploadedAt: '2024-01-18' },
])

const previews = ref([
  { id: 1, name: 'Site Web Principal', url: 'https://nelf.com', thumbnail: '/assets/affiche.jpeg', uploadedAt: '2024-01-12' },
  { id: 2, name: 'Portfolio', url: 'https://portfolio.nelf.com', thumbnail: '/assets/affiche2.jpeg', uploadedAt: '2024-01-22' },
])

const teamMembers = ref([
  { id: 1, name: 'Jean Dupont', role: 'Directeur Créatif', description: 'Expert en design et stratégie créative', avatar: 'JD' },
  { id: 2, name: 'Marie Martin', role: 'Développeuse Full-Stack', description: 'Spécialisée en Vue.js et Node.js', avatar: 'MM' },
  { id: 3, name: 'Pierre Bernard', role: 'Chef de Projet', description: 'Gestion de projets et coordination d\'équipe', avatar: 'PB' },
])

let unsubscribe: (() => void) | null = null

onMounted(() => {
  unsubscribe = onAuthStateChanged(auth, (u) => {
    user.value = u
    if (!u) {
      router.push({ name: 'admin-login' })
    }
  })
})

onUnmounted(() => {
  unsubscribe?.()
  unsubscribe = null
})

async function handleLogout() {
  isLoading.value = true
  try {
    await signOut(auth)
    router.push({ name: 'admin-login' })
  } catch (error) {
    console.error('Logout error:', error)
  } finally {
    isLoading.value = false
  }
}

function handleFileUpload(type: 'flyers' | 'videos' | 'previews') {
  // Mock upload - in real implementation, this would upload to Firebase Storage
  alert(`Upload ${type} - Cette fonctionnalité sera implémentée avec Firebase Storage`)
}

function handleAddTeamMember() {
  const name = prompt('Nom du membre:')
  const role = prompt('Rôle:')
  const description = prompt('Description:')
  
  if (name && role && description) {
    const initials = name.split(' ').map(n => n[0]).join('').toUpperCase()
    teamMembers.value.push({
      id: teamMembers.value.length + 1,
      name,
      role,
      description,
      avatar: initials,
    })
  }
}

function handleDeleteItem(type: 'flyers' | 'videos' | 'previews' | 'team', id: number) {
  if (confirm('Êtes-vous sûr de vouloir supprimer cet élément?')) {
    if (type === 'flyers') {
      flyers.value = flyers.value.filter(f => f.id !== id)
    } else if (type === 'videos') {
      videos.value = videos.value.filter(v => v.id !== id)
    } else if (type === 'previews') {
      previews.value = previews.value.filter(p => p.id !== id)
    } else if (type === 'team') {
      teamMembers.value = teamMembers.value.filter(t => t.id !== id)
    }
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

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-6 py-8">
      <!-- Tabs -->
      <div class="mb-8 flex items-center gap-2 p-1 rounded-xl bg-white/5 border border-white/10 w-fit">
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
          <h2 class="text-xl font-bold text-white">Gestion des Affiches</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg"
            @click="handleFileUpload('flyers')"
          >
            + Ajouter une affiche
          </button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="flyer in flyers"
            :key="flyer.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-4 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="aspect-[3/4] rounded-xl bg-white/5 mb-4 overflow-hidden">
              <img :src="flyer.url" :alt="flyer.name" class="w-full h-full object-cover" />
            </div>
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-white">{{ flyer.name }}</h3>
                <p class="text-xs text-white/60 mt-1">{{ flyer.uploadedAt }}</p>
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
          <h2 class="text-xl font-bold text-white">Gestion des Vidéos</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg"
            @click="handleFileUpload('videos')"
          >
            + Ajouter une vidéo
          </button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div
            v-for="video in videos"
            :key="video.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="aspect-video rounded-xl bg-white/5 mb-4 flex items-center justify-center">
              <svg class="w-16 h-16 text-white/40" fill="currentColor" viewBox="0 0 24 24">
                <path d="M8 5v14l11-7z" />
              </svg>
            </div>
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-white">{{ video.name }}</h3>
                <p class="text-xs text-white/60 mt-1">{{ video.uploadedAt }}</p>
                <a :href="video.url" target="_blank" class="text-sm text-[#c62d6a] hover:underline mt-2 block">
                  {{ video.url }}
                </a>
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
          <h2 class="text-xl font-bold text-white">Gestion des Aperçus Web</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg"
            @click="handleFileUpload('previews')"
          >
            + Ajouter un aperçu
          </button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div
            v-for="preview in previews"
            :key="preview.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="aspect-video rounded-xl bg-white/5 mb-4 overflow-hidden">
              <img :src="preview.thumbnail" :alt="preview.name" class="w-full h-full object-cover" />
            </div>
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-white">{{ preview.name }}</h3>
                <p class="text-xs text-white/60 mt-1">{{ preview.uploadedAt }}</p>
                <a :href="preview.url" target="_blank" class="text-sm text-[#c62d6a] hover:underline mt-2 block">
                  {{ preview.url }}
                </a>
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
          <h2 class="text-xl font-bold text-white">Gestion de l'Équipe</h2>
          <button
            class="px-6 py-3 rounded-xl bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all font-semibold shadow-lg"
            @click="handleAddTeamMember"
          >
            + Ajouter un membre
          </button>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="member in teamMembers"
            :key="member.id"
            class="rounded-2xl bg-white/10 border border-white/20 p-6 backdrop-blur-md hover:bg-white/15 transition-all"
          >
            <div class="flex items-center gap-4 mb-4">
              <div
                class="w-16 h-16 rounded-full bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] flex items-center justify-center text-white font-bold text-xl"
              >
                {{ member.avatar }}
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-white">{{ member.name }}</h3>
                <p class="text-sm text-[#c62d6a] mt-1">{{ member.role }}</p>
              </div>
            </div>
            <p class="text-sm text-white/80 mb-4">{{ member.description }}</p>
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
  </div>
</template>

<style scoped>
/* Additional custom styles if needed */
</style>

