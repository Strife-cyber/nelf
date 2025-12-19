<script setup lang="ts">
const scrollToSection = (id: string) => {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
  }
}
</script>

<template>
  <section class="py-20 px-4 md:px-12 bg-background-light" id="apport">
    <div class="max-w-[1200px] mx-auto">
      <div class="flex flex-col lg:flex-row gap-12 items-center">
        <div
          class="w-full lg:w-1/2 aspect-square lg:aspect-[4/3] rounded-xl overflow-hidden relative group"
        >
          <div
            class="absolute inset-0 bg-[#2b2d75]/20 group-hover:bg-transparent transition-colors z-10"
          ></div>
          <img
            alt="Futuristic abstract data visualization with glowing nodes"
            class="w-full h-full object-cover transform group-hover:scale-110 transition-transform duration-700"
            src="https://lh3.googleusercontent.com/aida-public/AB6AXuDL_Kbjx0xB3cYeVVbkR-kQqgf5krUjebFotPj7U35_P6508aHQpQdwS-IQ2AGM3ulPhbqxhtVjGw6zD4NFgV1q7rJl_Wi2xdVUeiLaA2zx9NWdWqd88irr3g8rnvPoXBx9NU-fks92UKZvMA1YS4NTra4ZgS84sZXyi4U5NPOYye1-sOjK4sAIQMHwW09sSqLfe7tlxpHhkPs9yECyLpiJZFjuPTSYjDBeoLc2UQcxeNtzMCQ4qNPvCrkLvDFD_pv2_LWxaVwG3oM"
          />
        </div>
        <div class="w-full lg:w-1/2 flex flex-col gap-8">
          <div>
            <span
              class="font-bold tracking-widest text-sm uppercase mb-2 block"
              style="color: #c62d6a;"
              >Proposition de Valeur</span
            >
            <h2
              class="text-4xl md:text-5xl font-black leading-tight tracking-tight mb-6"
              style="color: #2b2d75;"
            >
              APPORT
            </h2>
            <div
              class="w-20 h-2 rounded-full mb-6"
              style="background-color: #c62d6a;"
            ></div>
            <p class="text-gray-700 text-lg md:text-xl leading-relaxed mb-6 font-medium">
              Nous apportons de la clarté à la complexité, en offrant une valeur distincte grâce à
              un design innovant et une réflexion stratégique.
            </p>
            <p class="text-gray-500 text-base leading-relaxed">
              Notre approche garantit un ROI mesurable et une croissance durable. Nous ne livrons
              pas seulement un produit ; nous livrons un partenariat qui évolue avec votre ambition.
              En comblant le fossé entre les besoins des utilisateurs et les objectifs commerciaux,
              nous créons des écosystèmes numériques qui prospèrent.
            </p>
          </div>
          <div class="flex flex-col sm:flex-row gap-4">
            <div class="flex items-center gap-3">
              <span class="material-symbols-outlined text-3xl" style="color: #c62d6a;"
                >check_circle</span
              >
              <span class="font-bold" style="color: #2b2d75;">ROI Mesurable</span>
            </div>
            <div class="flex items-center gap-3">
              <span class="material-symbols-outlined text-3xl" style="color: #c62d6a;"
                >check_circle</span
              >
              <span class="font-bold" style="color: #2b2d75;">Croissance Durable</span>
            </div>
          </div>
          <button
            class="w-fit flex items-center justify-center rounded-full h-12 px-8 text-white text-sm font-bold tracking-wide transition-colors mt-4 hover:bg-[#4c2e6c]"
            style="background-color: #2b2d75;"
            @click="scrollToSection('vision')"
          >
            En Savoir Plus sur Notre Impact
          </button>
        </div>
      </div>
    </div>
  </section>
</template>
