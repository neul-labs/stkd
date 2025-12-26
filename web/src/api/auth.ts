import { get, post } from './client'

export interface User {
  id: string
  username: string
  email: string | null
  display_name: string | null
  avatar_url: string | null
  provider: string
}

export interface LoginResponse {
  token: string
  user: User
}

export interface OAuthStartResponse {
  url: string
  state: string
}

export const authApi = {
  async startOAuth(provider: 'github' | 'gitlab'): Promise<OAuthStartResponse> {
    return post(`/auth/oauth/${provider}/start`)
  },

  async getMe(token: string): Promise<User> {
    return get('/auth/me', token)
  },

  async logout(token: string): Promise<void> {
    await post('/auth/logout', undefined, token)
  }
}
