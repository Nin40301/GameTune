import { useState } from 'react'
import type { Backend, Bootstrap, DiagnosticPreview, Language, SupportTool } from '../types'
import { translator } from '../i18n'
import { Button } from './Button'
import { Checkbox } from './Checkbox'
export function LegacyGuidance({ language }: { language: Language }) {
  const t = translator(language)
  return (
    <section className="notice warning" aria-labelledby="legacy-title">
      <h2 id="legacy-title">{t('legacyTitle')}</h2>
      <p>{t('legacyGuide')}</p>
    </section>
  )
}
export function SupportPanel({
  backend,
  language,
  system,
}: {
  backend: Backend
  language: Language
  system?: Bootstrap['system']
}) {
  const t = translator(language)
  const [preview, setPreview] = useState<DiagnosticPreview | null>(null)
  const [consent, setConsent] = useState(false)
  const [error, setError] = useState('')
  const [busy, setBusy] = useState(false)
  const perform = async (action: Parameters<Backend['support']>[0]) => {
    setError('')
    setBusy(true)
    try {
      setPreview(await backend.support(action))
      setConsent(false)
    } catch (e) {
      setError(typeof e === 'string' ? e : 'operation_failed')
    } finally {
      setBusy(false)
    }
  }
  const open = (tool: SupportTool) => void perform({ kind: 'open', tool })
  return (
    <section className="support-panel">
      <h2>{t('supportTitle')}</h2>
      <h3>{t('systemProtection')}</h3>
      <p>{t(`protection_${system?.protection ?? 'unknown'}`)}</p>
      <p>{t('protectionWhy')}</p>
      <div className="actions">
        <Button variant="glass" disabled={busy} onClick={() => open('protection')}>
          {t('openProtection')}
        </Button>
        <Button variant="ghost" disabled={busy} onClick={() => open('logs')}>
          {t('openLogs')}
        </Button>
      </div>
      <p>{t('diagnosticNote')}</p>
      <Button variant="glass" disabled={busy} onClick={() => void perform({ kind: 'preview' })}>
        {t('previewDiagnostic')}
      </Button>
      {error && <p role="alert">{t(error)}</p>}
      {preview && (
        <section aria-labelledby="diagnostic-title">
          <h3 id="diagnostic-title">{t('diagnosticPreview')}</h3>
          {preview.unavailable.length > 0 && <p>{t('diagnosticUnavailable')}</p>}
          {preview.files.map((file) => (
            <details key={file.name}>
              <summary>
                {file.name} · {new Intl.NumberFormat(language).format(file.bytes)}
              </summary>
              <pre>{file.content}</pre>
            </details>
          ))}
          <label className="check-line">
            <Checkbox checked={consent} onChange={(e) => setConsent(e.target.checked)} />
            {t('diagnosticConsent')}
          </label>
          <Button
            disabled={!consent || busy}
            onClick={() => void perform({ kind: 'save', token: preview.token, confirm: consent })}
          >
            {t('saveDiagnostic')}
          </Button>
        </section>
      )}
      <p className="small">{t('supportNext')}</p>
    </section>
  )
}
