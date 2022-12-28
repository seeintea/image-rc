const AUTH_TOKEN = 'admin-token'

export const getAuthToken: () => string  = () => {
  return localStorage.getItem(AUTH_TOKEN) || ''
}

export const setAuthToken: (s: string) => void = (token) => {
  localStorage.setItem(AUTH_TOKEN, token)
}

export const removeAuthToken: () => void = () => {
  localStorage.removeItem(AUTH_TOKEN)
}

