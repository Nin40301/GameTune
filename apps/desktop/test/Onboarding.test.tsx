import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { App } from '../src/App'
import { createMockBackend } from '../src/test-backend'
import { initialLanguage } from '../src/hooks/usePreferences'
import { plural, date, number } from '../src/i18n-format'
test('first run explains scope, protection and profile without authorizing changes', async () => {
  localStorage.removeItem('ametista.onboarding')
  const backend = createMockBackend(),
    support = vi.spyOn(backend, 'support')
  const user = userEvent.setup()
  render(<App backend={backend} />)
  expect(await screen.findByRole('heading', { level: 1 })).toHaveTextContent('Cuide do Windows')
  await user.click(screen.getByRole('button', { name: 'Continue' }))
  expect(screen.getByRole('heading', { level: 1 })).toHaveFocus()
  expect(screen.getByText(/Não foi possível confirmar/)).toBeVisible()
  await user.click(screen.getByRole('button', { name: 'Confira a proteção' }))
  expect(support).toHaveBeenCalledWith({ kind: 'open', tool: 'protection' })
  await user.click(screen.getByRole('button', { name: 'Continue' }))
  await user.click(screen.getByRole('button', { name: 'Comece pela simulação' }))
  expect(await screen.findByRole('button', { name: 'Simule' })).toBeEnabled()
  expect(localStorage.getItem('ametista.onboarding')).toBe('done')
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('supported language selection and Intl handle numbers, plurals and dates', () => {
  vi.spyOn(navigator, 'language', 'get').mockReturnValue('en-US')
  expect(initialLanguage()).toBe('en')
  expect(plural('pt-BR', 'selected', 1)).toBe('1 selecionado')
  expect(plural('en', 'selected', 2)).toBe('2 selected')
  expect(number('pt-BR', 1234.5, 1)).toContain('1.234,5')
  expect(date('en', 'not a date')).toBe('—')
})
