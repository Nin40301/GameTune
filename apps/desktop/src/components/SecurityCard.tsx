import { useApp } from '../state/AppContext'
import { plural } from '../i18n-format'
import { Icon } from './Icon'
export function SecurityCard({ compact = false }: { compact?: boolean }) {
  const { t, data, history, activeHistory, language, setPage, busy } = useApp()
  return (
    <section
      className={`security-card ${compact ? 'security-card--compact' : ''}`}
      aria-label={t('security')}
    >
      <h2>
        <Icon name="safe" />
        {t('security')}
      </h2>
      <p>
        {t('systemProtection')}
        <strong>
          {t(data?.system.protection === 'disabled_by_policy' ? 'protectionInactive' : 'unknown')}
        </strong>
      </p>
      <p>
        {history.available
          ? activeHistory.length
            ? plural(language, 'restorable', activeHistory.length)
            : t('restorable_zero')
          : t('historyUnavailable')}
      </p>
      <button
        type="button"
        className="button subtle"
        disabled={busy}
        onClick={() => setPage('history')}
      >
        {t('viewHistory')}
      </button>
    </section>
  )
}
