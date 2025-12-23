<script setup lang="ts">
import { RouterView } from 'vue-router'
import { onMounted } from 'vue'

// Hardcode dark mode - always active
onMounted(() => {
  document.documentElement.classList.add('dark')
  // Prevent any light mode switching
  document.documentElement.style.colorScheme = 'dark'
})
</script>

<template>
  <div
    class="bg-background-dark text-white font-display overflow-x-hidden selection:bg-primary selection:text-white"
  >
    <RouterView />
  </div>
</template>

<style scoped></style>
