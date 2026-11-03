import { operationRows } from '../state/result'
import { useApp } from '../state/AppContext'
import { OperationResult } from '../components/OperationResult'
import { Button } from '../components/Button'
import { InlineStatus } from '../components/InlineStatus'
export function ResultsPage() {
  const {
    reply,
    language,
    advanced,
    data,
    t,
    prepare,
    setPage,
    backend,
    setError,
    busy,
    activeHistory,
  } = useApp()
  if (!reply) return <p className="empty-state">{t('noResults')}</p>
  const reboot = operationRows(reply).some(
    (row) =>
      row.requiresReboot === true &&
      ['applied', 'already_applied', 'reverted'].includes(String(row.outcome)),
  )
  return (
    <section className="glass glass--pad results-panel">
      {reply.ok &&
        !operationRows(reply).some((row) => ['failed', 'error'].includes(String(row.outcome))) && (
          <div className="success-check" role="status">
            <svg viewBox="0 0 40 40" aria-hidden="true">
              <circle cx="20" cy="20" r="19" />
              <path d="m11 20 6 6 12-13" />
            </svg>
            <h2>{t('successTitle')}</h2>
          </div>
        )}
      <OperationResult
        reply={reply}
        language={language}
        advanced={advanced}
        items={data?.items ?? []}
      />
      {reboot && (
        <InlineStatus
          next={t('restartNow')}
          onNext={() =>
            void backend
              .support({ kind: 'open', tool: 'restart_dialog' })
              .catch(() => setError('operation_failed'))
          }
        >
          {t('rebootConfirmation')}
        </InlineStatus>
      )}
      <p>{t('resultNext')}</p>
      <div className="actions">
        <Button
          disabled={busy}
          onClick={() => (activeHistory.length ? void prepare('undo_all', []) : setPage('history'))}
        >
          {t(activeHistory.length ? 'undoAll' : 'viewHistory')}
        </Button>
        {activeHistory.length > 0 && (
          <Button variant="ghost" onClick={() => setPage('history')}>
            {t('viewHistory')}
          </Button>
        )}
      </div>
    </section>
  )
}
