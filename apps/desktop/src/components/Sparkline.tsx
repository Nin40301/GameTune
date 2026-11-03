import { memo } from 'react'
export const Sparkline = memo(
  function Sparkline({ values, label }: { values: number[]; label: string }) {
    const samples = values.filter(Number.isFinite).slice(-60)
    if (samples.length < 2) return null
    const low = Math.min(...samples),
      high = Math.max(...samples)
    const span = Math.max(high - low, 1)
    const points = samples
      .map((v, i) => `${4 + (i / (samples.length - 1)) * 232},${44 - ((v - low) / span) * 36}`)
      .join(' ')
    return (
      <svg className="sparkline" viewBox="0 0 240 52" role="img" aria-label={label}>
        <path d="M4 48H236" fill="none" stroke="currentColor" strokeOpacity=".16" />
        <polyline
          points={points}
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinejoin="round"
          strokeLinecap="round"
        />
      </svg>
    )
  },
  (previous, next) =>
    previous.label === next.label &&
    previous.values.length === next.values.length &&
    previous.values.every((value, index) => Object.is(value, next.values[index])),
)
