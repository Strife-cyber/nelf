<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import gsap from 'gsap'

interface Props {
  width?: number
  height?: number
  cardDistance?: number
  verticalDistance?: number
  delay?: number
  pauseOnHover?: boolean
  skewAmount?: number
  easing?: 'elastic' | 'smooth'
  images?: string[]
}

const props = withDefaults(defineProps<Props>(), {
  width: 500,
  height: 400,
  cardDistance: 60,
  verticalDistance: 70,
  delay: 5000,
  pauseOnHover: true,
  skewAmount: 6,
  easing: 'elastic',
  images: () => [],
})

const emit = defineEmits<{
  cardClick: [index: number]
}>()

const containerRef = ref<HTMLElement | null>(null)
const cardRefs = ref<(HTMLElement | null)[]>([])
const order = ref<number[]>([])
const timelineRef = ref<gsap.core.Timeline | null>(null)
const intervalRef = ref<number | null>(null)
const isPaused = ref(false)
const isAnimating = ref(false)

// Configuration des animations selon le type d'easing
const config = computed(() => {
  if (props.easing === 'elastic') {
    return {
      ease: 'elastic.out(0.6,0.9)',
      durDrop: 2,
      durMove: 2,
      durReturn: 2,
      promoteOverlap: 0.9,
      returnDelay: 0.05,
    }
  } else {
    return {
      ease: 'power1.inOut',
      durDrop: 0.8,
      durMove: 0.8,
      durReturn: 0.8,
      promoteOverlap: 0.45,
      returnDelay: 0.2,
    }
  }
})

// Créer un slot pour chaque carte
const makeSlot = (i: number, distX: number, distY: number, total: number) => ({
  x: i * distX,
  y: -i * distY,
  z: -i * distX * 1.5,
  zIndex: total - i,
})

// Placer une carte à une position donnée
const placeNow = (el: HTMLElement, slot: ReturnType<typeof makeSlot>, skew: number) => {
  gsap.set(el, {
    x: slot.x,
    y: slot.y,
    z: slot.z,
    xPercent: -50,
    yPercent: -50,
    skewY: skew,
    transformOrigin: 'center center',
    zIndex: slot.zIndex,
    force3D: true,
    opacity: 1,
  })
}

// Fonction principale d'animation de swap
const swap = () => {
  if (order.value.length < 2 || !cardRefs.value.length || isAnimating.value) return

  isAnimating.value = true
  const [front, ...rest] = order.value
  const elFront = cardRefs.value[front ?? 0]

  if (!elFront) {
    isAnimating.value = false
    return
  }

  const tl = gsap.timeline()
  timelineRef.value = tl

  // Faire tomber la carte du devant
  tl.to(elFront, {
    y: '+=500',
    duration: config.value.durDrop,
    ease: config.value.ease,
  })

  // Promouvoir les autres cartes
  tl.addLabel('promote', `-=${config.value.durDrop * config.value.promoteOverlap}`)

  rest.forEach((idx, i) => {
    const el = cardRefs.value[idx]
    if (!el) return

    const slot = makeSlot(i, props.cardDistance, props.verticalDistance, cardRefs.value.length)
    tl.set(el, { zIndex: slot.zIndex }, 'promote')
    tl.to(
      el,
      {
        x: slot.x,
        y: slot.y,
        z: slot.z,
        duration: config.value.durMove,
        ease: config.value.ease,
      },
      `promote+=${i * 0.15}`,
    )
  })

  // Remettre la carte du devant à l'arrière
  const backSlot = makeSlot(
    cardRefs.value.length - 1,
    props.cardDistance,
    props.verticalDistance,
    cardRefs.value.length,
  )

  tl.addLabel('return', `promote+=${config.value.durMove * config.value.returnDelay}`)
  tl.call(
    () => {
      if (elFront) {
        gsap.set(elFront, { zIndex: backSlot.zIndex })
      }
    },
    undefined,
    'return',
  )

  tl.to(
    elFront,
    {
      x: backSlot.x,
      y: backSlot.y,
      z: backSlot.z,
      duration: config.value.durReturn,
      ease: config.value.ease,
    },
    'return',
  )

  tl.call(() => {
    order.value = [...rest, front ?? 0]
    isAnimating.value = false
  })
}

// Fonction publique pour déclencher manuellement le swap
const triggerSwap = () => {
  if (!isAnimating.value) {
    swap()
  }
}

// Initialiser les cartes
const initialize = () => {
  if (!cardRefs.value.length || !props.images.length) return

  const total = cardRefs.value.length
  order.value = Array.from({ length: total }, (_, i) => i)

  cardRefs.value.forEach((ref, i) => {
    if (ref) {
      placeNow(
        ref,
        makeSlot(i, props.cardDistance, props.verticalDistance, total),
        props.skewAmount,
      )
    }
  })

  // Démarrer l'animation automatique après un court délai
  if (intervalRef.value) {
    clearInterval(intervalRef.value)
  }
  intervalRef.value = window.setInterval(() => {
    if (!isPaused.value && !isAnimating.value) {
      swap()
    }
  }, props.delay)
}

// Gestion du hover pour pause/resume
const handleMouseEnter = () => {
  if (props.pauseOnHover) {
    isPaused.value = true
    timelineRef.value?.pause()
    if (intervalRef.value) {
      clearInterval(intervalRef.value)
    }
  }
}

const handleMouseLeave = () => {
  if (props.pauseOnHover) {
    isPaused.value = false
    timelineRef.value?.play()
    if (intervalRef.value) {
      clearInterval(intervalRef.value)
    }
    intervalRef.value = window.setInterval(() => {
      if (!isPaused.value) {
        swap()
      }
    }, props.delay)
  }
}

// Gérer le clic sur une carte - déclencher le swap manuellement
const handleCardClick = (index: number) => {
  // Vérifier que c'est la carte du devant qui est cliquée
  if (order.value[0] === index && !isAnimating.value) {
    triggerSwap()
  }
  emit('cardClick', index)
}

// Exposer la fonction pour utilisation externe
defineExpose({
  triggerSwap,
})

onMounted(() => {
  // Attendre que les refs soient disponibles
  setTimeout(() => {
    initialize()
    // Démarrer la première animation après l'initialisation
    setTimeout(() => {
      if (!isAnimating.value) {
        swap()
      }
    }, 300)
  }, 100)
})

onUnmounted(() => {
  if (intervalRef.value) {
    clearInterval(intervalRef.value)
  }
  timelineRef.value?.kill()
})

// Réinitialiser si les props changent
watch(
  () => props.images,
  () => {
    if (props.images.length > 0) {
      setTimeout(() => {
        initialize()
        setTimeout(() => {
          if (!isAnimating.value) {
            swap()
          }
        }, 300)
      }, 100)
    }
  },
  { deep: true },
)

watch(
  () => [props.cardDistance, props.verticalDistance, props.delay, props.skewAmount],
  () => {
    if (cardRefs.value.length > 0) {
      initialize()
    }
  },
)
</script>

<template>
  <div
    ref="containerRef"
    class="card-swap-container"
    :style="{ width: `${props.width}px`, height: `${props.height}px` }"
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
  >
    <div
      v-for="(image, index) in props.images"
      :key="index"
      :ref="
        (el) => {
          if (el) cardRefs[index] = el as HTMLElement
        }
      "
      class="card"
      :style="{ width: `${props.width}px`, height: `${props.height}px` }"
      @click="handleCardClick(index)"
    >
      <img :src="image" :alt="`Affiche ${index + 1}`" class="w-full h-full object-cover" />
    </div>
  </div>
</template>

<style scoped>
.card-swap-container {
  position: absolute;
  bottom: 40%;
  right: 5%;
  transform: translate(0, 50%);
  transform-origin: center right;
  perspective: 900px;
  overflow: visible;
  filter: drop-shadow(0 0 30px rgba(198, 45, 106, 0.3));
}

.card {
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 16px;
  border: 2px solid rgba(255, 255, 255, 0.15);
  background: #000;
  overflow: hidden;
  cursor: pointer;
  box-shadow:
    0 25px 80px rgba(0, 0, 0, 0.6),
    0 0 50px rgba(198, 45, 106, 0.4),
    0 0 100px rgba(76, 46, 108, 0.2);
  transform-style: preserve-3d;
  will-change: transform;
  backface-visibility: hidden;
  -webkit-backface-visibility: hidden;
  transition:
    box-shadow 0.3s ease,
    border-color 0.3s ease;
  opacity: 1;
  visibility: visible;
}

.card:hover {
  box-shadow:
    0 30px 100px rgba(0, 0, 0, 0.7),
    0 0 70px rgba(198, 45, 106, 0.6),
    0 0 120px rgba(76, 46, 108, 0.3);
  border-color: rgba(255, 255, 255, 0.3);
}

.card img {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

@media (max-width: 1024px) {
  .card-swap-container {
    transform: scale(0.7) translate(20%, 20%);
  }
}

@media (max-width: 768px) {
  .card-swap-container {
    transform: scale(0.6) translate(15%, 15%);
  }
}

@media (max-width: 480px) {
  .card-swap-container {
    transform: scale(0.5) translate(10%, 10%);
  }
}
</style>
