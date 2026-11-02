import '@testing-library/jest-dom/vitest'
import { afterEach, beforeEach, vi } from 'vitest'
import { cleanup } from '@testing-library/react'
beforeEach(() => {
  vi.spyOn(document, 'hasFocus').mockReturnValue(true)
})
afterEach(() => {
  vi.restoreAllMocks()
  cleanup()
  localStorage.clear()
  delete window.__ametistaTest
})
