import { useApp } from '../state/AppContext'
import { BrandMark, Icon } from './Icon'
export function StatusStrip() {
  const { t, data, busy, setPage } = useApp()
  return (
    <header className="topbar glass status-strip">
      <span className="topbar-location mobile-brand">
        <BrandMark />
        {t('brand')}
      </span>
      <ul aria-label={t('statusStrip')}>
        <li>
          <button
            type="button"
            className="status-chip"
            disabled={busy}
            onClick={() => setPage('settings')}
          >
            <Icon name="safe" />
            <span>
              {t('systemProtection')}
              <strong>
                {t(
                  data?.system.protection === 'disabled_by_policy'
                    ? 'protectionInactive'
                    : 'unknown',
                )}
              </strong>
            </span>
          </button>
        </li>
        <li>
          <button
            type="button"
            className="status-chip"
            disabled={busy}
            onClick={() => setPage('settings')}
          >
            <Icon name="offline" />
            <span>
              {t('offline')}
              <strong>{t('noTelemetry')}</strong>
            </span>
          </button>
        </li>
      </ul>
      <span className="mode-label">{t(busy ? 'working' : 'readOnly')}</span>
    </header>
  )
}
