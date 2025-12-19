<script setup lang="ts">
import TheHeader from '@/components/TheHeader.vue'
import TheHero from '@/components/TheHero.vue'
import ServicesSection from '@/components/ServicesSection.vue'
import ValuesSection from '@/components/ValuesSection.vue'
import VisionSection from '@/components/VisionSection.vue'
import CreationsSection from '@/components/CreationsSection.vue'
import TeamSection from '@/components/TeamSection.vue'
import TheFooter from '@/components/TheFooter.vue'
</script>

<template>
  <div class="layout-container flex h-full grow flex-col">
    <TheHeader />
    <TheHero />
    <ServicesSection />
    <ValuesSection />
    <VisionSection />
    <CreationsSection />
    <TeamSection />
    <TheFooter />
  </div>
</template>

<style scoped>
.layout-container {
  min-height: 100vh;
}
</style>
