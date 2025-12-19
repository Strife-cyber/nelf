<script setup lang="ts">
import AppNavigation from '../components/AppNavigation.vue'
import HeroSection from '../components/HeroSection.vue'
import ServicesSection from '../components/ServicesSection.vue'
import ValueProposition from '../components/ValueProposition.vue'
import VisionSection from '../components/VisionSection.vue'
import ProjectsSection from '../components/ProjectsSection.vue'
import Footer from '../components/Footer.vue'
</script>

<template>
  <div class="relative flex min-h-screen w-full flex-col">
    <AppNavigation />
    <HeroSection />
    <ServicesSection />
    <ValueProposition />
    <VisionSection />
    <ProjectsSection />
    <Footer />
  </div>
</template>
