import { get, post, patch, del } from './client'

export interface Organization {
  id: string
  name: string
  slug: string
  description: string | null
  avatar_url: string | null
  role: string
}

export interface Member {
  id: string
  username: string
  display_name: string | null
  avatar_url: string | null
  role: string
}

export interface CreateOrgRequest {
  name: string
  slug: string
  description?: string
}

export interface UpdateOrgRequest {
  name?: string
  description?: string
}

export const orgsApi = {
  async list(token: string): Promise<Organization[]> {
    return get('/orgs', token)
  },

  async get(token: string, slug: string): Promise<Organization> {
    return get(`/orgs/${slug}`, token)
  },

  async create(token: string, data: CreateOrgRequest): Promise<Organization> {
    return post('/orgs', data, token)
  },

  async update(token: string, slug: string, data: UpdateOrgRequest): Promise<Organization> {
    return patch(`/orgs/${slug}`, data, token)
  },

  async delete(token: string, slug: string): Promise<void> {
    await del(`/orgs/${slug}`, token)
  },

  async listMembers(token: string, slug: string): Promise<Member[]> {
    return get(`/orgs/${slug}/members`, token)
  },

  async removeMember(token: string, slug: string, memberId: string): Promise<void> {
    await del(`/orgs/${slug}/members/${memberId}`, token)
  }
}
