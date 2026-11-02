import { memo, useId } from 'react'
type Props = {
  value: number
  measurement?: number
  title: string
  caption: string
  compact?: boolean
}
export const StatusOrb = memo(
  function StatusOrb({ value, title, caption, compact = false }: Props) {
    const id = useId()
    const radius = 88,
      circumference = 2 * Math.PI * radius
    const bounded = Number.isFinite(value) ? Math.min(Math.max(value, 0), 1) : 0
    return (
      <div
        className={`orb ${compact ? 'orb--compact' : ''}`}
        role="img"
        aria-label={`${caption}: ${title}`}
      >
        <svg viewBox="0 0 200 200" aria-hidden="true">
          <defs>
            <linearGradient id={id} x1="0" y1="0" x2="1" y2="1">
              <stop offset="0" stopColor="#a78bfa" />
              <stop offset="1" stopColor="#6d28d9" />
            </linearGradient>
          </defs>
          <circle
            cx="100"
            cy="100"
            r={radius}
            fill="none"
            stroke="currentColor"
            strokeOpacity=".12"
            strokeWidth="10"
          />
          <circle
            cx="100"
            cy="100"
            r={radius}
            fill="none"
            stroke={`url(#${id})`}
            strokeWidth="10"
            strokeLinecap="round"
            strokeDasharray={`${circumference * bounded} ${circumference}`}
            transform="rotate(-90 100 100)"
          />
        </svg>
        <div className="orb__center">
          <strong className="num">{title}</strong>
          <span>{caption}</span>
        </div>
      </div>
    )
  },
  (previous, next) => {
    if (previous.caption !== next.caption || previous.compact !== next.compact) return false
    const oldValue = previous.measurement ?? previous.value
    const newValue = next.measurement ?? next.value
    if (!Number.isFinite(oldValue) || !Number.isFinite(newValue)) {
      return Object.is(oldValue, newValue) && previous.title === next.title
    }
    if (oldValue === 0) return newValue === 0 && previous.title === next.title
    // Compare against the last rendered sample, so small changes accumulate.
    // The tiny tolerance prevents binary rounding from treating exactly 1% as >1%.
    return Math.abs(newValue - oldValue) / Math.abs(oldValue) <= 0.010000000000001
  },
)
