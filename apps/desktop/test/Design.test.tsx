import { useState } from 'react'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Segmented } from '../src/components/Segmented'
import { Switch } from '../src/components/Switch'
import { StatusOrb } from '../src/components/StatusOrb'
import { Sparkline } from '../src/components/Sparkline'
import { Button } from '../src/components/Button'

test('segmented wraps and supports arrows, Home, End and a single tab stop', async () => {
  function Example() {
    const [value, setValue] = useState('safe')
    return (
      <Segmented
        label="Profile"
        value={value}
        onChange={setValue}
        options={['safe', 'balanced', 'custom'].map((value) => ({ value, label: value }))}
      />
    )
  }
  const user = userEvent.setup()
  render(<Example />)
  await user.tab()
  await user.keyboard('{ArrowLeft}')
  expect(screen.getByRole('radio', { name: 'custom' })).toHaveFocus()
  expect(screen.getByRole('radio', { name: 'custom' })).toBeChecked()
  await user.keyboard('{Home}{ArrowRight}')
  expect(screen.getByRole('radio', { name: 'balanced' })).toBeChecked()
  await user.keyboard('{End}')
  expect(screen.getAllByRole('radio').filter((radio) => radio.tabIndex === 0)).toHaveLength(1)
})
test('switch toggles using Space and disabled switches cannot change state', async () => {
  const onChange = vi.fn(),
    user = userEvent.setup()
  render(
    <>
      <Switch checked={false} onChange={onChange} label="Effects" />
      <Switch checked onChange={onChange} label="Disabled" disabled />
    </>,
  )
  await user.tab()
  await user.keyboard(' ')
  expect(onChange).toHaveBeenCalledExactlyOnceWith(true)
  await user.click(screen.getByRole('switch', { name: 'Disabled' }))
  expect(onChange).toHaveBeenCalledTimes(1)
})
test('orb labels measured values, clamps invalid fractions and isolates SVG ids', () => {
  const { container } = render(
    <>
      <StatusOrb value={Number.NaN} title="—" caption="RAM" />
      <StatusOrb value={5} title="16 GiB" caption="RAM" />
    </>,
  )
  expect(screen.getByRole('img', { name: 'RAM: —' })).toBeVisible()
  const gradients = Array.from(container.querySelectorAll('linearGradient')).map((el) => el.id)
  expect(new Set(gradients).size).toBe(2)
  expect(container.querySelectorAll('circle')[1]?.getAttribute('stroke-dasharray')).toMatch(/^0 /)
  expect(container.innerHTML).not.toContain('NaN')
})
test('sparklines require real finite samples and buttons preserve disabled semantics', () => {
  const onClick = vi.fn()
  const { rerender } = render(<Sparkline values={[Number.NaN, 1]} label="CPU" />)
  expect(screen.queryByRole('img')).not.toBeInTheDocument()
  rerender(
    <>
      <Sparkline values={[1, 2, 3]} label="CPU" />
      <Button disabled onClick={onClick}>
        Apply
      </Button>
    </>,
  )
  expect(screen.getByRole('img', { name: 'CPU' })).toBeVisible()
  expect(screen.getByRole('button', { name: 'Apply' })).toBeDisabled()
})
