<script setup lang="ts">
const scrollToSection = (id: string) => {
  const element = document.getElementById(id)
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' })
  }
}
</script>

<template>
  <footer
    class="text-white py-16 px-4 md:px-12 border-t border-white/10"
    style="background-color: #2b2d75;"
    id="contact"
  >
    <div class="max-w-[1200px] mx-auto">
      <div class="flex flex-col md:flex-row justify-between gap-12 mb-16">
        <div class="flex flex-col gap-6 max-w-sm">
          <div class="flex items-center gap-2 mb-2">
            <img src="/src/assets/logo.png" alt="NELF Logo" class="h-8 w-auto" />
            <span class="text-white text-2xl font-bold tracking-tight">NELF</span>
          </div>
          <p class="text-gray-400 text-sm leading-relaxed">
            Nous sommes une agence créative dédiée à la création de produits numériques qui
            comptent. L'innovation, la stratégie et le design sont au cœur de notre activité.
          </p>
          <div class="flex gap-4">
            <a
              class="size-10 rounded-full bg-white/10 flex items-center justify-center transition-all hover:bg-[#c62d6a] hover:text-white"
              href="#"
              aria-label="LinkedIn"
            >
              <span class="text-xs font-bold">In</span>
            </a>
            <a
              class="size-10 rounded-full bg-white/10 flex items-center justify-center transition-all hover:bg-[#c62d6a] hover:text-white"
              href="#"
              aria-label="Twitter"
            >
              <span class="text-xs font-bold">Tw</span>
            </a>
            <a
              class="size-10 rounded-full bg-white/10 flex items-center justify-center transition-all hover:bg-[#c62d6a] hover:text-white"
              href="#"
              aria-label="Instagram"
            >
              <span class="text-xs font-bold">Ig</span>
            </a>
          </div>
        </div>
        <div class="grid grid-cols-2 md:grid-cols-3 gap-8 md:gap-16">
          <div class="flex flex-col gap-4">
            <h4 class="font-bold text-lg">Sitemap</h4>
            <button
              class="text-gray-400 hover:text-[#c62d6a] transition-colors text-sm text-left"
              @click="scrollToSection('services')"
            >
              Ce qu'on fait
            </button>
            <button
              class="text-gray-400 hover:text-[#c62d6a] transition-colors text-sm text-left"
              @click="scrollToSection('apport')"
            >
              Apport
            </button>
            <button
              class="text-gray-400 hover:text-[#c62d6a] transition-colors text-sm text-left"
              @click="scrollToSection('vision')"
            >
              Vision
            </button>
            <button
              class="text-gray-400 hover:text-[#c62d6a] transition-colors text-sm text-left"
              @click="scrollToSection('realisation')"
            >
              Realisation
            </button>
          </div>
          <div class="flex flex-col gap-4">
            <h4 class="font-bold text-lg">Contact</h4>
            <a
              class="text-gray-400 hover:text-[#c62d6a] transition-colors text-sm"
              href="mailto:hello@nelf.com"
            >
              hello@nelf.com
            </a>
            <span class="text-gray-400 text-sm">+33 1 23 45 67 89</span>
            <span class="text-gray-400 text-sm">123 Design Avenue<br />Paris, France</span>
          </div>
        </div>
      </div>
      <div
        class="pt-8 border-t border-white/10 flex flex-col md:flex-row justify-between items-center gap-4"
      >
        <p class="text-gray-500 text-xs">© 2024 NELF Agency. Tous droits réservés.</p>
        <div class="flex gap-6">
          <a class="text-gray-500 hover:text-white text-xs" href="#">Politique de Confidentialité</a>
          <a class="text-gray-500 hover:text-white text-xs" href="#">Conditions d'Utilisation</a>
        </div>
      </div>
    </div>
  </footer>
</template>
