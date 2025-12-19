<script setup lang="ts">
import InfiniteMenu from './InfiniteMenu.vue'
import afficheImage from '@/assets/affiche.jpeg'
import affiche2Image from '@/assets/affiche2.jpeg'
import affiche3Image from '@/assets/affiche3.jpeg'
import type { InfiniteMenuItem } from '@/types/infiniteMenu'

const menuItems: InfiniteMenuItem[] = [
  {
    image: afficheImage,
    title: 'Location d\'Appartements',
    description: 'Guest House - Kribi, Douala, Yaoundé, Limbe. Pour tout type de location d\'appartement pour vos séjours.',
    link: '#',
  },
  {
    image: affiche2Image,
    title: 'DAYCARE',
    description: 'En l\'honneur de la Journée Mondiale des Soins Palliatifs. Partageons ensemble des moments de joie, de relaxation et de lien social.',
    link: '#',
  },
  {
    image: affiche3Image,
    title: 'Buffet Spécial',
    description: 'Jeudi 25.09.2025 à 14h30. À partir de 2500 frs. Billard disponible. Réservation : +237 686677666 / +237 693443782',
    link: '#',
  },
]
</script>

<template>
  <section
    class="py-24 px-4 md:px-12 relative overflow-hidden"
    style="background: linear-gradient(180deg, #ffffff 0%, #f8f8f5 100%);"
    id="realisation"
  >
    <div class="max-w-[1400px] mx-auto">
      <div class="flex justify-between items-end mb-12">
        <div>
          <h2
            class="text-4xl md:text-5xl font-black tracking-tight mb-4"
            style="color: #2b2d75;"
          >
            REALISATION
          </h2>
          <p class="text-gray-500 text-lg">Sélection de travaux et études de cas.</p>
        </div>
      </div>

      <!-- Infinite Menu Container -->
      <div class="relative w-full" style="height: 80vh; min-height: 600px; max-height: 900px;">
        <div
          class="absolute inset-0 rounded-2xl overflow-hidden shadow-2xl"
          style="background: radial-gradient(circle at center, #2b2d75 0%, #1a1b4b 100%);"
        >
          <InfiniteMenu :items="menuItems" :scale="0.8" />
        </div>

        <!-- Instructions overlay -->
        <div
          class="absolute bottom-8 left-1/2 -translate-x-1/2 z-20 bg-black/50 backdrop-blur-md rounded-full px-6 py-3 border border-white/20"
        >
          <p class="text-white text-sm font-medium flex items-center gap-2">
            <span class="material-symbols-outlined text-base">touch_app</span>
            Glissez pour explorer nos réalisations
          </p>
        </div>
      </div>
    </div>
  </section>
</template>
