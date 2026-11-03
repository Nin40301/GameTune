import { operationRows } from '../state/result'
import type { Item, Language, Reply } from '../types'
import { translator } from '../i18n'
export function OperationResult({
  reply,
  language,
  advanced,
  items,
}: {
  reply: Reply
  language: Language
  advanced: boolean
  items: Item[]
}) {
  const t = translator(language)
  const rows = operationRows(reply)
  return (
    <section className={`notice ${reply.ok ? 'success' : 'danger'}`} role="status">
      <strong>{t(reply.ok ? 'operationDone' : 'operationError')}</strong>
      {rows.length > 0 && (
        <ul className="result-list">
          {rows.map((row, index) => (
            <li key={index}>
              <span>
                {items.find((item) => item.id === row.itemId)?.name[language] ??
                  String(row.itemId ?? '')}
              </span>
              <strong>
                {t(
                  typeof row.outcome === 'string'
                    ? row.outcome
                    : row.ok === true
                      ? 'removed'
                      : 'failed',
                )}
              </strong>
            </li>
          ))}
        </ul>
      )}
      {advanced && (
        <details>
          <summary>{t('technical')}</summary>
          <pre>{JSON.stringify(reply.result?.operation ?? reply.result, null, 2)}</pre>
        </details>
      )}
    </section>
  )
}
