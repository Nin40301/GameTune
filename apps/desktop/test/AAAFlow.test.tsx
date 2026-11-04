import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { useState } from 'react'
import { ProfileCards } from '../src/components/ProfileCards'
import { currentLabel, nextLabel } from '../src/state/presentation'
import { translator } from '../src/i18n'
import { App } from '../src/App'
import { createMockBackend } from '../src/test-backend'
import pt from '../src/locales/pt-BR.json'
import type { Observation } from '../src/types'

test('profile cards expose count and risk while preserving one keyboard tab stop', async () => {
  function Example() {
    const [value, setValue] = useState('safe')
    return (
      <ProfileCards
        value={value}
        onChange={setValue}
        counts={{ safe: ['one', 'two'] }}
        language="pt-BR"
        disabled={false}
      />
    )
  }
  const user = userEvent.setup()
  render(<Example />)
  expect(screen.getByText('2 ajustes')).toBeVisible()
  await user.tab()
  await user.keyboard('{ArrowRight}')
  expect(screen.getByRole('radio', { name: pt.balanced })).toBeChecked()
  await user.keyboard('{End}')
  expect(screen.getByRole('radio', { name: pt.custom })).toHaveFocus()
  expect(screen.getAllByRole('radio').filter((element) => element.tabIndex === 0)).toHaveLength(1)
})
test('review presentation keeps unknown, absent and zero distinct; it never guesses a snapshot', () => {
  const t = translator('pt-BR'),
    row: Observation = {
      itemId: 'reg.example',
      presence: 'exists',
      exists: true,
      alreadyApplied: false,
      current: null,
      currentDword: 0,
      error: null,
      scope: 'user',
      removable: null,
    }
  expect(currentLabel(undefined, t)).toBe(pt.valueUnknown)
  expect(currentLabel(row, t)).toBe('0')
  expect(
    currentLabel({ ...row, currentDword: null, current: { type: 'registry', value: null } }, t),
  ).toBe(pt.valueAbsent)
  expect(currentLabel({ ...row, error: 'denied' }, t)).toBe(pt.valueUnknown)
  expect(nextLabel({ type: 'registry', target: { value: 1 } }, 'apply', t)).toBe('1')
})
test('group selection remains reversible verified and free of high-risk automatic choices', async () => {
  const user = userEvent.setup(),
    backend = createMockBackend()
  render(<App backend={backend} />)
  await user.click(await screen.findByRole('button', { name: pt.items }))
  await user.click(screen.getAllByRole('button', { name: pt.selectGroup })[0]!)
  await user.click(screen.getByRole('button', { name: pt.filterSelected }))
  expect(
    screen
      .getAllByRole('switch')
      .every((element) => element.getAttribute('aria-checked') === 'true'),
  ).toBe(true)
  expect(window.__ametistaTest?.executions).toHaveLength(0)
})
