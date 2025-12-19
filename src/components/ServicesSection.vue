<script setup lang="ts">
import ServiceCard from './ServiceCard.vue'

const services = [
  {
    icon: 'lightbulb',
    title: 'Stratégie',
    description:
      'Définir le chemin vers le succès avec des insights basés sur les données. Nous cartographions le territoire avant de construire la route.',
    hoverColor: 'brand-deep' as const,
  },
  {
    icon: 'palette',
    title: 'Design',
    description:
      'Créer des interfaces visuellement époustouflantes et centrées sur l\'utilisateur. Nous créons des expériences dont les utilisateurs tombent amoureux.',
    hoverColor: 'brand-magenta' as const,
  },
  {
    icon: 'code',
    title: 'Développement',
    description:
      'Construire des solutions numériques robustes et évolutives. Code propre, performance rapide et architecture tournée vers l\'avenir.',
    hoverColor: 'brand-deep' as const,
  },
]
</script>

<template>
  <section class="py-24 px-4 md:px-12 bg-white relative" id="services">
    <div class="max-w-[1200px] mx-auto flex flex-col gap-16">
      <div
        class="flex flex-col md:flex-row justify-between items-start md:items-end gap-6 border-b border-gray-100 pb-8"
      >
        <div class="flex flex-col gap-3">
          <span
            class="font-bold tracking-widest text-sm uppercase"
            style="color: #c62d6a;"
            >Services</span
          >
          <h2
            class="text-4xl md:text-5xl font-black tracking-tight"
            style="color: #2b2d75;"
          >
            CE QU'ON FAIT
          </h2>
        </div>
        <p class="text-gray-500 max-w-md text-base md:text-lg">
          Services numériques complets adaptés à vos besoins, définissant le chemin vers le succès
          avec des insights basés sur les données.
        </p>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <ServiceCard
          v-for="service in services"
          :key="service.title"
          :icon="service.icon"
          :title="service.title"
          :description="service.description"
          :hover-color="service.hoverColor"
        />
      </div>
    </div>
  </section>
</template>
