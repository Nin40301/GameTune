import { useRef, type KeyboardEvent } from 'react'
type Props<T extends string> = {
  options: { value: T; label: string }[]
  value: T
  onChange: (value: T) => void
  label: string
  disabled?: boolean
}
export function Segmented<T extends string>({
  options,
  value,
  onChange,
  label,
  disabled,
}: Props<T>) {
  const index = Math.max(
    0,
    options.findIndex((o) => o.value === value),
  )
  const refs = useRef<(HTMLButtonElement | null)[]>([])
  function onKey(e: KeyboardEvent, i: number) {
    if (disabled || !options.length) return
    const n = options.length
    let next = i
    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') next = (i + 1) % n
    else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') next = (i - 1 + n) % n
    else if (e.key === 'Home') next = 0
    else if (e.key === 'End') next = n - 1
    else return
    e.preventDefault()
    onChange(options[next].value)
    refs.current[next]?.focus()
  }
  return (
    <div
      role="radiogroup"
      aria-label={label}
      className="segmented"
      data-count={options.length}
      data-index={index}
    >
      <span className="segmented__thumb" aria-hidden="true" />
      {options.map((o, i) => (
        <button
          key={o.value}
          type="button"
          role="radio"
          aria-checked={o.value === value}
          disabled={disabled}
          tabIndex={o.value === value ? 0 : -1}
          ref={(el) => {
            refs.current[i] = el
          }}
          className="segmented__opt"
          onClick={() => onChange(o.value)}
          onKeyDown={(e) => onKey(e, i)}
        >
          {o.label}
        </button>
      ))}
    </div>
  )
}
