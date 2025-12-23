<script setup lang="ts">
// Section héro principale
</script>

<template>
  <div
    class="relative w-full flex items-center justify-center overflow-hidden min-h-screen hero-gradient"
  >
    <!-- Effets de fond animés -->
    <div class="absolute inset-0 z-0">
      <div
        class="absolute top-[-20%] left-[-10%] w-125 h-125 rounded-full bg-secondary/30 blur-[100px] animate-pulse-slow"
      ></div>
      <div
        class="absolute bottom-[-10%] right-[-5%] w-150 h-150 rounded-full bg-primary/20 blur-[120px] animate-float"
      ></div>
      <div
        class="absolute top-[40%] left-[60%] w-75 h-75 rounded-full bg-accent/40 blur-[80px] animate-float-delayed"
      ></div>
    </div>

    <!-- Image de fond -->
    <div
      class="absolute inset-0 z-0 opacity-40 mix-blend-overlay bg-fixed bg-cover bg-center"
      style="
        background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuCcea9F2M5YCZsFgfnHtbwa4qogJi2CRb4zH6b9sv2ScUqP9x01jc0zdk177RG6IyjWYQelMBvyImbMfJ3E-Y-ZP2pey_km8NpjybbRi4IJXcLMDvNCwyy3xusv8310-vmAS6sn8CQtMTFKtTUxsFhu4btg6tdzeGt-2r0aYjsEwZeWg523i_IeKDWLqGoktmE6-GX_jqmmn5D3RynG-ZJDCuKioW9pX5N6titqsYWvd_v4Wn-wOs_qnXv_2N4LNHZ5X88gFEIgaTVd');
      "
    ></div>

    <!-- Contenu principal -->
    <div
      class="layout-content-container relative z-10 flex flex-col items-center justify-center max-w-300 w-full text-center gap-8"
    >
      <div
        class="inline-flex items-center gap-2 px-4 py-2 rounded-full glass-panel border border-[#c62d6a] text-primary animate-float"
      >
        <span class="material-symbols-outlined text-[#c62d6a] text-sm">rocket_launch</span>
        <span class="text-sm text-[#c62d6a] font-bold uppercase tracking-wider"
          >AGENCE DIGITALE PREMUIM</span
        >
      </div>
      <h1
        class="text-5xl md:text-7xl lg:text-8xl font-black leading-tight tracking-tighter text-white drop-shadow-xl"
      >
        <span
          class="block text-transparent bg-clip-text bg-linear-to-r from-white via-gray-200 to-gray-400"
        >
          NELF
        </span>
        <span
          class="block text-3xl md:text-5xl lg:text-6xl mt-2 font-bold text-primary text-[#c62d6a]"
        >
          Votre troisième main
        </span>
      </h1>
      <p
        class="text-gray-300 text-lg md:text-xl font-normal leading-relaxed max-w-200 backdrop-blur-sm rounded-xl p-4"
      >
        Nous fusionnons créativité et technologie pour vous assister au quotidien. Devenez
        remarquable avec une équipe qui étend vos capacités digitales.
      </p>
      <div class="flex flex-col sm:flex-row gap-4 mt-4">
        <button
          class="flex items-center gap-2 cursor-pointer rounded-full h-14 px-8 bg-[#c62d6a] text-white text-base font-bold shadow-lg shadow-primary/40 hover:scale-105 hover:bg-primary/90 transition-all"
        >
          <span>Démarrer un projet</span>
          <span class="material-symbols-outlined">arrow_forward</span>
        </button>
        <button
          class="flex items-center gap-2 cursor-pointer rounded-full h-14 px-8 bg-transparent border-2 border-white/20 hover:bg-white/10 text-white text-base font-bold transition-all backdrop-blur-md"
        >
          <span class="material-symbols-outlined">play_circle</span>
          <span>Voir la démo</span>
        </button>
      </div>
    </div>

    <!-- Indicateur de scroll -->
    <div
      class="absolute bottom-10 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-white/50 animate-bounce"
    >
      <span class="text-xs uppercase tracking-widest">Découvrir</span>
      <span class="material-symbols-outlined">keyboard_arrow_down</span>
    </div>
  </div>
</template>

<style scoped>
.hero-gradient {
  background:
    radial-gradient(circle at top left, rgba(255, 255, 255, 0.18), transparent 45%),
    radial-gradient(circle at bottom right, rgba(255, 255, 255, 0.12), transparent 55%), #1a1b41;
}

.glass-panel {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border: 0.5px solid #c62d6a;
}
</style>
