<script setup lang="ts">
// Footer - sera dynamique plus tard
</script>

<template>
  <footer
    class="bg-[#2b2d75] py-16 border-t border-white/5 text-white relative overflow-hidden"
  >
    <div
      class="absolute top-0 right-0 w-75 h-75 bg-primary/10 blur-[100px] rounded-full pointer-events-none"
    ></div>
    <div
      class="layout-content-container max-w-300 mx-auto flex flex-col md:flex-row justify-between items-center md:items-start gap-10"
    >
      <div class="flex flex-col gap-4 text-center md:text-left">
        <div class="flex items-center justify-center md:justify-start gap-3">
          <div
            class="size-8 bg-linear-to-br from-primary to-secondary rounded-lg flex items-center justify-center"
          >
            <span class="material-symbols-outlined text-lg">front_hand</span>
          </div>
          <span class="font-bold text-2xl tracking-tight">NELF</span>
        </div>
        <p class="text-gray-400 text-sm max-w-xs">
          Votre partenaire digital de confiance. Nous construisons le futur de votre entreprise,
          main dans la main.
        </p>
      </div>
      <div class="flex flex-col md:flex-row gap-8 md:gap-16 text-center md:text-left">
        <div class="flex flex-col gap-3">
          <h4 class="font-bold text-[#c62d6a] mb-1">Navigation</h4>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#services">
            Services
          </a>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#valeurs">
            Valeurs
          </a>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#creations">
            Projets
          </a>
        </div>
        <div class="flex flex-col gap-3">
          <h4 class="font-bold text-[#c62d6a] mb-1">Légal</h4>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#">
            Mentions Légales
          </a>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#">
            Confidentialité
          </a>
          <a class="text-gray-400 hover:text-white transition-colors text-sm" href="#">CGV</a>
        </div>
      </div>
      <div class="flex gap-4">
        <a
          class="size-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center text-white hover:bg-primary hover:text-white hover:scale-110 hover:border-primary transition-all duration-300"
          href="#"
        >
          <span class="material-symbols-outlined text-xl">mail</span>
        </a>
        <a
          class="size-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center text-white hover:bg-primary hover:text-white hover:scale-110 hover:border-primary transition-all duration-300"
          href="#"
        >
          <span class="material-symbols-outlined text-xl">call</span>
        </a>
        <a
          class="size-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center text-white hover:bg-primary hover:text-white hover:scale-110 hover:border-primary transition-all duration-300"
          href="#"
        >
          <span class="material-symbols-outlined text-xl">share</span>
        </a>
      </div>
    </div>
    <div class="text-center mt-12 pt-8 border-t border-white/10 text-gray-500 text-xs">
      © 2025 NELF. Tous droits réservés. "Votre troisième main" est une marque déposée.
    </div>
  </footer>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
