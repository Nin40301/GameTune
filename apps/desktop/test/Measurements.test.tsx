import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Measurements } from '../src/Measurements'
import { createMockBackend } from '../src/test-backend'
import pt from '../src/locales/pt-BR.json'

test('dashboard renders measured fields and can pause without a system operation', async () => {
  const user = userEvent.setup()
  render(<Measurements backend={createMockBackend()} language="pt-BR" dashboard />)
  await screen.findByText('6 GiB')
  await user.click(screen.getByRole('button', { name: pt.pauseMetrics }))
  expect(await screen.findByText(pt.metricsPaused)).toBeVisible()
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('benchmark requires all conditions and can discard a partial run', async () => {
  const user = userEvent.setup()
  render(<Measurements backend={createMockBackend()} language="pt-BR" />)
  const start = await screen.findByRole('button', { name: pt.startRun })
  expect(start).toBeDisabled()
  for (const label of [pt.condition_appsClosed, pt.condition_sameWorkload, pt.condition_cooled])
    await user.click(screen.getByLabelText(label))
  expect(start).toBeDisabled()
  await user.click(screen.getByLabelText(pt.condition_samePower))
  await user.click(start)
  await user.click(await screen.findByRole('button', { name: pt.cancelRun }))
  expect(await screen.findByText(pt.benchmarkInterrupted)).toBeVisible()
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('FPS stays absent without PresentMon and exports default to anonymized', async () => {
  render(<Measurements backend={createMockBackend()} language="pt-BR" />)
  expect(await screen.findByRole('button', { name: pt.importPresentmon })).toBeDisabled()
  expect(screen.queryByText(pt.averageFps)).not.toBeInTheDocument()
  expect(screen.getByRole('checkbox', { name: pt.anonymizeReport })).toBeChecked()
})
test('production sampling errors never become fictitious values', async () => {
  const backend = createMockBackend()
  backend.lab = async () => {
    throw 'metrics_unavailable'
  }
  render(<Measurements backend={backend} language="en" dashboard />)
  expect(await screen.findByRole('alert')).toHaveTextContent('No values were estimated')
  expect(screen.queryByText('6 GiB')).not.toBeInTheDocument()
})
