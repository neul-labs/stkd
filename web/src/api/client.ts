const API_BASE = '/api'

export class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

export async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {},
  token?: string
): Promise<T> {
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers
  }

  if (token) {
    (headers as Record<string, string>)['Authorization'] = `Bearer ${token}`
  }

  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers
  })

  if (!response.ok) {
    const error = await response.json().catch(() => ({
      error: 'unknown',
      message: 'An error occurred'
    }))
    throw new ApiError(response.status, error.error, error.message)
  }

  return response.json()
}

export function get<T>(endpoint: string, token?: string): Promise<T> {
  return apiRequest<T>(endpoint, { method: 'GET' }, token)
}

export function post<T>(endpoint: string, data?: unknown, token?: string): Promise<T> {
  return apiRequest<T>(
    endpoint,
    {
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined
    },
    token
  )
}

export function patch<T>(endpoint: string, data?: unknown, token?: string): Promise<T> {
  return apiRequest<T>(
    endpoint,
    {
      method: 'PATCH',
      body: data ? JSON.stringify(data) : undefined
    },
    token
  )
}

export function del<T>(endpoint: string, token?: string): Promise<T> {
  return apiRequest<T>(endpoint, { method: 'DELETE' }, token)
}
