import { useId } from 'react'
type Props = { value: number; title: string; caption: string; compact?: boolean }
export function StatusOrb({ value, title, caption, compact = false }: Props) {
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
}
