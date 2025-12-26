import { get, post, del } from './client'

export interface Repository {
  id: string
  provider: string
  owner: string
  name: string
  full_name: string
  default_branch: string
  is_active: boolean
  synced_at: string | null
}

export interface Branch {
  id: string
  name: string
  parent_name: string | null
  status: string
  head_sha: string | null
  mr: MergeRequest | null
}

export interface MergeRequest {
  id: string
  number: number
  title: string
  state: string
  url: string
}

export interface Stack {
  branches: Branch[]
}

export interface ConnectRepoRequest {
  provider: string
  owner: string
  name: string
  default_branch?: string
}

export const reposApi = {
  async list(token: string, orgSlug: string): Promise<Repository[]> {
    return get(`/orgs/${orgSlug}/repos`, token)
  },

  async connect(token: string, orgSlug: string, data: ConnectRepoRequest): Promise<Repository> {
    return post(`/orgs/${orgSlug}/repos`, data, token)
  },

  async disconnect(token: string, orgSlug: string, repoId: string): Promise<void> {
    await del(`/orgs/${orgSlug}/repos/${repoId}`, token)
  },

  async sync(token: string, orgSlug: string, repoId: string): Promise<void> {
    await post(`/orgs/${orgSlug}/repos/${repoId}/sync`, undefined, token)
  },

  async getStack(token: string, repoId: string): Promise<Stack> {
    return get(`/repos/${repoId}/stacks`, token)
  }
}
