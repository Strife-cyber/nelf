<script setup lang="ts">
import { onMounted, ref, computed, onUnmounted } from 'vue'
import { flyersService, videosService, previewsService } from '@/services/firestore'
import type { Flyer, Video, WebsitePreview } from '@/types/models'

type CreationItem = {
  id: string
  type: 'flyer' | 'video' | 'website'
  title: string
  eventTitle?: string
  description?: string
  shortInfo?: string
  image: string
  url?: string
  aspect: 'portrait' | 'video' | 'square'
  isVideo?: boolean
  data: Flyer | Video | WebsitePreview
}

const flyers = ref<Flyer[]>([])
const videos = ref<Video[]>([])
const websites = ref<WebsitePreview[]>([])
const isLoading = ref(true)
const selectedItem = ref<CreationItem | null>(null)
const showDetailModal = ref(false)
const visibleVideos = ref<Set<string>>(new Set())
const videoRefs = ref<Map<string, HTMLVideoElement>>(new Map())
let intersectionObserver: IntersectionObserver | null = null

const creations = computed<CreationItem[]>(() => {
  const items: CreationItem[] = []

  // Add flyers - use full-size images for better quality
  flyers.value.forEach((flyer) => {
    items.push({
      id: flyer.id,
      type: 'flyer',
      title: flyer.name,
      eventTitle: flyer.eventTitle,
      description: flyer.description,
      shortInfo: flyer.shortInfo,
      image: flyer.url || flyer.thumbnailUrl || '', // Use full-size URL first
      aspect: 'portrait',
      data: flyer,
    })
  })

  // Add videos - use full-size images for better quality
  videos.value.forEach((video) => {
    items.push({
      id: video.id,
      type: 'video',
      title: video.name,
      eventTitle: video.eventTitle,
      description: video.description,
      shortInfo: video.shortInfo,
      image: video.url || video.thumbnailUrl || '', // Use full-size URL first
      url: video.url,
      aspect: 'video',
      isVideo: true,
      data: video,
    })
  })

  // Add websites - use full-size images for better quality
  websites.value.forEach((website) => {
    items.push({
      id: website.id,
      type: 'website',
      title: website.name,
      description: website.description,
      image: website.thumbnailUrl, // Website thumbnails are full screenshots
      url: website.url,
      aspect: 'video',
      data: website,
    })
  })

  return items
})

onMounted(async () => {
  try {
    isLoading.value = true
    const [flyersData, videosData, websitesData] = await Promise.all([
      flyersService.readAll(true),
      videosService.readAll(true),
      previewsService.readAll(true),
    ])
    flyers.value = flyersData
    videos.value = videosData
    websites.value = websitesData

    // Setup intersection observer for video prefetching
    setupVideoObserver()
  } catch (error) {
    console.error('Error loading creations:', error)
  } finally {
    isLoading.value = false
  }
})

onUnmounted(() => {
  if (intersectionObserver) {
    intersectionObserver.disconnect()
    intersectionObserver = null
  }
})

function setupVideoObserver() {
  intersectionObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        const videoId = entry.target.getAttribute('data-video-id')
        if (!videoId) return

        if (entry.isIntersecting && entry.intersectionRatio > 0.3) {
          // Video is visible - prefetch and play
          visibleVideos.value.add(videoId)
          const video = videoRefs.value.get(videoId)
          if (video) {
            video.preload = 'auto'
            // Ensure video is muted for autoplay
            video.muted = true
            video.load()
            video.play().catch(() => {
              // Autoplay failed, user interaction required - keep poster visible
              visibleVideos.value.delete(videoId)
            })
          }
        } else {
          // Video is out of viewport - pause and reduce preload
          visibleVideos.value.delete(videoId)
          const video = videoRefs.value.get(videoId)
          if (video) {
            video.pause()
            video.currentTime = 0 // Reset to start
            video.preload = 'metadata'
          }
        }
      })
    },
    {
      rootMargin: '100px', // Start loading videos 100px before they enter viewport
      threshold: [0, 0.5, 1],
    },
  )
}

function registerVideoRef(videoId: string, element: HTMLVideoElement | null) {
  if (element) {
    videoRefs.value.set(videoId, element)
    if (intersectionObserver) {
      intersectionObserver.observe(element)
    }
  } else {
    videoRefs.value.delete(videoId)
  }
}

function handleItemClick(item: CreationItem) {
  if (item.type === 'website' && item.url) {
    // Navigate to website
    window.open(item.url, '_blank')
  } else {
    // Show detail modal
    selectedItem.value = item
    showDetailModal.value = true
  }
}

function closeModal() {
  showDetailModal.value = false
  selectedItem.value = null
}
</script>

<template>
  <div class="py-24 bg-[#121226] overflow-hidden" id="creations">
    <div class="layout-content-container max-w-350 mx-auto px-4 md:px-10 flex flex-col">
      <div class="flex flex-col md:flex-row justify-between items-end mb-12 gap-6">
        <div>
          <h2 class="text-white text-4xl font-bold tracking-tight mb-2">
            Nos Créations
          </h2>
          <p class="text-gray-400 text-lg">
            Découvrez nos réalisations récentes en mode "Showroom".
          </p>
        </div>
        <div class="flex gap-2">
          <span
            class="text-sm text-gray-400 font-medium mr-2 self-center hidden md:block"
          >
            Glisser pour découvrir
          </span>
          <span class="material-symbols-outlined text-primary animate-pulse">arrow_right_alt</span>
        </div>
      </div>

      <div v-if="isLoading" class="flex items-center justify-center py-20">
        <div class="text-gray-400">Chargement des créations...</div>
      </div>
      <div
        v-else-if="creations.length > 0"
        class="flex overflow-x-auto gap-6 pb-12 snap-x snap-mandatory no-scrollbar px-4 -mx-4 md:px-0 md:mx-0"
      >
        <div
          v-for="creation in creations"
          :key="creation.id"
          class="flex-none snap-center group relative rounded-2xl overflow-hidden shadow-2xl cursor-pointer w-[85vw] h-[500px] md:w-[450px] md:h-[600px] bg-gray-900"
          @click="handleItemClick(creation)"
        >
          <!-- Video element for video creations -->
          <video
            v-if="creation.isVideo && creation.url"
            :ref="(el) => registerVideoRef(creation.id, el as HTMLVideoElement)"
            :data-video-id="creation.id"
            :src="creation.url"
            :poster="creation.image"
            :preload="visibleVideos.has(creation.id) ? 'auto' : 'metadata'"
            loop
            muted
            playsinline
            class="absolute inset-0 w-full h-full object-cover transition-transform duration-700 group-hover:scale-105"
            :class="visibleVideos.has(creation.id) ? 'opacity-100' : 'opacity-0'"
          ></video>
          <!-- Background image for non-video or fallback -->
          <div
            v-if="!creation.isVideo || !creation.url"
            class="absolute inset-0 bg-cover transition-transform duration-700 group-hover:scale-105"
            :class="[
              creation.type === 'website' ? 'bg-top' : 'bg-center',
              creation.type === 'flyer' ? 'object-contain' : '',
            ]"
            :style="{ backgroundImage: `url(${creation.image})` }"
          ></div>
          <!-- Poster/thumbnail overlay for videos when not playing -->
          <div
            v-if="creation.isVideo && !visibleVideos.has(creation.id)"
            class="absolute inset-0 bg-cover bg-center transition-transform duration-700"
            :style="{ backgroundImage: `url(${creation.image})` }"
          ></div>
          <div
            v-if="!creation.isVideo"
            class="absolute inset-0 bg-linear-to-t from-black/80 via-black/20 to-transparent opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity duration-300"
          ></div>
          <!-- Play button overlay for videos -->
          <div
            v-if="creation.isVideo"
            class="absolute inset-0 flex items-center justify-center pointer-events-none"
            :class="visibleVideos.has(creation.id) ? 'opacity-0' : 'opacity-100'"
          >
            <div
              class="size-20 rounded-full bg-white/10 backdrop-blur-md border border-white/20 flex items-center justify-center text-white group-hover:scale-110 group-hover:bg-primary group-hover:border-primary transition-all duration-300 shadow-[0_0_30px_rgba(255,255,255,0.2)]"
            >
              <span class="material-symbols-outlined text-4xl ml-1">play_arrow</span>
            </div>
          </div>
          <div
            :class="[
              'absolute bottom-0 left-0 w-full p-8',
              !creation.isVideo
                ? 'translate-y-0 md:translate-y-4 md:group-hover:translate-y-0 transition-transform duration-300'
                : '',
            ]"
          >
            <div class="flex items-center gap-3 mb-2">
              <span
                :class="[
                  'px-3 py-1 text-white text-xs font-bold rounded-full uppercase tracking-wide',
                  creation.type === 'website' ? 'bg-[#c62d6a]' : '',
                  creation.type === 'flyer' ? 'bg-black' : '',
                  creation.type === 'video' ? 'bg-[#4c2e6c]' : '',
                ]"
              >
                {{
                  creation.type === 'website'
                    ? 'Web'
                    : creation.type === 'flyer'
                      ? 'Print'
                      : 'Motion'
                }}
              </span>
            </div>
            <h3 class="text-white text-2xl font-bold mb-2">{{ creation.title }}</h3>
            <p v-if="creation.eventTitle" class="text-[#c62d6a] text-sm font-medium mb-1">
              {{ creation.eventTitle }}
            </p>
            <p v-if="creation.shortInfo" class="text-gray-300 text-xs mb-1">
              {{ creation.shortInfo }}
            </p>
            <p v-if="creation.description" class="text-gray-300 text-sm line-clamp-2">
              {{ creation.description }}
            </p>
          </div>
        </div>
      </div>
      <div v-else class="text-center py-20 text-gray-400">
        Aucune création disponible pour le moment
      </div>

      <!-- Detail Modal -->
      <div
        v-if="showDetailModal && selectedItem"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
        @click.self="closeModal"
      >
        <div
          class="w-full max-w-4xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#121226] to-[#2b2d75] shadow-2xl flex flex-col max-h-[90vh] overflow-hidden"
        >
          <div class="flex items-center justify-between p-6 border-b border-white/10">
            <h2 class="text-2xl font-bold text-white">{{ selectedItem.title }}</h2>
            <button class="text-white/60 hover:text-white transition-colors" @click="closeModal">
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

          <div class="flex-1 overflow-y-auto p-6">
            <!-- Video player for videos -->
            <div v-if="selectedItem.type === 'video' && selectedItem.url" class="mb-6">
              <video
                :src="selectedItem.url"
                :poster="selectedItem.image"
                controls
                preload="metadata"
                class="w-full rounded-xl aspect-video bg-black"
              >
                Votre navigateur ne supporte pas la lecture de vidéos.
              </video>
            </div>

            <!-- Image for non-video items -->
            <div v-else class="mb-6">
              <img
                :src="selectedItem.image"
                :alt="selectedItem.title"
                class="w-full rounded-xl max-h-96"
                :class="[
                  selectedItem.aspect === 'portrait' ? 'aspect-3/4 object-cover' : '',
                  selectedItem.type === 'website'
                    ? 'aspect-video object-cover object-top'
                    : selectedItem.aspect === 'video'
                      ? 'aspect-video object-cover'
                      : '',
                ]"
              />
            </div>

            <div class="space-y-4 text-white">
              <div v-if="selectedItem.eventTitle">
                <h3 class="text-[#c62d6a] text-xl font-bold mb-2">Événement</h3>
                <p class="text-white/90">{{ selectedItem.eventTitle }}</p>
              </div>

              <div v-if="selectedItem.shortInfo">
                <h3 class="text-white/80 font-semibold mb-2">Informations</h3>
                <p class="text-white/70">{{ selectedItem.shortInfo }}</p>
              </div>

              <div v-if="selectedItem.description">
                <h3 class="text-white/80 font-semibold mb-2">Description</h3>
                <p class="text-white/70">{{ selectedItem.description }}</p>
              </div>

              <div v-if="selectedItem.type === 'website' && selectedItem.url" class="pt-4">
                <a
                  :href="selectedItem.url"
                  target="_blank"
                  class="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-primary text-white hover:bg-[#d63d7a] transition-all font-semibold"
                >
                  <span>Visiter le site</span>
                  <span class="material-symbols-outlined">open_in_new</span>
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
