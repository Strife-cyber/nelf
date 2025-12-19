<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import {
  GithubAuthProvider,
  GoogleAuthProvider,
  onAuthStateChanged,
  signInWithEmailAndPassword,
  signInWithPopup,
  signOut,
  type User,
} from 'firebase/auth'
import { getFirebaseAuth } from '@/services/firebase'

const router = useRouter()
const auth = getFirebaseAuth()

const email = ref('')
const password = ref('')
const isLoading = ref(false)
const errorMsg = ref<string | null>(null)
const user = ref<User | null>(null)

const isAuthed = computed(() => !!user.value)

let unsubscribe: (() => void) | null = null

onMounted(() => {
  unsubscribe = onAuthStateChanged(auth, (u) => {
    user.value = u
  })
})

onUnmounted(() => {
  unsubscribe?.()
  unsubscribe = null
})

function setError(e: unknown) {
  const message = e instanceof Error ? e.message : 'Une erreur est survenue.'
  errorMsg.value = message
}

async function loginWithGoogle() {
  isLoading.value = true
  errorMsg.value = null
  try {
    const provider = new GoogleAuthProvider()
    await signInWithPopup(auth, provider)
  } catch (e) {
    setError(e)
  } finally {
    isLoading.value = false
  }
}

async function loginWithGithub() {
  isLoading.value = true
  errorMsg.value = null
  try {
    const provider = new GithubAuthProvider()
    await signInWithPopup(auth, provider)
  } catch (e) {
    setError(e)
  } finally {
    isLoading.value = false
  }
}

async function loginWithEmailPassword() {
  isLoading.value = true
  errorMsg.value = null
  try {
    await signInWithEmailAndPassword(auth, email.value.trim(), password.value)
  } catch (e) {
    setError(e)
  } finally {
    isLoading.value = false
  }
}

async function logout() {
  isLoading.value = true
  errorMsg.value = null
  try {
    await signOut(auth)
  } catch (e) {
    setError(e)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center px-6 py-16">
    <div class="w-full max-w-lg rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-8 shadow-xl">
      <div class="flex items-start justify-between gap-4">
        <div>
          <h1 class="text-2xl font-bold tracking-tight">Admin · Connexion</h1>
          <p class="mt-1 text-sm text-white/70">
            Accès réservé. Connectez-vous pour gérer le contenu.
          </p>
        </div>
        <button
          class="text-sm font-semibold text-white/70 hover:text-white transition-colors"
          type="button"
          @click="router.push({ name: 'home' })"
        >
          Retour
        </button>
      </div>

      <div v-if="isAuthed" class="mt-6 rounded-xl bg-black/20 border border-white/10 p-4">
        <p class="text-sm text-white/80">
          Connecté en tant que <span class="font-semibold">{{ user?.email ?? user?.uid }}</span>
        </p>
        <div class="mt-4 flex items-center gap-3">
          <button
            class="inline-flex items-center justify-center rounded-xl px-4 py-2 text-sm font-bold bg-white text-[#1a1b41] hover:bg-white/90 transition-colors disabled:opacity-60"
            type="button"
            :disabled="isLoading"
            @click="logout"
          >
            Se déconnecter
          </button>
        </div>
      </div>

      <form v-else class="mt-6 space-y-4" @submit.prevent="loginWithEmailPassword">
        <div class="space-y-2">
          <label class="block text-sm font-semibold text-white/80">Email</label>
          <input
            v-model="email"
            class="w-full rounded-xl bg-black/20 border border-white/10 px-4 py-3 text-sm outline-none focus:ring-2 focus:ring-[#c62d6a]/60"
            type="email"
            autocomplete="email"
            placeholder="admin@nelf.com"
            required
          />
        </div>
        <div class="space-y-2">
          <label class="block text-sm font-semibold text-white/80">Mot de passe</label>
          <input
            v-model="password"
            class="w-full rounded-xl bg-black/20 border border-white/10 px-4 py-3 text-sm outline-none focus:ring-2 focus:ring-[#c62d6a]/60"
            type="password"
            autocomplete="current-password"
            placeholder="••••••••"
            required
          />
        </div>

        <button
          class="w-full inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-bold bg-linear-to-r from-[#c62d6a] to-[#2b2d75] hover:brightness-110 transition disabled:opacity-60"
          type="submit"
          :disabled="isLoading"
        >
          Se connecter (Email / Mot de passe)
        </button>
      </form>

      <div v-if="!isAuthed" class="mt-6">
        <div class="flex items-center gap-3">
          <div class="h-px flex-1 bg-white/10"></div>
          <span class="text-xs text-white/50">ou</span>
          <div class="h-px flex-1 bg-white/10"></div>
        </div>

        <div class="mt-4 grid grid-cols-1 sm:grid-cols-2 gap-3">
          <button
            class="inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-bold bg-white/10 border border-white/10 hover:bg-white/15 transition disabled:opacity-60"
            type="button"
            :disabled="isLoading"
            @click="loginWithGoogle"
          >
            Continuer avec Google
          </button>
          <button
            class="inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-bold bg-white/10 border border-white/10 hover:bg-white/15 transition disabled:opacity-60"
            type="button"
            :disabled="isLoading"
            @click="loginWithGithub"
          >
            Continuer avec GitHub
          </button>
        </div>
      </div>

      <p v-if="errorMsg" class="mt-4 text-sm text-red-300">
        {{ errorMsg }}
      </p>

      <p class="mt-6 text-xs text-white/50">
        Si Google/GitHub ne s’ouvre pas: vérifiez que le provider est activé dans Firebase Console et que votre domaine
        est autorisé.
      </p>
    </div>
  </div>
</template>


