import { useApp } from '../state/AppContext'
import { errorKey } from '../state/errors'
import { useAnnouncement } from '../hooks/useAnnouncement'
export function ScanStatus() {
  const { scanning, analysis, ready, page, t, backend, setAnalysis, setError, language } = useApp()
  const text = t(
    analysis.status === 'running'
      ? 'analyzing'
      : analysis.status === 'cancelling'
        ? 'cancelling'
        : analysis.status === 'cancelled'
          ? 'cancelled'
          : 'complete',
  )
  const percentage = new Intl.NumberFormat(language, {
    style: 'percent',
    maximumFractionDigits: 0,
  }).format(analysis.completed / Math.max(analysis.total, 1))
  const announcement = useAnnouncement(analysis.status, `${text} ${percentage}`)
  if (!(
    scanning ||
    ((analysis.status === 'cancelled' || ready) && ['home', 'items', 'apps'].includes(page))
  ))
    return null
  return (
    <section className="scan-status" aria-label={t('progress')}>
      <span className="sr-only" role="status" aria-live="polite" aria-atomic="true">
        {announcement}
      </span>
      <div>
        <strong>{text}</strong>
        <span className="num">
          {new Intl.NumberFormat(language).format(analysis.completed)} /{' '}
          {analysis.total
            ? new Intl.NumberFormat(language).format(analysis.total)
            : t('unavailableValue')}
        </span>
      </div>
      {scanning && (
        <>
          <progress
            max={Math.max(analysis.total, 1)}
            value={analysis.completed}
            aria-label={t('progress')}
          />
          <div className="scan-bottom">
            <span>{t('scanBackground')}</span>
            <button
              className="button subtle"
              disabled={analysis.status === 'cancelling'}
              onClick={() =>
                void backend
                  .cancel()
                  .then(() => setAnalysis((old) => ({ ...old, status: 'cancelling' })))
                  .catch((e) => setError(errorKey(e)))
              }
            >
              {t('cancel')}
            </button>
          </div>
          <div className="skeleton-list" aria-hidden="true">
            <span />
            <span />
            <span />
          </div>
        </>
      )}
    </section>
  )
}
