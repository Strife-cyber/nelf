<script setup lang="ts">
const scrollToSection = (id: string) => {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
  }
}
</script>

<template>
  <section
    class="relative min-h-[90vh] flex flex-col items-center justify-center px-4 pt-24 pb-12 overflow-hidden"
    style="background: radial-gradient(circle at center, #c62d6a 0%, #4c2e6c 50%, #2b2d75 100%);"
  >
    <!-- Background Elements -->
    <div
      class="absolute inset-0 z-0 opacity-40 mix-blend-overlay bg-[url('https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?q=80&w=2564&auto=format&fit=crop')] bg-cover bg-center"
    ></div>
    <div
      class="absolute top-0 right-0 w-[500px] h-[500px] opacity-20 blur-[120px] rounded-full translate-x-1/3 -translate-y-1/3"
      style="background-color: #c62d6a;"
    ></div>
    <div
      class="absolute bottom-0 left-0 w-[600px] h-[600px] opacity-30 blur-[100px] rounded-full -translate-x-1/3 translate-y-1/3"
      style="background-color: #4c2e6c;"
    ></div>

    <div class="relative z-10 container mx-auto flex flex-col items-center text-center max-w-4xl">
      <div class="mb-8">
        <img src="/src/assets/logo.png" alt="NELF Logo" class="h-32 md:h-48 w-auto mx-auto" />
        <p class="text-white text-lg md:text-xl mt-4 font-light">VOTRE TROISIÈME MAIN</p>
      </div>
      <div class="flex flex-col items-center gap-6">
        <h2
          class="text-white text-4xl md:text-6xl font-bold leading-tight tracking-tight max-w-3xl"
        >
          Architecturer les <span class="text-white italic font-light">Futurs Numériques</span>
        </h2>
        <p class="text-white/90 text-lg md:text-xl font-normal max-w-2xl leading-relaxed">
          Nous mélangeons le chaos créatif avec la précision stratégique pour construire des
          marques qui ne se contentent pas d'exister, mais qui dominent.
        </p>
        <div class="flex gap-4 mt-4">
          <button
            class="flex h-14 px-8 items-center justify-center rounded-full text-white text-base font-bold tracking-wide transition-all"
            style="background-color: #c62d6a;"
            @click="scrollToSection('realisation')"
          >
            Voir nos Réalisations
          </button>
          <button
            class="flex h-14 px-8 items-center justify-center rounded-full border-2 border-white/50 bg-white/10 backdrop-blur-sm text-white text-base font-medium hover:bg-white/20 transition-all"
            @click="scrollToSection('apport')"
          >
            Notre Processus
          </button>
        </div>
      </div>
    </div>

    <!-- Scroll Indicator -->
    <div class="absolute bottom-8 left-1/2 -translate-x-1/2 text-white/50 animate-bounce">
      <span class="material-symbols-outlined">keyboard_arrow_down</span>
    </div>
  </section>
</template>
