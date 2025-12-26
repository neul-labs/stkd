import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi, type User } from '@/api/auth'

const TOKEN_KEY = 'stack_token'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem(TOKEN_KEY))
  const loading = ref(false)

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  async function restoreSession() {
    if (!token.value) return

    try {
      loading.value = true
      const me = await authApi.getMe(token.value)
      user.value = me
    } catch {
      // Token is invalid, clear it
      logout()
    } finally {
      loading.value = false
    }
  }

  function setToken(newToken: string) {
    token.value = newToken
    localStorage.setItem(TOKEN_KEY, newToken)
  }

  function setUser(newUser: User) {
    user.value = newUser
  }

  function logout() {
    user.value = null
    token.value = null
    localStorage.removeItem(TOKEN_KEY)
  }

  async function startOAuth(provider: 'github' | 'gitlab') {
    const { url } = await authApi.startOAuth(provider)
    window.location.href = url
  }

  return {
    user,
    token,
    loading,
    isAuthenticated,
    restoreSession,
    setToken,
    setUser,
    logout,
    startOAuth
  }
})
