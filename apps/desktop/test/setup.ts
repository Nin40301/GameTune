import '@testing-library/jest-dom/vitest'
import { afterEach, beforeEach, vi } from 'vitest'
import { cleanup } from '@testing-library/react'
beforeEach(() => {
  localStorage.setItem('ametista.onboarding', 'done')
  vi.spyOn(navigator, 'language', 'get').mockReturnValue('pt-BR')
  vi.spyOn(document, 'hasFocus').mockReturnValue(true)
})
afterEach(() => {
  vi.restoreAllMocks()
  cleanup()
  localStorage.clear()
  delete window.__ametistaTest
})
