<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { gsap } from 'gsap'
import CardSwap from './CardSwap.vue'
import afficheImage from '@/assets/affiche.jpeg'
import affiche2Image from '@/assets/affiche2.jpeg'
import affiche3Image from '@/assets/affiche3.jpeg'

const heroRef = ref<HTMLElement | null>(null)
const mouseX = ref(0)
const mouseY = ref(0)

// Références pour les éléments 3D
const cube1Ref = ref<HTMLElement | null>(null)
const cube2Ref = ref<HTMLElement | null>(null)
const cube3Ref = ref<HTMLElement | null>(null)
const sphere1Ref = ref<HTMLElement | null>(null)
const sphere2Ref = ref<HTMLElement | null>(null)
const pyramidRef = ref<HTMLElement | null>(null)
const torusRef = ref<HTMLElement | null>(null)

// Images pour les cartes - différentes affiches
const cardImages = ref<string[]>([
  afficheImage,
  affiche2Image,
  affiche3Image,
  afficheImage,
  affiche2Image,
])

// État pour le carousel mobile
const currentCarouselIndex = ref(0)
const carouselInterval = ref<number | null>(null)
const touchStartX = ref(0)
const touchEndX = ref(0)

// Navigation du carousel
const nextCarouselSlide = () => {
  currentCarouselIndex.value = (currentCarouselIndex.value + 1) % cardImages.value.length
  stopCarouselAutoPlay()
  setTimeout(() => startCarouselAutoPlay(), 5000)
}

const prevCarouselSlide = () => {
  currentCarouselIndex.value =
    currentCarouselIndex.value === 0 ? cardImages.value.length - 1 : currentCarouselIndex.value - 1
  stopCarouselAutoPlay()
  setTimeout(() => startCarouselAutoPlay(), 5000)
}

// Gestion du swipe tactile
const handleTouchStart = (e: TouchEvent) => {
  if (e.touches && e.touches[0]) {
    touchStartX.value = e.touches[0].clientX
  }
}

const handleTouchEnd = (e: TouchEvent) => {
  if (e.changedTouches && e.changedTouches[0]) {
    touchEndX.value = e.changedTouches[0].clientX
    handleSwipe()
  }
}

const handleSwipe = () => {
  const swipeThreshold = 50
  const diff = touchStartX.value - touchEndX.value

  if (Math.abs(diff) > swipeThreshold) {
    if (diff > 0) {
      // Swipe gauche - image suivante
      nextCarouselSlide()
    } else {
      // Swipe droite - image précédente
      prevCarouselSlide()
    }
  }
}

// Auto-play pour le carousel
const startCarouselAutoPlay = () => {
  if (carouselInterval.value) {
    clearInterval(carouselInterval.value)
  }
  carouselInterval.value = window.setInterval(() => {
    nextCarouselSlide()
  }, 4000)
}

const stopCarouselAutoPlay = () => {
  if (carouselInterval.value) {
    clearInterval(carouselInterval.value)
    carouselInterval.value = null
  }
}

// Initialiser les animations 3D
const init3DAnimations = () => {
  // Animation du cube 1 - rotation continue
  if (cube1Ref.value) {
    gsap.to(cube1Ref.value, {
      rotationX: 360,
      rotationY: 360,
      duration: 20,
      repeat: -1,
      ease: 'none',
    })
    gsap.to(cube1Ref.value, {
      y: '+=30',
      duration: 3,
      repeat: -1,
      yoyo: true,
      ease: 'power1.inOut',
    })
  }

  // Animation du cube 2 - rotation inverse
  if (cube2Ref.value) {
    gsap.to(cube2Ref.value, {
      rotationX: -360,
      rotationY: -360,
      duration: 25,
      repeat: -1,
      ease: 'none',
    })
    gsap.to(cube2Ref.value, {
      x: '+=40',
      duration: 4,
      repeat: -1,
      yoyo: true,
      ease: 'power1.inOut',
    })
  }

  // Animation du cube 3 - mouvement complexe
  if (cube3Ref.value) {
    gsap.to(cube3Ref.value, {
      rotationZ: 360,
      duration: 15,
      repeat: -1,
      ease: 'none',
    })
    gsap.to(cube3Ref.value, {
      y: '-=50',
      x: '+=30',
      duration: 5,
      repeat: -1,
      yoyo: true,
      ease: 'power2.inOut',
    })
  }

  // Animation de la sphère 1 - pulsation
  if (sphere1Ref.value) {
    gsap.to(sphere1Ref.value, {
      scale: 1.2,
      duration: 2,
      repeat: -1,
      yoyo: true,
      ease: 'power1.inOut',
    })
    gsap.to(sphere1Ref.value, {
      rotationX: 360,
      rotationY: 360,
      duration: 18,
      repeat: -1,
      ease: 'none',
    })
  }

  // Animation de la sphère 2 - flottement
  if (sphere2Ref.value) {
    gsap.to(sphere2Ref.value, {
      y: '+=60',
      x: '-=40',
      duration: 6,
      repeat: -1,
      yoyo: true,
      ease: 'sine.inOut',
    })
    gsap.to(sphere2Ref.value, {
      rotationX: -360,
      rotationY: 360,
      duration: 22,
      repeat: -1,
      ease: 'none',
    })
  }

  // Animation de la pyramide
  if (pyramidRef.value) {
    gsap.to(pyramidRef.value, {
      rotationY: 360,
      rotationX: 180,
      duration: 12,
      repeat: -1,
      ease: 'none',
    })
    gsap.to(pyramidRef.value, {
      z: '+=100',
      duration: 4,
      repeat: -1,
      yoyo: true,
      ease: 'power1.inOut',
    })
  }

  // Animation du tore
  if (torusRef.value) {
    gsap.to(torusRef.value, {
      rotationX: 360,
      rotationZ: 360,
      duration: 16,
      repeat: -1,
      ease: 'none',
    })
    gsap.to(torusRef.value, {
      scale: 0.8,
      duration: 3,
      repeat: -1,
      yoyo: true,
      ease: 'power1.inOut',
    })
  }
}

// Animation parallaxe 3D basée sur la souris
const update3DParallax = () => {
  const parallaxStrength = 20

  if (cube1Ref.value) {
    gsap.to(cube1Ref.value, {
      rotationY: mouseX.value * parallaxStrength,
      rotationX: mouseY.value * parallaxStrength,
      duration: 1,
      ease: 'power2.out',
    })
  }

  if (cube2Ref.value) {
    gsap.to(cube2Ref.value, {
      rotationY: -mouseX.value * parallaxStrength * 0.7,
      rotationX: -mouseY.value * parallaxStrength * 0.7,
      x: mouseX.value * parallaxStrength * 0.5,
      y: mouseY.value * parallaxStrength * 0.5,
      duration: 1,
      ease: 'power2.out',
    })
  }

  if (sphere1Ref.value) {
    gsap.to(sphere1Ref.value, {
      x: mouseX.value * parallaxStrength * 0.6,
      y: mouseY.value * parallaxStrength * 0.6,
      duration: 1,
      ease: 'power2.out',
    })
  }

  if (pyramidRef.value) {
    gsap.to(pyramidRef.value, {
      rotationY: mouseX.value * parallaxStrength * 0.8,
      x: -mouseX.value * parallaxStrength * 0.4,
      duration: 1,
      ease: 'power2.out',
    })
  }
}

onMounted(() => {
  // Démarrer l'auto-play du carousel sur mobile
  if (window.innerWidth < 1024) {
    startCarouselAutoPlay()
  }

  // Initialiser les animations 3D après un court délai
  setTimeout(() => {
    init3DAnimations()
  }, 300)
})

onUnmounted(() => {
  stopCarouselAutoPlay()
})

// Parallax & Glow Logic
const handleMouseMove = (e: MouseEvent) => {
  mouseX.value = (e.clientX / window.innerWidth) * 2 - 1
  mouseY.value = (e.clientY / window.innerHeight) * 2 - 1

  if (heroRef.value) {
    const x = e.clientX
    const y = e.clientY
    heroRef.value.style.setProperty('--x', `${x}px`)
    heroRef.value.style.setProperty('--y', `${y}px`)
  }

  // Mettre à jour le parallaxe 3D
  update3DParallax()
}

// Smooth Scroll
const scrollToSection = (id: string) => {
  document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' })
}

// Référence au composant CardSwap pour déclencher manuellement le swap
const cardSwapRef = ref<InstanceType<typeof CardSwap> | null>(null)

// Gérer le clic sur une carte - déclencher le swap pour voir la carte suivante
const handleCardClick = () => {
  // Le swap est géré directement dans CardSwap au clic
  // Cette fonction peut être utilisée pour d'autres actions si nécessaire
}
</script>

<template>
  <main
    ref="heroRef"
    class="relative h-screen w-full overflow-hidden bg-black text-white selection:bg-[#c62d6a] selection:text-white"
    @mousemove="handleMouseMove"
  >
    <div
      class="absolute inset-0 z-0 transition-transform duration-100 ease-out"
      style="
        background: radial-gradient(
          circle at var(--x, 50%) var(--y, 50%),
          #c62d6a 0%,
          #4c2e6c 40%,
          #1a1b4b 100%
        );
        transform: scale(1.1);
      "
    ></div>

    <div
      class="absolute inset-0 z-[1] opacity-20 mix-blend-overlay pointer-events-none"
      style="background-image: url('https://grainy-gradients.vercel.app/noise.svg')"
    ></div>

    <!-- Éléments 3D animés avec GSAP -->
    <div class="absolute inset-0 z-[3] pointer-events-none overflow-hidden">
      <!-- Cube 1 -->
      <div
        ref="cube1Ref"
        class="absolute top-20 left-10 w-24 h-24 opacity-15"
        style="transform-style: preserve-3d"
      >
        <div class="w-full h-full" style="transform-style: preserve-3d">
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateY(0deg) translateZ(48px);
            "
          ></div>
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateY(90deg) translateZ(48px);
            "
          ></div>
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateY(180deg) translateZ(48px);
            "
          ></div>
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateY(-90deg) translateZ(48px);
            "
          ></div>
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateX(90deg) translateZ(48px);
            "
          ></div>
          <div
            class="absolute w-full h-full border-2 border-[#c62d6a]/30"
            style="
              background: linear-gradient(135deg, rgba(198, 45, 106, 0.1), rgba(76, 46, 108, 0.1));
              transform: rotateX(-90deg) translateZ(48px);
            "
          ></div>
        </div>
      </div>

      <!-- Cube 2 -->
      <div
        ref="cube2Ref"
        class="absolute top-1/3 right-20 w-16 h-16 opacity-12"
        style="transform-style: preserve-3d"
      >
        <div class="w-full h-full" style="transform-style: preserve-3d">
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateY(0deg) translateZ(32px);
            "
          ></div>
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateY(90deg) translateZ(32px);
            "
          ></div>
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateY(180deg) translateZ(32px);
            "
          ></div>
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateY(-90deg) translateZ(32px);
            "
          ></div>
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateX(90deg) translateZ(32px);
            "
          ></div>
          <div
            class="absolute w-full h-full border border-[#4c2e6c]/40"
            style="
              background: linear-gradient(45deg, rgba(76, 46, 108, 0.15), rgba(198, 45, 106, 0.15));
              transform: rotateX(-90deg) translateZ(32px);
            "
          ></div>
        </div>
      </div>

      <!-- Cube 3 -->
      <div
        ref="cube3Ref"
        class="absolute bottom-32 left-1/4 w-20 h-20 opacity-10"
        style="transform-style: preserve-3d"
      >
        <div class="w-full h-full" style="transform-style: preserve-3d">
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateY(0deg) translateZ(40px)"
          ></div>
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateY(90deg) translateZ(40px)"
          ></div>
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateY(180deg) translateZ(40px)"
          ></div>
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateY(-90deg) translateZ(40px)"
          ></div>
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateX(90deg) translateZ(40px)"
          ></div>
          <div
            class="absolute w-full h-full border border-white/20"
            style="background: rgba(198, 45, 106, 0.1); transform: rotateX(-90deg) translateZ(32px)"
          ></div>
        </div>
      </div>

      <!-- Sphère 1 -->
      <div
        ref="sphere1Ref"
        class="absolute top-1/2 right-1/4 w-32 h-32 opacity-12"
        style="transform-style: preserve-3d"
      >
        <div
          class="w-full h-full rounded-full border-2 border-[#c62d6a]/30"
          style="
            background: radial-gradient(
              circle at 30% 30%,
              rgba(198, 45, 106, 0.2),
              rgba(76, 46, 108, 0.1)
            );
            box-shadow:
              inset -20px -20px 40px rgba(0, 0, 0, 0.3),
              inset 20px 20px 40px rgba(255, 255, 255, 0.1);
          "
        ></div>
      </div>

      <!-- Sphère 2 -->
      <div
        ref="sphere2Ref"
        class="absolute bottom-20 right-10 w-28 h-28 opacity-10"
        style="transform-style: preserve-3d"
      >
        <div
          class="w-full h-full rounded-full border border-[#4c2e6c]/40"
          style="
            background: radial-gradient(
              circle at 40% 40%,
              rgba(76, 46, 108, 0.25),
              rgba(198, 45, 106, 0.15)
            );
            box-shadow:
              inset -15px -15px 30px rgba(0, 0, 0, 0.3),
              inset 15px 15px 30px rgba(255, 255, 255, 0.1);
          "
        ></div>
      </div>

      <!-- Pyramide -->
      <div
        ref="pyramidRef"
        class="absolute top-40 right-1/3 w-0 h-0 opacity-12"
        style="transform-style: preserve-3d"
      >
        <div
          class="absolute border-l-[40px] border-r-[40px] border-b-[60px] border-l-transparent border-r-transparent border-b-[#c62d6a]/30"
          style="transform: rotateX(30deg) translateZ(0px)"
        ></div>
        <div
          class="absolute border-l-[40px] border-r-[40px] border-b-[60px] border-l-transparent border-r-transparent border-b-[#4c2e6c]/30"
          style="transform: rotateY(90deg) rotateX(30deg) translateZ(0px)"
        ></div>
        <div
          class="absolute border-l-[40px] border-r-[40px] border-b-[60px] border-l-transparent border-r-transparent border-b-[#c62d6a]/25"
          style="transform: rotateY(180deg) rotateX(30deg) translateZ(0px)"
        ></div>
        <div
          class="absolute border-l-[40px] border-r-[40px] border-b-[60px] border-l-transparent border-r-transparent border-b-[#4c2e6c]/25"
          style="transform: rotateY(-90deg) rotateX(30deg) translateZ(0px)"
        ></div>
        <div
          class="absolute w-[80px] h-[80px] border-2 border-[#c62d6a]/20"
          style="
            background: linear-gradient(135deg, rgba(198, 45, 106, 0.15), rgba(76, 46, 108, 0.15));
            transform: rotateX(-90deg) translateZ(30px);
          "
        ></div>
      </div>

      <!-- Tore (anneau) -->
      <div
        ref="torusRef"
        class="absolute bottom-1/3 left-1/3 w-24 h-24 opacity-10"
        style="transform-style: preserve-3d"
      >
        <div
          class="w-full h-full rounded-full border-4 border-[#c62d6a]/25"
          style="
            background: radial-gradient(
              circle,
              transparent 30%,
              rgba(198, 45, 106, 0.1) 30%,
              rgba(198, 45, 106, 0.1) 70%,
              transparent 70%
            );
          "
        ></div>
      </div>
    </div>

    <div
      class="absolute inset-0 z-[2] flex items-center justify-start pointer-events-none overflow-hidden pl-12"
    >
      <h1
        class="text-[12vw] lg:text-[15vw] leading-none font-black text-transparent opacity-3 whitespace-nowrap select-none"
        style="
          -webkit-text-stroke: 2px rgba(255, 255, 255, 0.2);
          transform: translateX(calc(var(--mouseX) * -30px));
        "
      >
        NELF VISION
      </h1>
    </div>

    <div class="relative z-10 h-full w-full p-6 md:p-12 flex flex-col justify-between">
      <!-- Header -->
      <div class="flex justify-between items-start">
        <div class="animate-slide-in-left">
          <div class="flex items-center gap-4">
            <div
              class="h-12 w-12 bg-white text-[#c62d6a] flex items-center justify-center font-bold text-xl tracking-tighter"
            >
              N.
            </div>
            <div class="hidden md:block">
              <span class="block font-bold tracking-[0.2em] text-sm">NELF AGENCY</span>
              <span class="block text-[10px] text-white/60 font-mono">EST. 2025</span>
            </div>
          </div>
        </div>

        <div class="max-w-xs text-right animate-slide-in-right">
          <div
            class="inline-flex items-center gap-2 border border-white/20 px-3 py-1 rounded-full bg-black/20 backdrop-blur-md mb-2"
          >
            <span class="relative flex h-2 w-2">
              <span
                class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"
              ></span>
              <span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
            </span>
            <span class="text-[10px] font-mono uppercase tracking-widest">Info Line v1.0</span>
          </div>
          <p class="text-sm font-light text-white/80 leading-snug">
            Actuellement en production sur le projet
            <span class="text-[#c62d6a] font-bold">Alpha-Web</span>. Lancement imminent.
          </p>
        </div>
      </div>

      <!-- Main Content - Left Aligned -->
      <div class="flex-1 flex items-center">
        <div class="max-w-2xl lg:max-w-3xl animate-slide-in-left">
          <div class="relative inline-block group">
            <div
              class="absolute -left-8 -top-8 w-32 h-32 border border-white/5 rounded-full animate-spin-slow pointer-events-none opacity-30"
            ></div>
            <div
              class="absolute -left-4 -top-4 w-24 h-24 border border-dashed border-white/10 rounded-full animate-reverse-spin pointer-events-none opacity-30"
            ></div>

            <h2
              class="text-5xl md:text-7xl lg:text-8xl font-black tracking-tighter mix-blend-screen hover:scale-105 transition-transform duration-500 leading-tight"
            >
              ARCHITECTURER <br />
              <span
                class="text-transparent bg-clip-text bg-gradient-to-r from-[#ff8fab] to-white filter drop-shadow-[0_0_15px_rgba(255,255,255,0.5)]"
              >
                L'IMPOSSIBLE
              </span>
            </h2>

            <p
              class="mt-6 text-lg md:text-xl lg:text-2xl font-light text-white/80 max-w-xl backdrop-blur-sm"
            >
              Nous créons ce qui n'existe pas encore. Votre vision, notre code.
            </p>

            <!-- CTA Buttons -->
            <div class="flex flex-col sm:flex-row gap-4 mt-8">
              <button
                @click="scrollToSection('realisation')"
                class="group relative flex items-center gap-4 pl-8 pr-2 py-3 bg-white text-black transition-all hover:pr-8 w-fit"
              >
                <span class="font-bold tracking-widest text-base uppercase">Voir les Projets</span>
                <div
                  class="h-12 w-12 bg-black text-white flex items-center justify-center transition-transform group-hover:rotate-45"
                >
                  <span class="material-symbols-outlined text-2xl">arrow_outward</span>
                </div>
              </button>

              <button
                @click="scrollToSection('apport')"
                class="group relative flex items-center gap-4 pl-8 pr-2 py-3 border-2 border-white/50 bg-white/10 backdrop-blur-md text-white transition-all hover:bg-white/20 hover:border-white/80 w-fit"
              >
                <span class="font-medium tracking-wide text-base">Notre Processus</span>
                <div
                  class="h-12 w-12 bg-white/10 flex items-center justify-center transition-transform group-hover:rotate-90"
                >
                  <span class="material-symbols-outlined text-2xl">settings</span>
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Bottom Section -->
      <!--<div class="flex flex-col md:flex-row justify-between items-start gap-8">
        <div class="max-w-md animate-slide-in-up">
          <span class="text-[#c62d6a] font-mono text-xs mb-2 block">// NOTRE APPORT</span>
          <h3 class="text-2xl font-bold leading-tight mb-2">Au-delà de la vitrine.</h3>
          <p class="text-sm text-white/60 leading-relaxed border-l border-white/30 pl-4">
            Nous ne livrons pas des sites, nous livrons de la croissance. Stratégie, Design, Tech :
            l'alliance parfaite pour dominer votre marché.
          </p>
        </div>
      </div>-->
    </div>

    <!-- CardSwap Component - Affiches animées (Desktop uniquement) -->
    <div class="absolute inset-0 z-[15] pointer-events-none overflow-hidden hidden lg:block">
      <CardSwap
        ref="cardSwapRef"
        :images="cardImages"
        :width="420"
        :height="580"
        :card-distance="45"
        :vertical-distance="55"
        :delay="4000"
        :pause-on-hover="true"
        :skew-amount="5"
        easing="elastic"
        class="pointer-events-auto"
        @card-click="handleCardClick"
      />
    </div>

    <!-- Carousel pour Mobile/Tablet -->
    <div
      class="absolute inset-0 z-[15] flex items-center justify-center lg:hidden"
      @mouseenter="stopCarouselAutoPlay"
      @mouseleave="startCarouselAutoPlay"
    >
      <div class="relative w-full max-w-sm px-4">
        <!-- Carousel Container -->
        <div
          class="relative overflow-hidden rounded-2xl"
          @touchstart="handleTouchStart"
          @touchend="handleTouchEnd"
        >
          <div
            class="flex transition-transform duration-500 ease-in-out"
            :style="{ transform: `translateX(-${currentCarouselIndex * 100}%)` }"
          >
            <div v-for="(image, index) in cardImages" :key="index" class="min-w-full flex-shrink-0">
              <img
                :src="image"
                :alt="`Affiche ${index + 1}`"
                class="w-full h-auto rounded-2xl shadow-2xl border-2 border-white/20 object-cover"
                style="max-height: 70vh"
              />
            </div>
          </div>

          <!-- Navigation Buttons -->
          <button
            @click="prevCarouselSlide"
            class="absolute left-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 backdrop-blur-md text-white p-3 rounded-full transition-all z-10"
            aria-label="Image précédente"
          >
            <span class="material-symbols-outlined">chevron_left</span>
          </button>
          <button
            @click="nextCarouselSlide"
            class="absolute right-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 backdrop-blur-md text-white p-3 rounded-full transition-all z-10"
            aria-label="Image suivante"
          >
            <span class="material-symbols-outlined">chevron_right</span>
          </button>

          <!-- Indicators -->
          <div class="absolute bottom-4 left-1/2 -translate-x-1/2 flex gap-2 z-10">
            <button
              v-for="(_, index) in cardImages"
              :key="index"
              @click="currentCarouselIndex = index"
              class="h-2 rounded-full transition-all"
              :class="
                currentCarouselIndex === index
                  ? 'w-8 bg-white'
                  : 'w-2 bg-white/50 hover:bg-white/75'
              "
              :aria-label="`Aller à l'image ${index + 1}`"
            ></button>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
/* Custom Keyframes for that "Sci-Fi" feel */
@keyframes spin-slow {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes reverse-spin {
  from {
    transform: rotate(360deg);
  }
  to {
    transform: rotate(0deg);
  }
}

@keyframes slide-in-left {
  from {
    opacity: 0;
    transform: translateX(-50px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slide-in-right {
  from {
    opacity: 0;
    transform: translateX(50px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slide-in-up {
  from {
    opacity: 0;
    transform: translateY(50px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-spin-slow {
  animation: spin-slow 20s linear infinite;
}

.animate-reverse-spin {
  animation: reverse-spin 15s linear infinite;
}

.animate-slide-in-left {
  animation: slide-in-left 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.animate-slide-in-right {
  animation: slide-in-right 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.animate-slide-in-up {
  animation: slide-in-up 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

/* Utility to access CSS vars in style */
main {
  --x: 50%;
  --y: 50%;
  perspective: 1000px;
}

/* Styles pour les éléments 3D */
[style*='transform-style: preserve-3d'] {
  transform-style: preserve-3d;
  backface-visibility: hidden;
  -webkit-backface-visibility: hidden;
}

/* Amélioration de la perspective 3D */
.absolute[ref] {
  will-change: transform;
}
</style>
