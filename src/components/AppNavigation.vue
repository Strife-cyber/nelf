<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const isScrolled = ref(false)

const handleScroll = () => {
  isScrolled.value = window.scrollY > 20
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})

const scrollToSection = (id: string) => {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
  }
}
</script>

<template>
  <nav
    class="fixed top-0 left-0 right-0 z-50 flex justify-center pt-4 px-4 transition-all"
    :class="{ 'pt-2': isScrolled }"
  >
    <div
      class="flex items-center justify-between w-full max-w-[1200px] bg-white/90 backdrop-blur-md rounded-full px-6 py-3 shadow-lg border border-white/20 transition-all"
      :class="{ 'py-2': isScrolled }"
    >
      <div class="flex items-center gap-2">
        <img src="/src/assets/logo.png" alt="NELF Logo" class="h-8 w-8 rounded-full object-cover" />
        <span class="text-[#2b2d75] text-xl font-bold tracking-tight">NELF</span>
      </div>
      <div class="hidden md:flex items-center gap-8">
        <button
          class="text-[#181811] text-sm font-medium hover:text-[#c62d6a] transition-colors cursor-pointer"
          @click="scrollToSection('services')"
        >
          CE QU'ON FAIT
        </button>
        <button
          class="text-[#181811] text-sm font-medium hover:text-[#c62d6a] transition-colors cursor-pointer"
          @click="scrollToSection('apport')"
        >
          APPORT
        </button>
        <button
          class="text-[#181811] text-sm font-medium hover:text-[#c62d6a] transition-colors cursor-pointer"
          @click="scrollToSection('vision')"
        >
          VISION
        </button>
        <button
          class="text-[#181811] text-sm font-medium hover:text-[#c62d6a] transition-colors cursor-pointer"
          @click="scrollToSection('realisation')"
        >
          REALISATION
        </button>
      </div>
      <button
        class="flex items-center justify-center rounded-full h-10 px-5 text-white text-sm font-bold shadow-md hover:scale-105 transition-transform cursor-pointer"
        style="background-color: #c62d6a"
        @click="scrollToSection('contact')"
      >
        Nous Contacter
      </button>
    </div>
  </nav>
</template>
