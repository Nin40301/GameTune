import { memo } from 'react'
import type { Comparison, Language } from '../types'
import { translator } from '../i18n'
// Display the Rust distributions unchanged; statistics are never recomputed in the UI.
export const IntervalChart = memo(function IntervalChart({
  comparison: c,
  language,
}: {
  comparison: Comparison
  language: Language
}) {
  const t = translator(language),
    format = (n: number) => new Intl.NumberFormat(language, { maximumFractionDigits: 1 }).format(n)
  const low = Math.min(c.before.p5, c.after.p5),
    high = Math.max(c.before.p95, c.after.p95),
    span = Math.max(high - low, 1)
  const x = (value: number) => 72 + ((value - low) / span) * 288
  return (
    <figure className="interval-chart">
      <figcaption>
        {t(`metric_${c.metric}`)} · {t('rangeLegend')}
      </figcaption>
      <svg
        viewBox="0 0 420 136"
        role="img"
        aria-label={`${t('beforeLegend')}: ${format(c.before.median)} (${format(c.before.p5)}–${format(c.before.p95)}); ${t('afterLegend')}: ${format(c.after.median)} (${format(c.after.p5)}–${format(c.after.p95)})`}
      >
        <path className="chart-axis" d="M72 104H360" />
        {[c.before, c.after].map((distribution, index) => (
          <g key={index} className={index ? 'chart-after' : 'chart-before'}>
            <text x="4" y={35 + index * 42}>
              {t(index ? 'after' : 'before')}
            </text>
            <line
              x1={x(distribution.p5)}
              x2={x(distribution.p95)}
              y1={30 + index * 42}
              y2={30 + index * 42}
            />
            <circle cx={x(distribution.median)} cy={30 + index * 42} r="5" />
          </g>
        ))}
        <text x="72" y="126">
          {format(low)}
        </text>
        <text x="360" y="126" textAnchor="end">
          {format(high)}
        </text>
      </svg>
    </figure>
  )
})
