<script setup lang="ts">
// Section services - structure originale du HTML
const services = [
  {
    id: 1,
    title: 'Développement Web',
    description:
      'Architectures robustes et interfaces fluides. Nous bâtissons des écosystèmes digitaux (Vitrines, E-commerce, Apps) taillés pour la performance.',
    icon: 'terminal',
  },
  {
    id: 2,
    title: 'Design & Branding',
    description:
      "Identités visuelles qui marquent les esprits. Du logo à la charte graphique complète, nous forgeons l'âme visuelle de votre marque.",
    icon: 'brush',
  },
  {
    id: 3,
    title: 'Motion & Contenu',
    description:
      "Donnez vie à votre message. Animations dynamiques, vidéos promotionnelles et stratégies de contenu pour capter l'attention instantanément.",
    icon: 'movie_filter',
  },
]
</script>

<template>
  <div class="relative py-24 bg-[#121226]" id="services">
    <div class="layout-content-container max-w-300 mx-auto px-4 md:px-10">
      <div class="flex flex-col gap-4 mb-16 text-center">
        <h2 class="text-white text-4xl md:text-5xl font-bold tracking-tight">
          Ce que nous
          <span class="text-[#c62d6a] relative inline-block">
            faisons
            <svg
              class="absolute w-full h-3 -bottom-1 left-0 text-primary opacity-40"
              preserveAspectRatio="none"
              viewBox="0 0 100 10"
            >
              <path
                d="M0 5 Q 50 10 100 5"
                fill="none"
                stroke="currentColor"
                stroke-width="3"
              ></path>
            </svg>
          </span>
        </h2>
        <p class="text-gray-400 text-lg max-w-2xl mx-auto">
          Une expertise 360° pour concrétiser vos ambitions avec précision et audace.
        </p>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div
          v-for="service in services"
          :key="service.id"
          class="group relative p-8 bg-accent/20 rounded-2xl overflow-hidden hover:bg-accent/40 transition-all duration-300 border border-transparent hover:border-primary/30 shadow-sm hover:shadow-xl hover:shadow-primary/10"
        >
          <div
            class="absolute -right-10 -top-10 size-40 bg-primary/5 rounded-full blur-2xl group-hover:bg-primary/10 transition-colors"
          ></div>
          <div class="relative z-10 flex flex-col gap-6">
            <div
              class="size-16 rounded-2xl bg-linear-to-br from-[#4c2e6c] to-[#c62d6a] flex items-center justify-center text-white shadow-lg transform group-hover:-translate-y-2 transition-transform duration-300"
            >
              <span class="material-symbols-outlined text-3xl">{{ service.icon }}</span>
            </div>
            <div>
              <h3
                class="text-2xl font-bold mb-3 text-white group-hover:text-primary transition-colors"
              >
                {{ service.title }}
              </h3>
              <p class="text-gray-300 leading-relaxed">
                {{ service.description }}
              </p>
            </div>
            <a
              class="inline-flex items-center text-[#c62d6a] font-bold text-sm mt-2 hover:translate-x-2 transition-transform"
              href="#"
            >
              En savoir plus
              <span class="material-symbols-outlined text-base ml-1">arrow_right_alt</span>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
