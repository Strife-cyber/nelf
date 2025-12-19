<script setup lang="ts">
import ProjectCard from './ProjectCard.vue'

const projects = [
  {
    category: 'Fintech',
    title: 'Nova Banking App',
    description:
      'Une refonte complète de l\'expérience bancaire mobile axée sur l\'accessibilité.',
    imageUrl:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuBL1aRB__XioxDBQRDIDSSTq3j1kYSaZ7qM6v4ifLRvLb9w7xhSlcTQ_9Noxr3pszUjKLqRUvKzWiFCmFl77Evo6ov9VLUuzNN_WLSLzKJwsikZnLSipJcDtnoFqePeCHCaPUtLbgIE8RLUK-pQEsQQNNRbeeXaiWthYpGGdiQMOjwqsLFL7S_G81rmmLM4eewFooVrzm431GUdnP0jhcCDEjWv48wlyz_2MOUKX9b3It7B0O_M1ODWV2Mhvx4PESnaJkcQnoJ-FqU',
    imageAlt: 'Interface de tableau de bord moderne sur un écran d\'ordinateur portable',
    gradientColor: 'brand-deep' as const,
  },
  {
    category: 'E-Commerce',
    title: 'Luxe Fashion',
    description:
      'Élever le parcours d\'achat en ligne avec une narration visuelle immersive.',
    imageUrl:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuAEE_PghinPInUiubz1_SWexQfq1OYg5Zgmlml3RvRYkKH-BOKcuar2zLxnxrjud6BfeooxNZVi-BaqsZwxiCNzDla-PgZzwR88uqPK4kk9Ybw-_qVylqBMVBYjKp3IDBUmkew0LwWiAS_fApsXNFdiK-m5-rpYXPbczolizezmPwwNWqvAksYSG0O4HY7BmV284abFUlMnLUSLJnd07lJb1N8YBo2yIxjfjJvE6y1mdqkQUsi4BDH4ovE-FY3OWgF92mN1MDanLfs',
    imageAlt: 'Mise en page de site e-commerce minimaliste avec articles de mode',
    gradientColor: 'brand-magenta' as const,
  },
  {
    category: 'Branding',
    title: 'Vertex Identity',
    description:
      'Création d\'un langage de marque cohérent pour une startup technologique de premier plan.',
    imageUrl:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuC1f7q31aYlf81meX2UnFEjwt8-l0t0yDHfkjhQdh_XdfE6dHMJM7r9ZpWz-jXHqT4xl9_QIwbXnn8OJU-1kLwZMCfEWqG-RQKooZGu47DK-Awtkxd4bby0Rt_mSaQ-u8TDdzhZTQacxFUGDThDtCpQUDJ88I2m5_rdxygihRNjRP6xg6c1EmmIMBCb_qoWTISbkwGXQgSrjBaQWwy6xrv9z7mz2t55Tyx41FwURQODOhUeDs_JomLAN0Axc0zylI74MZebttZeL6s',
    imageAlt: 'Formes géométriques 3D abstraites représentant une structure de données',
    gradientColor: 'brand-deep' as const,
  },
]
</script>

<template>
  <section class="py-24 px-4 md:px-12 bg-white" id="realisation">
    <div class="max-w-[1200px] mx-auto">
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
        <button
          class="hidden md:flex items-center gap-2 font-bold hover:gap-4 transition-all"
          style="color: #c62d6a;"
        >
          Voir Tous les Projets <span class="material-symbols-outlined">arrow_forward</span>
        </button>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <ProjectCard
          v-for="project in projects"
          :key="project.title"
          :category="project.category"
          :title="project.title"
          :description="project.description"
          :image-url="project.imageUrl"
          :image-alt="project.imageAlt"
          :gradient-color="project.gradientColor"
        />
      </div>
      <div class="mt-8 text-center md:hidden">
        <button
          class="px-6 py-3 rounded-full border font-bold text-sm"
          style="border-color: #2b2d75; color: #2b2d75;"
        >
          Voir Tous les Projets
        </button>
      </div>
    </div>
  </section>
</template>
