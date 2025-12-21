<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { teamService } from '@/services/firestore'
import type { TeamMember } from '@/types/models'

const teamMembers = ref<TeamMember[]>([])
const isLoading = ref(true)
const selectedMember = ref<TeamMember | null>(null)
const showDetailModal = ref(false)

const gradients = [
  'from-[#4c2e6c] via-[#4c2e6c]/50 to-transparent',
  'from-[#c62d6a] via-[#c62d6a]/50 to-transparent',
  'from-[#2b2d75] via-[#2b2d75]/50 to-transparent',
]

function getGradient(index: number): string {
  return gradients[index % gradients.length]
}

onMounted(async () => {
  try {
    isLoading.value = true
    teamMembers.value = await teamService.readAll(true)
  } catch (error) {
    console.error('Error loading team members:', error)
  } finally {
    isLoading.value = false
  }
})

function handleMemberClick(member: TeamMember) {
  selectedMember.value = member
  showDetailModal.value = true
}

function closeModal() {
  showDetailModal.value = false
  selectedMember.value = null
}
</script>

<template>
  <div class="py-24 bg-white dark:bg-[#121226]" id="equipe">
    <div class="layout-content-container max-w-300 mx-auto px-4 md:px-10">
      <div class="text-center mb-16">
        <span class="text-[#c62d6a] font-bold tracking-widest uppercase text-sm mb-2 block">
          Le Cœur de NELF
        </span>
        <h2 class="text-[#111714] dark:text-white text-4xl font-bold leading-tight">
          Notre Équipe
        </h2>
      </div>

      <div v-if="isLoading" class="flex items-center justify-center py-20">
        <div class="text-[#637588] dark:text-gray-400">Chargement de l'équipe...</div>
      </div>
      <div v-else-if="teamMembers.length > 0" class="flex flex-wrap justify-center gap-10">
        <div
          v-for="(member, index) in teamMembers"
          :key="member.id"
          class="group relative w-72 h-96 rounded-2xl overflow-hidden shadow-xl cursor-pointer"
          @click="handleMemberClick(member)"
        >
          <img
            v-if="member.avatar"
            :alt="`Portrait of ${member.name}`"
            :src="member.avatar"
            class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
          />
          <div
            v-else
            class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] text-white text-6xl font-bold"
          >
            {{ member.avatarInitials || member.name.charAt(0).toUpperCase() }}
          </div>
          <div
            :class="[
              'absolute inset-0 bg-linear-to-t opacity-60 group-hover:opacity-90 transition-opacity duration-300',
              getGradient(index),
            ]"
          ></div>
          <div
            class="absolute bottom-0 left-0 w-full p-6 text-white transform translate-y-12 group-hover:translate-y-0 transition-transform duration-300"
          >
            <h3 class="text-2xl font-bold">{{ member.name }}</h3>
            <p
              :class="[
                'font-medium mb-4',
                member.role.includes('CEO') || member.role.includes('Dév') || member.role.includes('Lead')
                  ? 'text-[#c62d6a]'
                  : 'text-white/80',
              ]"
            >
              {{ member.role }}
            </p>
            <p
              class="text-sm text-gray-200 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-100 mb-4 line-clamp-2"
            >
              {{ member.description }}
            </p>
            <div
              v-if="member.skills && member.skills.length > 0"
              class="flex flex-wrap gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-200 mb-4"
            >
              <span
                v-for="skill in member.skills.slice(0, 3)"
                :key="skill"
                class="px-2 py-0.5 rounded text-xs bg-white/20 text-white/90"
              >
                {{ skill }}
              </span>
            </div>
            <div
              class="flex gap-4 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-200"
            >
              <a
                v-if="member.email"
                :href="`mailto:${member.email}`"
                @click.stop
                class="material-symbols-outlined text-xl transition-colors hover:text-[#c62d6a]"
              >
                mail
              </a>
              <a
                v-if="member.socialLinks?.linkedin"
                :href="member.socialLinks.linkedin"
                target="_blank"
                @click.stop
                class="material-symbols-outlined text-xl transition-colors hover:text-[#c62d6a]"
              >
                link
              </a>
              <a
                v-if="member.socialLinks?.portfolio"
                :href="member.socialLinks.portfolio"
                target="_blank"
                @click.stop
                class="material-symbols-outlined text-xl transition-colors hover:text-[#c62d6a]"
              >
                language
              </a>
            </div>
          </div>
        </div>
      </div>
      <div v-else class="text-center py-20 text-[#637588] dark:text-gray-400">
        Aucun membre d'équipe disponible pour le moment
      </div>

      <!-- Detail Modal -->
      <div
        v-if="showDetailModal && selectedMember"
        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
        @click.self="closeModal"
      >
        <div
          class="w-full max-w-2xl rounded-3xl border border-white/20 bg-gradient-to-br from-[#121226] to-[#2b2d75] shadow-2xl overflow-hidden"
        >
          <div class="flex items-center justify-between p-6 border-b border-white/10">
            <h2 class="text-2xl font-bold text-white">{{ selectedMember.name }}</h2>
            <button
              class="text-white/60 hover:text-white transition-colors"
              @click="closeModal"
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

          <div class="p-6 space-y-6">
            <div class="flex items-center gap-6">
              <div
                v-if="selectedMember.avatar"
                class="w-32 h-32 rounded-full overflow-hidden bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c]"
              >
                <img
                  :src="selectedMember.avatar"
                  :alt="selectedMember.name"
                  class="w-full h-full object-cover"
                />
              </div>
              <div
                v-else
                class="w-32 h-32 rounded-full bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] flex items-center justify-center text-white text-4xl font-bold"
              >
                {{ selectedMember.avatarInitials || selectedMember.name.charAt(0).toUpperCase() }}
              </div>
              <div class="flex-1">
                <h3 class="text-3xl font-bold text-white mb-2">{{ selectedMember.name }}</h3>
                <p class="text-[#c62d6a] text-xl font-semibold">{{ selectedMember.role }}</p>
              </div>
            </div>

            <div class="text-white/90">
              <p class="text-lg leading-relaxed">{{ selectedMember.description }}</p>
            </div>

            <div v-if="selectedMember.skills && selectedMember.skills.length > 0">
              <h4 class="text-white/80 font-semibold mb-3">Compétences</h4>
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="skill in selectedMember.skills"
                  :key="skill"
                  class="px-3 py-1 rounded-full text-sm bg-white/10 text-white/90"
                >
                  {{ skill }}
                </span>
              </div>
            </div>

            <div v-if="selectedMember.email || selectedMember.phone || selectedMember.socialLinks" class="flex flex-wrap gap-4 pt-4 border-t border-white/10">
              <a
                v-if="selectedMember.email"
                :href="`mailto:${selectedMember.email}`"
                class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-all"
              >
                <span class="material-symbols-outlined">mail</span>
                <span>Email</span>
              </a>
              <a
                v-if="selectedMember.socialLinks?.linkedin"
                :href="selectedMember.socialLinks.linkedin"
                target="_blank"
                class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-all"
              >
                <span class="material-symbols-outlined">link</span>
                <span>LinkedIn</span>
              </a>
              <a
                v-if="selectedMember.socialLinks?.portfolio"
                :href="selectedMember.socialLinks.portfolio"
                target="_blank"
                class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-all"
              >
                <span class="material-symbols-outlined">language</span>
                <span>Portfolio</span>
              </a>
              <a
                v-if="selectedMember.socialLinks?.github"
                :href="selectedMember.socialLinks.github"
                target="_blank"
                class="flex items-center gap-2 px-4 py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-all"
              >
                <span class="material-symbols-outlined">code</span>
                <span>GitHub</span>
              </a>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
