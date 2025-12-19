<script setup lang="ts">
interface Props {
  category: string
  title: string
  description: string
  imageUrl: string
  imageAlt: string
  gradientColor?: 'brand-deep' | 'brand-magenta'
}

withDefaults(defineProps<Props>(), {
  gradientColor: 'brand-deep',
})
</script>

<template>
  <div
    class="group relative rounded-xl overflow-hidden cursor-pointer aspect-[4/5] md:aspect-square lg:aspect-[3/4]"
  >
    <div class="absolute inset-0 bg-gray-200">
      <img
        :alt="imageAlt"
        class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105"
        :src="imageUrl"
      />
    </div>
    <div
      class="absolute inset-0 bg-gradient-to-t opacity-60 group-hover:opacity-90 transition-opacity duration-300"
      :style="{
        background: gradientColor === 'brand-magenta'
          ? 'linear-gradient(to top, rgba(198, 45, 106, 0.9), transparent)'
          : 'linear-gradient(to top, rgba(43, 45, 117, 0.9), transparent)',
      }"
    ></div>
    <div
      class="absolute bottom-0 left-0 w-full p-8 translate-y-4 group-hover:translate-y-0 transition-transform duration-300"
    >
      <span
        class="text-xs font-bold uppercase tracking-wider mb-2 block"
        style="color: #ffffff;"
        >{{ category }}</span
      >
      <h3 class="text-white text-2xl font-bold mb-2">{{ title }}</h3>
      <p
        class="text-white/80 text-sm opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-75"
      >
        {{ description }}
      </p>
    </div>
  </div>
</template>
