import { render, screen, within } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { App } from '../src/App'
import { createMockBackend } from '../src/test-backend'
import pt from '../src/locales/pt-BR.json'
import en from '../src/locales/en.json'
async function analyzed() {
  const user = userEvent.setup()
  render(<App backend={createMockBackend()} />)
  await user.click(await screen.findByRole('button', { name: /^Simule/ }))
  await screen.findByText('Análise concluída. Nenhuma alteração foi feita', {}, { timeout: 5000 })
  return user
}
test('starts read-only with Safe and cannot apply before analysis and review', async () => {
  const user = userEvent.setup()
  render(<App backend={createMockBackend()} />)
  await screen.findByRole('radio', { name: /Seguro/ })
  expect(screen.getByRole('radio', { name: /Seguro/ })).toBeChecked()
  expect(document.documentElement.dataset.theme).toBe('light')
  expect(document.documentElement.dataset.effects).toBe('reduced')
  expect(screen.queryByRole('button', { name: 'Aplique os ajustes' })).not.toBeInTheDocument()
  expect(screen.getByRole('button', { name: 'Revise os ajustes' })).toBeDisabled()
  await user.click(screen.getByRole('button', { name: 'Itens' }))
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('analysis is cancellable and cancellation never executes changes', async () => {
  const user = userEvent.setup()
  render(<App backend={createMockBackend()} />)
  await user.click(await screen.findByRole('button', { name: /^Simule/ }))
  await user.click(await screen.findByRole('button', { name: 'Cancele' }))
  await screen.findByText(pt.cancelled)
  expect(window.__ametistaTest?.cancelled).toBe(1)
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('review requires final acknowledgement and submits only reviewed ids', async () => {
  const user = await analyzed()
  await user.click(screen.getByRole('button', { name: 'Revise os ajustes' }))
  const apply = await screen.findByRole('button', { name: 'Aplique os ajustes' })
  expect(apply).toBeDisabled()
  await user.click(screen.getByLabelText(pt.ackAll))
  await user.click(apply)
  await screen.findByText(pt.operationDone)
  expect(window.__ametistaTest?.executions).toHaveLength(1)
  expect(window.__ametistaTest?.reviews[0]?.kind).toBe('apply')
})
test('app cleanup requires a separate consent per selected app', async () => {
  const user = await analyzed()
  await user.click(screen.getByRole('button', { name: 'Limpeza de apps' }))
  await user.click(screen.getByRole('checkbox', { name: 'Remover Microsoft.Copilot' }))
  await user.click(screen.getByRole('checkbox', { name: 'Remover Microsoft.549981C3F5F10' }))
  await user.click(screen.getByRole('button', { name: /Revise a remoção/ }))
  expect(screen.getByText(pt.noUndo)).toBeVisible()
  const remove = screen.getByRole('button', { name: pt.remove })
  await user.click(screen.getByLabelText(pt.ackAll))
  expect(remove).toBeDisabled()
  const consents = screen.getAllByLabelText(pt.ackNoUndo)
  await user.click(consents[0]!)
  expect(remove).toBeDisabled()
  await user.click(consents[1]!)
  expect(remove).toBeEnabled()
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('settings switch language and theme and translations have identical keys', async () => {
  expect(Object.keys(pt).sort()).toEqual(Object.keys(en).sort())
  const user = userEvent.setup()
  render(<App backend={createMockBackend()} />)
  await user.click(screen.getByRole('button', { name: 'Configurações' }))
  await user.selectOptions(screen.getByLabelText('Idioma'), 'en')
  expect(await screen.findByRole('heading', { name: 'Make it yours' })).toBeVisible()
  await user.click(screen.getByRole('radio', { name: 'Light' }))
  expect(document.documentElement.dataset.theme).toBe('light')
  expect(document.documentElement.lang).toBe('en')
  expect(
    within(screen.getByRole('navigation')).getByRole('button', { name: 'History' }),
  ).toBeEnabled()
})

test('a helper error with null result is shown without treating it as success', async () => {
  const user = userEvent.setup(),
    backend = createMockBackend()
  backend.execute = async () => ({ ok: false, result: null, error: 'confirmation_required' })
  render(<App backend={backend} />)
  await user.click(screen.getByRole('button', { name: 'Histórico' }))
  await user.click(await screen.findByRole('button', { name: 'Desfaça tudo' }))
  await user.click(screen.getByLabelText(pt.ackAll))
  await user.click(screen.getByRole('button', { name: /^Desfaça$/ }))
  expect(await screen.findByRole('alert')).toHaveTextContent(pt.confirmation_required)
  expect(screen.queryByText(pt.operationDone)).not.toBeInTheDocument()
})
