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
