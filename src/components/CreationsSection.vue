<script setup lang="ts">
// Section créations/portfolio - sera dynamique plus tard
const creations = [
  {
    id: 1,
    title: 'E-Shop Mode Urbaine',
    description:
      "Refonte complète de l'expérience utilisateur et intégration d'un système de paiement custom.",
    category: 'Web',
    year: '2023',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuBbhacO8T_-FOCP07f-YxHcTuITirLANqarGAdVd60zAsd9uVsdgNKIXAjl8NR5Twu6zJhNA0kJxRKL88BLYRdG64PW1FeuQBDvZWMAVg4Dk2OKWt1HEYIjGJGcvFpmq10p89AMFw5UzoCXTWbGrAUZepGO3UFtZETrLJgWKQlj209gl3qcHvgxGl6xHO1v-X0nfVGwY2_OMVIretJUPiSh4wwuI7kUTnaNdMH2BUYsYOBOk3BTPWskXXJvJD1xSuss3nY-OnMukiHx',
    aspect: 'video',
  },
  {
    id: 2,
    title: "Festival d'Été",
    description: '',
    category: 'Print',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuAcZH1J01npofzg3AGp4qqIWTUmTv6AUHkAYEiJUn_-TwyVfOv5yqO-YOU_GqUSMheCfO2fdFbKs7__gspdSLu8jl7FRcjCjqlzKKLjbM_DMQ4VHARS7CgOMeIUbQAJK5gO3mPbHt3VEgqgY3sZF3nb-WmWuHkEU_BhZjsfTdjbNI8ZZBBqYKftXsQ-9fOCa04CzKVsf-hPGrQ_G5eH_J0K4qLdw3WDZl_DBJph9fjATmux_D7fXEvP5s1N8Mzd8RHtmtBgIsNWVlY7',
    aspect: 'portrait',
  },
  {
    id: 3,
    title: 'Showreel 2024',
    description: '',
    category: 'Motion',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuBnzgSd8M4-TUI5xia9HALdNuNwMGTbjH0Lxk07B7ZNe47hVldFMs8f0w8-hgodW7aApU4dclolhbjZDCWU1VXXqc3TTm8yt6Oyg62RkNLXZX6fwvWFvcJ82neK6dQ6KIlFidXIJfjEr-q9U8pbPIs4rZfVHdN74bRvaf4stpkFl97J-If7T9yqYM02r1zm8lG3oOMuK3Q7nRA99Cl0DcQCVYYqZuYo1wy6A809RyNkxd-byQVhB8wqDuOOSEyTc5PR3ppPXv-SZ4WX',
    aspect: 'video',
    isVideo: true,
  },
  {
    id: 4,
    title: 'Conférence Tech',
    description: 'Identité événementielle',
    category: '',
    image:
      'https://lh3.googleusercontent.com/aida-public/AB6AXuCPqO4u3IFBWvjpoQeHxfwV17ltEjIuo1fQF108WaUCElcjwLmJW9im0chWPboy6c1jFKPQHwA1bgDBjt1IUu7xNXDNaQlOy2BTgCHtSWDKgNlbxB0Pc4eRQJSWPH17ejGr9r-v_wTXElUAVWxBBGHt-jLi-r5vCEh9e3_LthoFfsf0qNCyQDA6InE5YECMO0yQcCBAh9KC3nNAAgs84aVapsVUEexFstJPw-QP5a52d56zkLtPKAKPYXo7sFR7viKFKEZvgqzR0HoU',
    aspect: 'square',
  },
]
</script>

<template>
  <div class="py-24 bg-white dark:bg-[#121226] overflow-hidden" id="creations">
    <div class="layout-content-container max-w-350 mx-auto px-4 md:px-10 flex flex-col">
      <div class="flex flex-col md:flex-row justify-between items-end mb-12 gap-6">
        <div>
          <h2 class="text-[#111714] dark:text-white text-4xl font-bold tracking-tight mb-2">
            Nos Créations
          </h2>
          <p class="text-[#637588] dark:text-gray-400 text-lg">
            Découvrez nos réalisations récentes en mode "Showroom".
          </p>
        </div>
        <div class="flex gap-2">
          <span
            class="text-sm text-gray-500 dark:text-gray-400 font-medium mr-2 self-center hidden md:block"
          >
            Glisser pour découvrir
          </span>
          <span class="material-symbols-outlined text-primary animate-pulse">arrow_right_alt</span>
        </div>
      </div>

      <div
        class="flex overflow-x-auto gap-6 pb-12 snap-x snap-mandatory no-scrollbar px-4 -mx-4 md:px-0 md:mx-0"
      >
        <div
          v-for="creation in creations"
          :key="creation.id"
          :class="[
            'flex-none snap-center group relative rounded-2xl overflow-hidden shadow-2xl',
            creation.aspect === 'video' ? 'w-[85vw] md:w-150 aspect-video' : '',
            creation.aspect === 'portrait' ? 'w-[85vw] md:w-100 aspect-9/16 md:aspect-3/4' : '',
            creation.aspect === 'square' ? 'w-[85vw] md:w-100 aspect-square' : '',
            creation.isVideo ? 'bg-gray-900' : '',
          ]"
        >
          <div
            class="absolute inset-0 bg-cover bg-center transition-transform duration-700 group-hover:scale-105"
            :class="creation.isVideo ? 'opacity-80 group-hover:opacity-60' : ''"
            :style="{ backgroundImage: `url(${creation.image})` }"
          ></div>
          <div
            v-if="!creation.isVideo"
            class="absolute inset-0 bg-linear-to-t from-black/80 via-black/20 to-transparent opacity-100 md:opacity-0 md:group-hover:opacity-100 transition-opacity duration-300"
          ></div>
          <div v-if="creation.isVideo" class="absolute inset-0 flex items-center justify-center">
            <div
              class="size-20 rounded-full bg-white/10 backdrop-blur-md border border-white/20 flex items-center justify-center text-white group-hover:scale-110 group-hover:bg-primary group-hover:border-primary transition-all duration-300 cursor-pointer shadow-[0_0_30px_rgba(255,255,255,0.2)]"
            >
              <span class="material-symbols-outlined text-4xl ml-1">play_arrow</span>
            </div>
          </div>
          <div
            :class="[
              'absolute bottom-0 left-0 w-full p-8',
              !creation.isVideo
                ? 'translate-y-0 md:translate-y-4 md:group-hover:translate-y-0 transition-transform duration-300'
                : '',
            ]"
          >
            <div v-if="creation.category" class="flex items-center gap-3 mb-2">
              <span
                :class="[
                  'px-3 py-1 text-white text-xs font-bold rounded-full uppercase tracking-wide',
                  creation.category === 'Web' ? 'bg-primary' : '',
                  creation.category === 'Print' ? 'bg-white text-secondary' : '',
                  creation.category === 'Motion' ? 'bg-accent' : '',
                ]"
              >
                {{ creation.category }}
              </span>
              <span v-if="creation.year" class="text-gray-300 text-xs uppercase tracking-wider">
                {{ creation.year }}
              </span>
            </div>
            <h3 class="text-white text-2xl font-bold mb-2">{{ creation.title }}</h3>
            <p v-if="creation.description" class="text-gray-300 text-sm line-clamp-2">
              {{ creation.description }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
