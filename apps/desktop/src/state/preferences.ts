export function preference(key: string, fallback: string) {
  try {
    return localStorage.getItem(`ametista.${key}`) ?? fallback
  } catch {
    return fallback
  }
}
export function persist(key: string, value: string) {
  try {
    localStorage.setItem(`ametista.${key}`, value)
  } catch {
    /* Preferences never authorize Windows operations. */
  }
}
