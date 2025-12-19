<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import {
  GithubAuthProvider,
  GoogleAuthProvider,
  onAuthStateChanged,
  signInWithEmailAndPassword,
  createUserWithEmailAndPassword,
  signInWithPopup,
  signOut,
  type User,
} from 'firebase/auth'
import { getFirebaseAuth } from '@/services/firebase'
import logoImage from '@/assets/logo.png'

const router = useRouter()
const auth = getFirebaseAuth()

const isLoginMode = ref(true)
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const isLoading = ref(false)
const errorMsg = ref<string | null>(null)
const user = ref<User | null>(null)

const isAuthed = computed(() => !!user.value)

let unsubscribe: (() => void) | null = null

onMounted(() => {
  unsubscribe = onAuthStateChanged(auth, (u) => {
    user.value = u
    if (u) {
      router.push({ name: 'admin-dashboard' })
    }
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

function clearForm() {
  email.value = ''
  password.value = ''
  confirmPassword.value = ''
  errorMsg.value = null
}

async function authenticateWithGoogle() {
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

async function authenticateWithGithub() {
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

async function handleEmailPassword() {
  isLoading.value = true
  errorMsg.value = null

  if (!isLoginMode.value && password.value !== confirmPassword.value) {
    errorMsg.value = 'Les mots de passe ne correspondent pas.'
    isLoading.value = false
    return
  }

  if (!isLoginMode.value && password.value.length < 6) {
    errorMsg.value = 'Le mot de passe doit contenir au moins 6 caractères.'
    isLoading.value = false
    return
  }

  try {
    if (isLoginMode.value) {
      await signInWithEmailAndPassword(auth, email.value.trim(), password.value)
    } else {
      await createUserWithEmailAndPassword(auth, email.value.trim(), password.value)
    }
    clearForm()
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
    clearForm()
  } catch (e) {
    setError(e)
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div
    class="min-h-screen flex items-center justify-center px-6 py-16 relative"
    :style="{
      backgroundImage: `url(${logoImage})`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
    }"
  >
    <!-- Dark overlay for readability -->
    <div class="absolute inset-0 bg-[#2b2d75]/80 backdrop-blur-sm"></div>

    <!-- Content -->
    <div class="relative z-10 w-full max-w-md">
      <div
        class="w-full rounded-3xl border border-white/20 bg-white/10 backdrop-blur-xl p-8 shadow-2xl"
      >
        <!-- Header -->
        <div class="flex items-start justify-between gap-4 mb-8">
          <div>
            <h1 class="text-3xl font-bold tracking-tight text-white">
              {{ isLoginMode ? 'Connexion' : 'Inscription' }}
            </h1>
            <p class="mt-2 text-sm text-white/80">
              {{
                isLoginMode ? 'Accès réservé aux administrateurs' : 'Créer un compte administrateur'
              }}
            </p>
          </div>
          <button
            class="text-sm font-semibold text-white/70 hover:text-white transition-colors px-3 py-1 rounded-lg hover:bg-white/10"
            type="button"
            @click="router.push({ name: 'home' })"
          >
            Retour
          </button>
        </div>

        <!-- Authenticated State -->
        <div v-if="isAuthed" class="space-y-4">
          <div class="rounded-2xl bg-white/10 border border-white/20 p-6">
            <div class="flex items-center gap-3 mb-4">
              <div
                class="w-12 h-12 rounded-full bg-gradient-to-br from-[#c62d6a] to-[#4c2e6c] flex items-center justify-center text-white font-bold text-lg"
              >
                {{ user?.email?.charAt(0).toUpperCase() || 'A' }}
              </div>
              <div class="flex-1">
                <p class="text-sm text-white/60">Connecté en tant que</p>
                <p class="text-base font-semibold text-white">{{ user?.email ?? user?.uid }}</p>
              </div>
            </div>
            <button
              class="w-full inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-bold bg-white text-[#2b2d75] hover:bg-white/90 transition-all disabled:opacity-60 shadow-lg"
              type="button"
              :disabled="isLoading"
              @click="logout"
            >
              Se déconnecter
            </button>
          </div>
        </div>

        <!-- Auth Form -->
        <div v-else>
          <!-- Mode Toggle -->
          <div
            class="mb-6 flex items-center gap-2 p-1 rounded-xl bg-white/5 border border-white/10"
          >
            <button
              :class="[
                'flex-1 rounded-lg px-4 py-2.5 text-sm font-semibold transition-all',
                isLoginMode
                  ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
                  : 'text-white/60 hover:text-white/80',
              ]"
              type="button"
              @click="isLoginMode = true"
            >
              Connexion
            </button>
            <button
              :class="[
                'flex-1 rounded-lg px-4 py-2.5 text-sm font-semibold transition-all',
                !isLoginMode
                  ? 'bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white shadow-lg'
                  : 'text-white/60 hover:text-white/80',
              ]"
              type="button"
              @click="isLoginMode = false"
            >
              Inscription
            </button>
          </div>

          <form class="space-y-4" @submit.prevent="handleEmailPassword">
            <div class="space-y-2">
              <label class="block text-sm font-semibold text-white/90">Email</label>
              <input
                v-model="email"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="email"
                :autocomplete="isLoginMode ? 'email' : 'email'"
                placeholder="admin@nelf.com"
                required
              />
            </div>

            <div class="space-y-2">
              <label class="block text-sm font-semibold text-white/90">Mot de passe</label>
              <input
                v-model="password"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="password"
                :autocomplete="isLoginMode ? 'current-password' : 'new-password'"
                placeholder="••••••••"
                required
              />
            </div>

            <div v-if="!isLoginMode" class="space-y-2">
              <label class="block text-sm font-semibold text-white/90"
                >Confirmer le mot de passe</label
              >
              <input
                v-model="confirmPassword"
                class="w-full rounded-xl bg-white/10 border border-white/20 px-4 py-3 text-sm text-white placeholder-white/40 outline-none focus:ring-2 focus:ring-[#c62d6a]/50 focus:border-[#c62d6a]/50 transition-all"
                type="password"
                autocomplete="new-password"
                placeholder="••••••••"
                required
              />
            </div>

            <button
              class="w-full inline-flex items-center justify-center rounded-xl px-4 py-3.5 text-sm font-bold bg-gradient-to-r from-[#c62d6a] to-[#4c2e6c] text-white hover:from-[#d63d7a] hover:to-[#5c3e7c] transition-all disabled:opacity-60 shadow-lg shadow-[#c62d6a]/30 hover:shadow-[#c62d6a]/50"
              type="submit"
              :disabled="isLoading"
            >
              <span v-if="isLoading" class="mr-2">⏳</span>
              {{ isLoading ? 'Traitement...' : isLoginMode ? 'Se connecter' : 'Créer un compte' }}
            </button>
          </form>

          <!-- Social Auth -->
          <div class="mt-6">
            <div class="flex items-center gap-3 mb-4">
              <div class="h-px flex-1 bg-white/20"></div>
              <span class="text-xs text-white/60 font-medium">ou</span>
              <div class="h-px flex-1 bg-white/20"></div>
            </div>

            <div class="grid grid-cols-2 gap-3">
              <button
                class="inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-semibold bg-white/10 border border-white/20 hover:bg-white/15 text-white transition-all disabled:opacity-60 shadow-md hover:shadow-lg"
                type="button"
                :disabled="isLoading"
                @click="authenticateWithGoogle"
              >
                <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24">
                  <path
                    fill="#4285F4"
                    d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                  />
                  <path
                    fill="#34A853"
                    d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                  />
                  <path
                    fill="#FBBC05"
                    d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                  />
                  <path
                    fill="#EA4335"
                    d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                  />
                </svg>
                Google
              </button>
              <button
                class="inline-flex items-center justify-center rounded-xl px-4 py-3 text-sm font-semibold bg-white/10 border border-white/20 hover:bg-white/15 text-white transition-all disabled:opacity-60 shadow-md hover:shadow-lg"
                type="button"
                :disabled="isLoading"
                @click="authenticateWithGithub"
              >
                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
                  <path
                    d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                  />
                </svg>
                GitHub
              </button>
            </div>
          </div>

          <!-- Error Message -->
          <div v-if="errorMsg" class="mt-4 rounded-xl bg-red-500/20 border border-red-500/30 p-3">
            <p class="text-sm text-red-200 font-medium">{{ errorMsg }}</p>
          </div>

          <!-- Help Text -->
          <p class="mt-6 text-xs text-white/50 leading-relaxed">
            <span class="font-semibold text-white/70">Note:</span> Google et GitHub fonctionnent
            pour la connexion et l'inscription. Si les popups ne s'ouvrent pas, vérifiez que les
            providers sont activés dans Firebase Console.
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Additional custom styles if needed */
</style>
