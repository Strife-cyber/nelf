<script setup lang="ts">
// import { ref } from 'vue'

// const isMenuOpen = ref(false)

/*const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}*/
</script>

<template>
  <header
    class="flex items-center justify-between whitespace-nowrap border-b border-solid bg-[#1a1b41] dark:border-white/10 backdrop-blur-md px-10 py-4 fixed top-0 w-full z-50 transition-all duration-300"
  >
    <div class="flex items-center gap-3 text-white group cursor-pointer">
      <div
        class="size-10 flex items-center justify-center text-white from-secondary to-primary rounded-xl shadow-lg group-hover:rotate-12 transition-transform duration-300"
      >
        <span class="material-symbols-outlined text-2xl w-[50%] h-[50%]">
          <img src="../assets/logo.png" alt="Logo NELF" class="w-full h-full" />
        </span>
      </div>
      <div>
        <h2 class="text-xl font-bold leading-none tracking-tight">NELF</h2>
        <span
          class="text-[10px] text-[#c62d6a] font-medium text-primary uppercase tracking-widest block mt-1"
        >
          Votre troisième main
        </span>
      </div>
    </div>
    <div class="hidden lg:flex flex-1 justify-center gap-8 items-center">
      <div class="flex items-center gap-8">
        <a
          class="text-[#111714] dark:text-gray-200 text-sm font-medium leading-normal hover:text-primary transition-colors relative after:content-[''] after:absolute after:-bottom-1 after:left-0 after:w-0 after:h-0.5 after:bg-primary after:transition-all hover:after:w-full"
          href="#services"
        >
          Ce qu'on fais
        </a>
        <a
          class="text-[#111714] dark:text-gray-200 text-sm font-medium leading-normal hover:text-primary transition-colors relative after:content-[''] after:absolute after:-bottom-1 after:left-0 after:w-0 after:h-0.5 after:bg-primary after:transition-all hover:after:w-full"
          href="#valeurs"
        >
          Ce qu'on apporte
        </a>
        <a
          class="text-[#111714] dark:text-gray-200 text-sm font-medium leading-normal hover:text-primary transition-colors relative after:content-[''] after:absolute after:-bottom-1 after:left-0 after:w-0 after:h-0.5 after:bg-primary after:transition-all hover:after:w-full"
          href="#vision"
        >
          Notre vision
        </a>
        <a
          class="text-[#111714] dark:text-gray-200 text-sm font-medium leading-normal hover:text-primary transition-colors relative after:content-[''] after:absolute after:-bottom-1 after:left-0 after:w-0 after:h-0.5 after:bg-primary after:transition-all hover:after:w-full"
          href="#creations"
        >
          Nos Créations
        </a>
        <a
          class="text-[#111714] dark:text-gray-200 text-sm font-medium leading-normal hover:text-primary transition-colors relative after:content-[''] after:absolute after:-bottom-1 after:left-0 after:w-0 after:h-0.5 after:bg-primary after:transition-all hover:after:w-full"
          href="#equipe"
        >
          Notre Équipe
        </a>
      </div>
    </div>
    <button
      class="flex min-w-21 bg-linear-to-r from-[#c62d6a] to-[#2b2d75] cursor-pointer items-center justify-end overflow-hidden rounded-full h-10 px-6 text-white text-sm font-bold leading-normal tracking-[0.015em] shadow-lg shadow-primary/30 hover:shadow-primary/50 hover:-translate-y-0.5 transition-all"
    >
      <span class="truncate">Contactez-nous</span>
    </button>
    <!--<button class="lg:hidden text-[#111714] dark:text-white p-2" @click="toggleMenu">
      <span class="material-symbols-outlined">menu</span>
    </button>-->
  </header>
</template>

<style scoped>
/* Styles spécifiques au header si nécessaire */
</style>
