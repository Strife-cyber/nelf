<script setup lang="ts">
// Section équipe - sera dynamique plus tard
const teamMembers = [
  {
    id: 1,
    name: 'Jean Dupont',
    role: 'Fondateur & CEO',
    description: "Visionnaire stratégique, il guide NELF vers l'innovation constante.",
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuDRidZ-rAMvGQpB1EHzw-RsjoLORXG7r8D0ZDcX7m1c-IjM0TD3_F6JHTUGHzaT7a0TjqE1NX3D8YZgc-6DkbgoaYv-6cOxiJjAgSI7ipw33kF5AM54TAso2ie7YCZfAa1dQuKU3EVpqnzbhG-vbVtUXjGKJI_CRUJkhHesrNmHHM2DgzC3xBTA7_bcp-eFNSti_VLJxt_GiwdXd9BkKOrSD8L2-ke3NwmTv5-_F3EAWFi4q5RIqtKLiZXugN_3yRGIlrtfOWu53xD8',
    gradient: 'from-[#4c2e6c] via-[#4c2e6c]/50 to-transparent',
  },
  {
    id: 2,
    name: 'Marie Curie',
    role: 'Directrice Artistique',
    description: 'Elle transforme les concepts abstraits en visuels percutants.',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuA_Aiyseo4wnkxbmelaA8Hp9P5ebsAhpMwFviFYVzz0zSXFtpJiIVJ358iVr3kWB-S4p_BqEw5kFM6LQvlddTVylFdsRa3Man91l7hqp6leuoEENSZ0rdP3PVLchNmFyKm3zPYJAnsIeY0v5hEaTIbMZfxn0GqfS3T8xuTkMECJcDJv6eXeobQAponzp1zipHK5FvrkutYV5icFS3ZE3oKTKCd0Oq3xN5q3V6gHwPv7sP3bGMhez_60HaW07nozICqtEWmsjc66yWqt',
    gradient: 'from-[#c62d6a] via-[#c62d6a]/50 to-transparent',
  },
  {
    id: 3,
    name: 'Pierre Martin',
    role: 'Lead Développeur',
    description: "L'architecte du code qui rend possible l'impossible.",
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuBVUxhRPquQhQ3Mt7a-hMW0sgtyP1Z31nGnjjjlwcpwZ_4gUUef9pebUP-lMp12NimfceqBCcmN4zjDpQIGyporjTO-2g1QiQIjl3EgtPQhgtUNHkfDOWUEJzC3Kjgxuou1I94o4VnABb--QeOb66ztpjV-6KrXYxtKztRq1radYTJYo_7HotYW_gXvdl1dn_DwuVe-Y2TQFbdtkcEMlAHFvd84rK3TmcNOw8Ib_DgL7FSffWmWTM7DvRWZuL_gKXNnwnPit8QH9e83',
    gradient: 'from-[#2b2d75] via-[#2b2d75]/50 to-transparent',
  },
]
</script>

<template>
  <div class="py-24 bg-white dark:bg-[#121226]" id="equipe">
    <div class="layout-content-container max-w-300 mx-auto px-4 md:px-10">
      <div class="text-center mb-16">
        <span class="text-[#c62d6a] font-bold tracking-widest uppercase text-sm mb-2 block">
          Le Cœur de NELF
        </span>
        <h2 class="text-[#111714] dark:text-white text-4xl font-bold leading-tight">
          Notre Équipe
        </h2>
      </div>

      <div class="flex flex-wrap justify-center gap-10">
        <div
          v-for="member in teamMembers"
          :key="member.id"
          class="group relative w-72 h-96 rounded-2xl overflow-hidden shadow-xl cursor-pointer"
        >
          <img
            :alt="`Portrait of ${member.name}`"
            :src="member.image"
            class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
          />
          <div
            :class="[
              'absolute inset-0 bg-linear-to-t opacity-60 group-hover:opacity-90 transition-opacity duration-300',
              member.gradient,
            ]"
          ></div>
          <div
            class="absolute bottom-0 left-0 w-full p-6 text-white transform translate-y-12 group-hover:translate-y-0 transition-transform duration-300"
          >
            <h3 class="text-2xl font-bold">{{ member.name }}</h3>
            <p
              :class="[
                'font-medium mb-4',
                member.role.includes('CEO') || member.role.includes('Dév')
                  ? 'text-[#c62d6a]'
                  : 'text-white/80',
              ]"
            >
              {{ member.role }}
            </p>
            <p
              class="text-sm text-gray-200 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-100 mb-4"
            >
              {{ member.description }}
            </p>
            <div
              class="flex gap-4 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-200"
            >
              <span
                :class="[
                  'material-symbols-outlined text-xl transition-colors',
                  member.role.includes('Artistique') ? 'hover:text-white' : 'hover:text-[#c62d6a]',
                ]"
              >
                mail
              </span>
              <span
                :class="[
                  'material-symbols-outlined text-xl transition-colors',
                  member.role.includes('Artistique') ? 'hover:text-white' : 'hover:text-[#c62d6a]',
                ]"
              >
                {{ member.role.includes('Artistique') ? 'brush' : 'link' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
