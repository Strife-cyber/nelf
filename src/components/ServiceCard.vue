<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  icon: string
  title: string
  description: string
  hoverColor?: 'brand-deep' | 'brand-magenta'
}

const props = withDefaults(defineProps<Props>(), {
  hoverColor: 'brand-deep',
})

const isHovered = ref(false)

const cardBgColor = () => {
  if (!isHovered.value) return '#f8f8f5'
  return props.hoverColor === 'brand-magenta' ? '#c62d6a' : '#2b2d75'
}

const iconColor = () => {
  if (!isHovered.value) {
    return props.hoverColor === 'brand-magenta' ? '#c62d6a' : '#2b2d75'
  }
  return '#ffffff'
}

const textColor = () => {
  if (!isHovered.value) return '#2b2d75'
  return '#ffffff'
}

const descriptionColor = () => {
  if (!isHovered.value) return '#6b7280'
  return '#ffffff'
}
</script>

<template>
  <div
    class="group flex flex-col gap-6 p-8 rounded-xl transition-all duration-300 border border-transparent"
    :style="{
      backgroundColor: cardBgColor(),
      borderColor: isHovered
        ? props.hoverColor === 'brand-magenta'
          ? '#c62d6a'
          : '#2b2d75'
        : 'transparent',
    }"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div
      class="size-14 rounded-full bg-white flex items-center justify-center shadow-sm transition-colors"
      :style="{ color: iconColor() }"
    >
      <span class="material-symbols-outlined text-3xl">{{ icon }}</span>
    </div>
    <div class="flex flex-col gap-3">
      <h3 class="text-2xl font-bold transition-colors" :style="{ color: textColor() }">
        {{ title }}
      </h3>
      <p class="transition-colors leading-relaxed" :style="{ color: descriptionColor() }">
        {{ description }}
      </p>
    </div>
    <div class="mt-auto pt-4">
      <span
        class="font-bold text-sm flex items-center gap-2 transition-colors"
        :style="{ color: textColor() }"
      >
        En savoir plus <span class="material-symbols-outlined text-sm">arrow_forward</span>
      </span>
    </div>
  </div>
</template>