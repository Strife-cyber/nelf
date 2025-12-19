<script setup lang="ts">
import { RouterView } from 'vue-router'
import { onMounted } from 'vue'

// Activer le mode sombre par défaut (peut être rendu dynamique plus tard)
onMounted(() => {
  document.documentElement.classList.add('dark')
})
</script>

<template>
  <div
    class="bg-background-light dark:bg-background-dark text-[#111714] dark:text-white font-display overflow-x-hidden selection:bg-primary selection:text-white"
  >
    <RouterView />
  </div>
</template>

<style scoped></style>
