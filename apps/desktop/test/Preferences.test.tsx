import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { usePreferences } from '../src/hooks/usePreferences'
import { pseudo } from '../src/i18n'
function Preferences() {
  const p = usePreferences()
  return (
    <>
      <button onClick={() => p.setTheme('system')}>system</button>
      <button onClick={() => p.setEffects('auto')}>auto</button>
      <button onClick={() => p.setTextSize(130)}>large</button>
      <button onClick={() => p.setMotion('reduced')}>motion</button>
    </>
  )
}
test('visual preferences resolve system values without granting any Windows consent', async () => {
  const user = userEvent.setup()
  render(<Preferences />)
  await user.click(screen.getByText('system'))
  await user.click(screen.getByText('auto'))
  await user.click(screen.getByText('large'))
  await user.click(screen.getByText('motion'))
  expect(localStorage.getItem('ametista.theme')).toBe('system')
  expect(document.documentElement.dataset.theme).toBe('light')
  expect(document.documentElement.dataset.effects).toBe('full')
  expect(document.documentElement.dataset.textSize).toBe('130')
  expect(document.documentElement.dataset.motion).toBe('reduced')
  expect(localStorage.getItem('ametista.consent')).toBeNull()
})
test('pseudo localization expands by at least forty percent and preserves format tokens', () => {
  const text = 'Review {count} adjustments'
  expect(pseudo(text).length).toBeGreaterThanOrEqual(text.length * 1.4)
  expect(pseudo(text)).toContain('{count}')
})
