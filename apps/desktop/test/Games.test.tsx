import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { GamesPanel } from '../src/pages/GamesPanel'
import { createMockBackend } from '../src/test-backend'
import pt from '../src/locales/pt-BR.json'
import { App } from '../src/App'
test('game registration starts disabled and needs profile consent', async () => {
  const user = userEvent.setup()
  render(<GamesPanel backend={createMockBackend()} language="pt-BR" />)
  await user.click(await screen.findByRole('button', { name: pt.addGame }))
  await user.click(await screen.findByRole('button', { name: /Sample.exe/ }))
  await user.click(screen.getByRole('switch', { name: pt.enableProfile }))
  expect(screen.getByRole('button', { name: pt.saveGame })).toBeDisabled()
  await user.click(screen.getByRole('switch', { name: pt['gameAction_reg.game_mode'] }))
  await user.click(screen.getByRole('checkbox', { name: pt.gameConsent }))
  await user.click(screen.getByRole('button', { name: pt.saveGame }))
  expect(await screen.findByText(pt.profileEnabled)).toBeVisible()
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
test('monitor starts on demand with acknowledgement and stops separately', async () => {
  const user = userEvent.setup()
  render(<GamesPanel backend={createMockBackend()} language="pt-BR" settings />)
  expect(await screen.findByRole('button', { name: pt.startMonitor })).toBeDisabled()
  await user.click(screen.getByRole('checkbox', { name: pt.monitorConsent }))
  await user.click(screen.getByRole('button', { name: pt.startMonitor }))
  expect(await screen.findByText(pt.monitorRunning)).toBeVisible()
  await user.click(screen.getByRole('button', { name: pt.stopMonitor }))
  expect(await screen.findByText(pt.monitorStopped)).toBeVisible()
})
test('configuration discovery does not enable profiles or guess executables', async () => {
  const user = userEvent.setup()
  render(<GamesPanel backend={createMockBackend()} language="en" />)
  await user.click(await screen.findByRole('button', { name: 'Discover games' }))
  expect(await screen.findByText('Sample Steam Game')).toBeVisible()
  expect(screen.getByRole('button', { name: 'Select the .exe manually' })).toBeDisabled()
})
test('startup offers recovery without starting monitor or requesting elevation automatically', async () => {
  const backend = createMockBackend()
  const original = backend.games
  backend.games = async (action) => {
    const view = await original(action)
    view.monitor.recoveryPending = true
    return view
  }
  const user = userEvent.setup()
  render(<App backend={backend} />)
  await user.click(await screen.findByRole('button', { name: pt.reviewGameRecovery }))
  expect(await screen.findByRole('button', { name: pt.startMonitor })).toBeDisabled()
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})

test('icons are requested without paths or PIDs and missing icons keep a usable game choice', async () => {
  const backend = createMockBackend(),
    games = vi.spyOn(backend, 'games'),
    user = userEvent.setup()
  render(<GamesPanel backend={backend} language="pt-BR" />)
  await user.click(await screen.findByRole('button', { name: pt.addGame }))
  await screen.findByRole('button', { name: /Sample.exe/ })
  expect(games).toHaveBeenCalledWith({ kind: 'icons' })
  expect(
    games.mock.calls
      .filter(([action]) => action.kind === 'icons')
      .every(([action]) => Object.keys(action).length === 1),
  ).toBe(true)
  expect(screen.getByRole('button', { name: /Sample.exe/ })).toBeEnabled()
})
