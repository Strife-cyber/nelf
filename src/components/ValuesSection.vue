<script setup lang="ts">
// Section des valeurs - sera dynamique plus tard
const values = [
  {
    title: 'Expertise Technique',
    description:
      'Des bases solides. Nous utilisons des technologies de pointe sécurisées pour garantir la pérennité de vos projets digitaux.',
    icon: 'verified_user',
    gradient: 'from-[#4c2e6c]/90 via-white/10 to-transparent',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuDv01qkvDqLnrG333XGsXcmJNqNWXxXFcUvTJUHcj3xsPPfb9h5V-v9H0ucEau8BTM1BhIO4RdXBWIsV6FTM_MZ5sRMGHdkDaZqNLwDge1KZgsb78uHmj9WGEyLIpxtwZiKJaSdaRZ47DwuaBB9uMlbitU8XQeGqSTtwobmL7aoTtYgsloXLOlpBEGtaWD6KJIiZ1Jfd0WAY78RuYxs_-eh35gel2fL3Uc7i5UJH9ytpBmiaNWAH_c4wUbyO07z7YVXV8H5ScAIe3E-',
  },
  {
    title: 'Créativité Débridée',
    description:
      'Sortir du cadre. Nos designs ne sont pas seulement beaux, ils racontent votre histoire et engagent émotionnellement vos clients.',
    icon: 'lightbulb',
    gradient: 'from-[#c62d6a]/90 via-white/10 to-transparent',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuBlIgRhZgq41jvD7KqWMBIML5Ly60PL2KvC2ZB5eIrkZr6Yjb-gzfzvUDP9iZIz0xlYDkQPcbEECUDMF7TZT8K40O2-HIVZDbGqLNBe9E0UBwJYvX0gHE-hzUzrByDq_BhJtI5YjCzjxfpOLNjF4VFKmKzKvx0ZSgzqtDEORBRwFgE8hbKm1PoHiHkC4C01SPGRolSAy0vOVSFdyW0TxFjnH7od5jrwEmXiLuKhPxT89zJlAVi04-kBQW6j6WQixuOShcjNQbnTDS71',
  },
  {
    title: 'Partenariat Humain',
    description:
      'Votre troisième main. Nous vous accompagnons pas à pas, avec transparence et réactivité, comme une extension de votre équipe.',
    icon: 'handshake',
    gradient: 'from-[#2b2d75]/90 via-white/10 to-transparent',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuCtoxp3rzMX73BwNp2Z6jf2g8guDeLBl1sAhJNWdjRSJTcd2V3OlPwonIOu9gfVnZ3Z8fzkOoH9K_1f-sfCkavRV9CDwIHQs2QTsVk9wlfzFGIzsWKZBdX7_EuZqhgvMIFa8edApz1mWYsi3ylphhKhC8-6Zsry1a-umIw10sPRKtTMJz8hED5hzJklmb5SlouileAx1tKkL9ReV2W2vWQ0sAZUc2U_QjmddPlGMX8miAWq5-_owGJP1Zs_mFNW-mts4Ks3Z3ASBEKO',
  },
]
</script>

<template>
  <div class="relative py-24 bg-[#121226] overflow-hidden" id="valeurs">
    <!-- Effets de fond -->
    <div class="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
      <div
        class="absolute top-[10%] -left-[5%] w-100 h-100 bg-secondary/10 rounded-full blur-[80px]"
      ></div>
      <div
        class="absolute bottom-[10%] -right-[5%] w-100 h-100 bg-primary/10 rounded-full blur-[80px]"
      ></div>
    </div>

    <div class="layout-content-container max-w-300 mx-auto relative z-10">
      <div class="flex flex-col md:flex-row items-end justify-between gap-8 mb-16">
        <div class="max-w-2xl">
          <h2
            class="text-white text-4xl md:text-5xl font-bold tracking-tight mb-4"
          >
            Ce qu'on <span class="text-primary text-[#c62d6a]">apporte</span>
          </h2>
          <p class="text-gray-400 text-lg">
            Plus qu'un prestataire, NELF est votre partenaire de croissance.
          </p>
        </div>
        <div
          class="hidden md:block h-0.5 flex-1 bg-linear-to-r from-transparent via-gray-700 to-transparent mx-8 mb-4"
        ></div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div
          v-for="(value, index) in values"
          :key="index"
          :class="[
            'relative h-100 rounded-3xl overflow-hidden group cursor-pointer',
            index === 1 ? 'mt-0 md:-mt-8' : '',
          ]"
        >
          <div
            class="absolute inset-0 bg-cover bg-center transition-transform duration-700 group-hover:scale-110"
            :style="{ backgroundImage: `url(${value.image})` }"
          ></div>
          <div
            :class="[
              'absolute inset-0 bg-linear-to-t opacity-90 transition-opacity duration-300',
              value.gradient,
            ]"
          ></div>
          <div class="absolute inset-0 p-8 flex flex-col justify-end">
            <div
              class="size-14 bg-white/10 backdrop-blur-md rounded-full flex items-center justify-center text-white mb-6 border border-white/20 group-hover:bg-primary group-hover:border-primary transition-colors duration-300"
            >
              <span class="material-symbols-outlined text-2xl">{{ value.icon }}</span>
            </div>
            <h3
              class="text-white text-2xl font-bold mb-3 transform translate-y-2 group-hover:translate-y-0 transition-transform duration-300"
            >
              {{ value.title }}
            </h3>
            <p
              class="text-gray-200 text-sm leading-relaxed opacity-0 group-hover:opacity-100 transform translate-y-4 group-hover:translate-y-0 transition-all duration-300"
            >
              {{ value.description }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Styles spécifiques si nécessaire */
</style>
