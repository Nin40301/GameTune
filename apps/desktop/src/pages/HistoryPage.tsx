import { useState } from 'react'
import { date } from '../i18n-format'
import { useApp } from '../state/AppContext'
import { OperationResult } from '../components/OperationResult'
import { Icon } from '../components/Icon'
export function HistoryPage() {
  const { page, t, reply, language, advanced, data, history, busy, prepare, activeHistory } =
    useApp()
  const [from, setFrom] = useState(''),
    [to, setTo] = useState(''),
    [kind, setKind] = useState('all')
  const matches = (itemId: string, at: string) => {
    const type = data?.items.find((item) => item.id === itemId)?.type
    const timestamp = new Date(at).getTime()
    return (
      (kind === 'all' || type === kind) &&
      (!from || timestamp >= new Date(from + 'T00:00:00').getTime()) &&
      (!to || timestamp <= new Date(to + 'T23:59:59.999').getTime())
    )
  }
  const entries =
    history.journal?.entries.filter((entry) => matches(entry.itemId, entry.appliedAt)) ?? []
  const apps = history.apps.filter((app) => matches(app.itemId, app.attemptedAt))
  return (
    <>
      {page === 'history' && (
        <section className="history-panel glass glass--pad">
          {reply && (
            <OperationResult
              reply={reply}
              language={language}
              advanced={advanced}
              items={data?.items ?? []}
            />
          )}
          {!history.available ? (
            <section className="surface empty-state">
              <h2>{t('protectedHistory')}</h2>
              <p>{t('protectedHistoryNote')}</p>
              <button
                className="button primary"
                disabled={busy}
                onClick={() => void prepare('history', [])}
              >
                {t('loadHistory')}
              </button>
            </section>
          ) : (
            <>
              {(history.journal?.needs_recovery || history.journal?.torn_tail) && (
                <p className="notice warning" role="alert">
                  {t('recover')}
                </p>
              )}
              <div className="section-top">
                <h2>{t('items')}</h2>
                <button
                  className="button primary"
                  disabled={!activeHistory.length || busy}
                  onClick={() => void prepare('undo_all', [])}
                >
                  {t('undoAll')}
                </button>
              </div>
              <div className="history-filters">
                <label>
                  {t('historyFrom')}
                  <input type="date" value={from} onChange={(e) => setFrom(e.target.value)} />
                </label>
                <label>
                  {t('historyTo')}
                  <input type="date" value={to} onChange={(e) => setTo(e.target.value)} />
                </label>
                <label>
                  {t('historyType')}
                  <span className="select-field">
                    <select value={kind} onChange={(e) => setKind(e.target.value)}>
                      {[
                        'all',
                        'registry',
                        'service',
                        'scheduled_task',
                        'startup',
                        'power',
                        'appx',
                      ].map((value) => (
                        <option key={value} value={value}>
                          {t(value)}
                        </option>
                      ))}
                    </select>
                    <Icon name="chevronDown" />
                  </span>
                </label>
                <button
                  className="button subtle"
                  onClick={() => {
                    setFrom('')
                    setTo('')
                    setKind('all')
                  }}
                >
                  {t('filterReset')}
                </button>
              </div>
              {!entries.length ? (
                <p className="empty-state">
                  {t(from || to || kind !== 'all' ? 'noHistoryMatches' : 'emptyHistory')}
                </p>
              ) : (
                <ul className="history-list">
                  {entries.map((entry) => (
                    <li key={entry.itemId}>
                      <div>
                        <strong>
                          {data?.items.find((item) => item.id === entry.itemId)?.name[language] ??
                            entry.itemId}
                        </strong>
                        <time dateTime={entry.appliedAt}>{date(language, entry.appliedAt)}</time>
                      </div>
                      <span className="pill">{t(entry.status)}</span>
                      <button
                        className="button subtle"
                        disabled={entry.status === 'reverted' || busy}
                        onClick={() => void prepare('undo', [entry.itemId])}
                      >
                        {t('undo')}
                      </button>
                    </li>
                  ))}
                </ul>
              )}
              <h2 className="space-top">{t('removedApps')}</h2>
              <p className="muted small">{t('noUndoDetail')}</p>
              <ul className="history-list">
                {apps.map((app, index) => (
                  <li key={`${app.itemId}-${index}`}>
                    <div>
                      <strong>
                        {data?.items.find((item) => item.id === app.itemId)?.name[language] ??
                          app.name}
                      </strong>
                      <span className="mono small">{app.packageFamilyNames.join(', ')}</span>
                    </div>
                    <span className="pill">{t(app.status)}</span>
                    <button
                      className="button"
                      disabled={busy}
                      onClick={() => void prepare('reinstall', [app.itemId])}
                    >
                      {t('reinstall')}
                      <Icon name="arrowUp" />
                    </button>
                  </li>
                ))}
              </ul>
            </>
          )}
        </section>
      )}
    </>
  )
}
