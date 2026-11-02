import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { SupportPanel, LegacyGuidance } from '../src/components/SupportPanel'
import { createMockBackend } from '../src/test-backend'
test('diagnostics require a preview and explicit consent before saving', async () => {
  const backend = createMockBackend()
  const support = vi.spyOn(backend, 'support')
  const user = userEvent.setup()
  render(<SupportPanel backend={backend} language="pt-BR" />)
  expect(screen.queryByRole('button', { name: 'Salve o ZIP' })).not.toBeInTheDocument()
  await user.click(screen.getByRole('button', { name: 'Revise o diagnóstico' }))
  const save = await screen.findByRole('button', { name: 'Salve o ZIP' })
  expect(save).toBeDisabled()
  await user.click(screen.getByRole('checkbox'))
  await user.click(save)
  expect(support).toHaveBeenLastCalledWith({ kind: 'save', token: 'mock-preview', confirm: true })
})
test('legacy guidance preserves old data and offers no automatic migration', () => {
  render(<LegacyGuidance language="pt-BR" />)
  expect(screen.getByText(/Não apague, mova ou renomeie/)).toBeInTheDocument()
  expect(screen.queryByRole('button')).not.toBeInTheDocument()
})
