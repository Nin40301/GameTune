import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { App } from '../src/App'
import { createMockBackend } from '../src/test-backend'
import { operationRows } from '../src/state/result'
import pt from '../src/locales/pt-BR.json'
test('confirmed reboot flag only opens the typed Windows choice', async () => {
  const backend = createMockBackend(),
    execute = backend.execute,
    support = vi.spyOn(backend, 'support')
  backend.execute = async (approval) => {
    const reply = await execute(approval)
    return {
      ...reply,
      result: {
        ...reply.result,
        operation: {
          report: {
            items: [{ itemId: 'reg.advertising', outcome: 'applied', requiresReboot: true }],
          },
        },
      },
    }
  }
  const user = userEvent.setup()
  render(<App backend={backend} />)
  await user.click(await screen.findByRole('button', { name: pt.simulate }))
  await screen.findByText(pt.complete, {}, { timeout: 5000 })
  await user.click(screen.getByRole('button', { name: pt.reviewSelection }))
  await user.click(screen.getByLabelText(pt.ackAll))
  await user.click(screen.getByRole('button', { name: pt.apply }))
  await user.click(await screen.findByRole('button', { name: pt.restartNow }))
  expect(support).toHaveBeenCalledWith({ kind: 'open', tool: 'restart_dialog' })
})
test('result projection accepts structured reports, never parses error strings as a report', () => {
  expect(operationRows({ ok: false, result: null, error: 'requiresReboot:true' })).toEqual([])
  expect(
    operationRows({ ok: true, result: { operation: '{"requiresReboot":true}' }, error: null }),
  ).toEqual([])
})
