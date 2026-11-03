import { appendCpuSample } from '../state/visualHistory'
import { Button } from '../components/Button'
import { InlineStatus } from '../components/InlineStatus'
import { useWindowActivity } from '../hooks/useWindowActivity'
import { Checkbox } from '../components/Checkbox'
import { useEffect, useState, type ReactNode } from 'react'
import { StatusOrb } from '../components/StatusOrb'
import { Icon } from '../components/Icon'
import { Sparkline } from '../components/Sparkline'
import type { Backend, Conditions, LabAction, LabFile, LabStatus, Language } from '../types'
import { translator } from '../i18n'

const unchecked: Conditions = {
  appsClosed: false,
  sameWorkload: false,
  cooled: false,
  samePower: false,
}
export function Measurements({
  backend,
  language,
  dashboard = false,
  profileControls,
  safetyCard,
}: {
  backend: Backend
  language: Language
  dashboard?: boolean
  profileControls?: ReactNode
  safetyCard?: ReactNode
}) {
  const active = useWindowActivity()
  const [processesOpen, setProcessesOpen] = useState(false)
  const [cpuSamples, setCpuSamples] = useState<{ time: number; value: number }[]>([])
  const [ramSamples, setRamSamples] = useState<{ time: number; value: number }[]>([])
  const [status, setStatus] = useState<LabStatus | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)
  const [conditions, setConditions] = useState<Conditions>(unchecked)
  const [reset, setReset] = useState(false)
  const [anonymize, setAnonymize] = useState(true)
  const [saved, setSaved] = useState(false)
  const t = translator(language)
  const number = (value: number | null | undefined, digits = 1) =>
    value == null
      ? '—'
      : new Intl.NumberFormat(language, { maximumFractionDigits: digits }).format(value)
  const gib = (value: number | undefined) =>
    value == null
      ? '—'
      : `${new Intl.NumberFormat(language, { minimumFractionDigits: 1, maximumFractionDigits: 1 }).format(value / 1073741824)} ${t('gibUnit')}`
  useEffect(() => {
    if (!active) return
    let alive = true,
      pending = false
    const poll = () => {
      if (pending || document.visibilityState === 'hidden') return
      pending = true
      void backend
        .lab({ kind: 'status' })
        .then((s) => {
          if (alive) {
            setStatus(s)
            const latest = s.latest
            if (latest && Number.isFinite(latest.availableBytes))
              setRamSamples((old) =>
                appendCpuSample(old, {
                  time: latest.timestampMs,
                  value: latest.availableBytes / 1073741824,
                }),
              )
            if (latest?.cpuPercent != null && Number.isFinite(latest.cpuPercent)) {
              const value = latest.cpuPercent
              setCpuSamples((old) => appendCpuSample(old, { time: latest.timestampMs, value }))
            }
          }
        })
        .catch((e: unknown) => {
          if (alive) setError(typeof e === 'string' ? e : 'metrics_unavailable')
        })
        .finally(() => {
          pending = false
        })
    }
    poll()
    const timer = setInterval(poll, dashboard ? 2000 : 1000)
    return () => {
      alive = false
      clearInterval(timer)
    }
  }, [backend, dashboard, active])
  async function act(action: LabAction) {
    setBusy(true)
    setError(null)
    setSaved(false)
    try {
      setStatus(await backend.lab(action))
      if (action.kind === 'start') setConditions(unchecked)
    } catch (e) {
      setError(typeof e === 'string' ? e : 'operation_failed')
    } finally {
      setBusy(false)
    }
  }
  async function file(action: LabFile) {
    setBusy(true)
    setError(null)
    setSaved(false)
    try {
      setStatus(await backend.labFile(action))
      if (action.kind === 'export') setSaved(true)
    } catch (e) {
      setError(typeof e === 'string' ? e : 'operation_failed')
    } finally {
      setBusy(false)
    }
  }
  const sample = status?.latest,
    p = status?.protocol
  const running = p?.pendingRun ?? false
  const completed =
    p?.phase === 'after' || p?.phase === 'complete' ? p.after.length : (p?.before.length ?? 0)
  return (
    <section className="measurements" aria-label={t(dashboard ? 'measuredDashboard' : 'benchmark')}>
      {(error || status?.error) && (
        <InlineStatus error next={t('retry')} onNext={() => void act({ kind: 'status' })}>
          {t(error ?? status?.error ?? 'metrics_unavailable')}
        </InlineStatus>
      )}
      {dashboard ? (
        <>
          <div className="section-top">
            <h2>{t('measuredDashboard')}</h2>
            <Button
              variant="ghost"
              icon={<Icon name={status?.monitor ? 'pause' : 'play'} />}
              disabled={busy || !status}
              onClick={() => {
                void act({ kind: 'monitor', enabled: !status?.monitor })
              }}
            >
              {t(status?.monitor ? 'pauseMetrics' : 'resumeMetrics')}
            </Button>
          </div>
          {!status?.monitor && <p className="notice">{t('metricsPaused')}</p>}
          <div className="overview">
            <section className="glass glass--strong glass--pad profile-panel">
              <div className="ram-status">
                <StatusOrb
                  value={
                    sample && sample.totalBytes > 0 ? sample.availableBytes / sample.totalBytes : 0
                  }
                  measurement={sample?.availableBytes}
                  title={gib(sample?.availableBytes)}
                  caption={
                    sample
                      ? t('availableOf').replace('{total}', gib(sample.totalBytes))
                      : t('availableRam')
                  }
                />
              </div>
              {profileControls}
            </section>
            <aside className="vitals" aria-label={t('systemSnapshot')}>
              <article className="surface metric metric--processes">
                <div>
                  <h3>
                    <Icon name="layers" />
                    {t('processCount')}
                  </h3>
                  <p>{t('systemSnapshot')}</p>
                </div>
                <strong className="num">{number(sample?.processCount, 0)}</strong>
              </article>
              <article className="surface metric metric--cpu">
                <h3>
                  <Icon name="cpu" />
                  {t('systemCpu')}
                </h3>
                <strong className="num">
                  {number(sample?.cpuPercent)} <small>%</small>
                </strong>
                <Sparkline values={cpuSamples.map((s) => s.value)} label={t('cpuHistory')} />
                <p>{t('cpuNote')}</p>
              </article>
              <article className="surface metric metric--memory">
                <h3>
                  <Icon name="memory" />
                  {t('memoryHistory')}
                </h3>
                <strong className="num">{gib(sample?.availableBytes)}</strong>
                <Sparkline values={ramSamples.map((s) => s.value)} label={t('memoryHistory')} />
                <p>{t('sampleShortHistory')}</p>
              </article>
              <section className="surface top-three">
                <h3>{t('topThree')}</h3>
                {sample ? (
                  <ol>
                    {sample.top.slice(0, 3).map((process) => (
                      <li key={process.name}>
                        <span>{process.name}</span>
                        <strong className="num">
                          {number(process.memoryBytes / 1048576)} {t('mibUnit')}
                        </strong>
                      </li>
                    ))}
                  </ol>
                ) : (
                  <p>{t('metricsWaiting')}</p>
                )}
              </section>
              {safetyCard}
              <p className="footprint">
                <Icon name="memory" />
                {t('appFootprint')}: {sample ? number(sample.appPrivateBytes / 1048576) : '—'}{' '}
                {t('mibUnit')} · {number(sample?.appCpuPercent, 2)} {t('percentUnit')}{' '}
                {t('cpuUnit')} · {number(sample?.appProcesses, 0)}{' '}
                {t('processCount').toLocaleLowerCase()}
              </p>
            </aside>
          </div>
          <p className="muted">{t('metricNote')}</p>
          <details
            className="surface process-list"
            onToggle={(event) => setProcessesOpen(event.currentTarget.open)}
          >
            <summary>{t('topMemory')}</summary>
            {processesOpen &&
              (sample ? (
                <div
                  className="table-scroll"
                  role="region"
                  tabIndex={0}
                  aria-label={t('tableNavigation')}
                >
                  <table>
                    <caption>{t('groupedMemory')}</caption>
                    <thead>
                      <tr>
                        <th scope="col">{t('processName')}</th>
                        <th scope="col">{t('instances')}</th>
                        <th scope="col">{t('mibUnit')}</th>
                      </tr>
                    </thead>
                    <tbody>
                      {sample.top.map((row) => (
                        <tr key={row.name}>
                          <th scope="row">{row.name}</th>
                          <td>{row.count}</td>
                          <td>{number(row.memoryBytes / 1048576)}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              ) : (
                <p>{t('metricsWaiting')}</p>
              ))}
          </details>
          {sample && (
            <p className="sample-stamp">
              {t('sampleTime')}{' '}
              <time dateTime={new Date(sample.timestampMs).toISOString()}>
                {new Intl.DateTimeFormat(language, { timeStyle: 'medium' }).format(
                  new Date(sample.timestampMs),
                )}
              </time>
            </p>
          )}
        </>
      ) : (
        <>
          <div className="measurement-overview">
            <StatusOrb
              compact
              value={
                sample && sample.totalBytes > 0 ? sample.availableBytes / sample.totalBytes : 0
              }
              measurement={sample?.availableBytes}
              title={gib(sample?.availableBytes)}
              caption={t('availableRam')}
            />
            <div>
              <p className="intro">{t('benchmarkIntro')}</p>
              <Sparkline values={cpuSamples.map((s) => s.value)} label={t('cpuHistory')} />
            </div>
          </div>
          <div className="benchmark-layout">
            <section className="surface benchmark-protocol glass">
              <p className="eyebrow">{t('protocol')}</p>
              <h2>{t(`phase_${p?.phase ?? 'before'}`)}</h2>
              <p>{t('benchmarkRules')}</p>
              <ol className="benchmark-steps">
                <li>
                  {t('beforeRuns')} <strong>{p?.before.length ?? 0} / 5</strong>
                </li>
                <li>{t('applyReboot')}</li>
                <li>
                  {t('afterRuns')} <strong>{p?.after.length ?? 0} / 5</strong>
                </li>
              </ol>
              {p?.interrupted && (
                <p className="notice warning" role="status">
                  {t('benchmarkInterrupted')}
                </p>
              )}
              {p?.phase === 'await_apply' && (
                <p className="notice warning">{t('awaitApplyNote')}</p>
              )}
              {p?.phase === 'await_reboot' && (
                <p className="notice warning">{t('awaitRebootNote')}</p>
              )}
              <p className="muted">
                {t('powerPlan')}{' '}
                <span className="mono break-text">{p?.powerPlan ?? sample?.powerPlan ?? '—'}</span>
              </p>
              {running ? (
                <div className="run-progress">
                  <p role="status">
                    {t('runInProgress')} {completed + 1} / 5 · {status?.elapsedSeconds ?? 0} /{' '}
                    {number(60, 0)} {t('secondsUnit')}
                  </p>
                  <progress
                    max={60}
                    value={status?.elapsedSeconds ?? 0}
                    aria-label={t('runProgress')}
                  />
                  <button
                    className="button secondary"
                    disabled={busy}
                    onClick={() => {
                      void act({ kind: 'cancel' })
                    }}
                  >
                    {t('cancelRun')}
                  </button>
                </div>
              ) : (
                <>
                  <fieldset
                    disabled={busy || p?.phase === 'await_apply' || p?.phase === 'complete'}
                  >
                    <legend>{t('confirmConditions')}</legend>
                    {(['appsClosed', 'sameWorkload', 'cooled', 'samePower'] as const).map((key) => (
                      <label className="check-line" key={key}>
                        <Checkbox
                          checked={conditions[key]}
                          onChange={(e) =>
                            setConditions({ ...conditions, [key]: e.target.checked })
                          }
                        />
                        {t(`condition_${key}`)}
                      </label>
                    ))}
                  </fieldset>
                  {!status?.monitor && (
                    <button
                      className="button secondary"
                      disabled={busy}
                      onClick={() => {
                        void act({ kind: 'monitor', enabled: true })
                      }}
                    >
                      {t('resumeMetrics')}
                    </button>
                  )}
                  <button
                    className="button primary"
                    disabled={
                      busy ||
                      (!sample?.cpuPercent && sample?.cpuPercent !== 0) ||
                      !Object.values(conditions).every(Boolean) ||
                      p?.phase === 'await_apply' ||
                      p?.phase === 'complete'
                    }
                    onClick={() => {
                      void act({ kind: 'start', conditions })
                    }}
                  >
                    {t('startRun')}
                  </button>
                </>
              )}
            </section>
            <aside className="surface benchmark-notes">
              <h2>{t('readResults')}</h2>
              <p>{t('noiseExplanation')}</p>
              <p>{t('temperatureNote')}</p>
              <p>{t('persistenceNote')}</p>
              <details>
                <summary>{t('resetExperiment')}</summary>
                <label className="check-line">
                  <Checkbox checked={reset} onChange={(e) => setReset(e.target.checked)} />
                  {t('confirmReset')}
                </label>
                <button
                  className="button secondary"
                  disabled={!reset || running || busy}
                  onClick={() => {
                    void act({ kind: 'reset', confirm: true })
                    setReset(false)
                  }}
                >
                  {t('resetExperiment')}
                </button>
              </details>
            </aside>
          </div>
          <section className="surface results-panel">
            <h2>{t('comparison')}</h2>
            {!status?.comparisons.length ? (
              <p>{t('noComparison')}</p>
            ) : (
              <>
                <p>{t('comparisonNote')}</p>
                <div
                  className="table-scroll"
                  role="region"
                  tabIndex={0}
                  aria-label={t('tableNavigation')}
                >
                  <table>
                    <caption>{t('runMediansCaption')}</caption>
                    <thead>
                      <tr>
                        <th scope="col">{t('metric')}</th>
                        <th scope="col">{t('before')}</th>
                        <th scope="col">{t('after')}</th>
                        <th scope="col">{t('noiseBand')}</th>
                        <th scope="col">{t('conclusion')}</th>
                      </tr>
                    </thead>
                    <tbody>
                      {status.comparisons.map((c) => (
                        <tr key={c.metric}>
                          <th scope="row">{t(`metric_${c.metric}`)}</th>
                          <td>
                            {number(c.before.median)}
                            <Sparkline
                              values={c.before.runMedians}
                              label={`${t('before')} · ${t(`metric_${c.metric}`)}`}
                            />
                            <small>
                              {t('p5Label')} {number(c.before.p5)} · {t('p95Label')}{' '}
                              {number(c.before.p95)}
                            </small>
                            <small>
                              {t('runVariation')} {number(c.before.mad)} ·{' '}
                              {c.before.runMedians.map((v) => number(v)).join(' / ')}
                            </small>
                          </td>
                          <td>
                            {number(c.after.median)}
                            <Sparkline
                              values={c.after.runMedians}
                              label={`${t('after')} · ${t(`metric_${c.metric}`)}`}
                            />
                            <small>
                              {t('p5Label')} {number(c.after.p5)} · {t('p95Label')}{' '}
                              {number(c.after.p95)}
                            </small>
                            <small>
                              {t('runVariation')} {number(c.after.mad)} ·{' '}
                              {c.after.runMedians.map((v) => number(v)).join(' / ')}
                            </small>
                          </td>
                          <td>± {number(c.noiseBand)}</td>
                          <td>
                            <span className="pill">{t(c.conclusion)}</span>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </>
            )}
          </section>
          <section className="surface results-panel">
            <h2>{t('optionalPresentmon')}</h2>
            <p>{t('presentmonExplanation')}</p>
            <div className="action-row">
              <button
                className="button secondary"
                disabled={busy || running}
                onClick={() => {
                  void file({ kind: 'choose_presentmon' })
                }}
              >
                {t('choosePresentmon')}
              </button>
              <button
                className="button secondary"
                disabled={busy || running || !status?.presentmonAvailable}
                onClick={() => {
                  void file({ kind: 'import_capture' })
                }}
              >
                {t('importPresentmon')}
              </button>
            </div>
            <p className="muted">{t('presentmonDefinition')}</p>
            {status?.presentmonAvailable &&
              p?.presentmon.map((capture, index) => (
                <article className="capture" key={index}>
                  <h3>{capture.application}</h3>
                  <dl>
                    <dt>{t('averageFps')}</dt>
                    <dd>{number(capture.averageFps)}</dd>
                    <dt>{t('onePercentLow')}</dt>
                    <dd>{number(capture.onePercentLowFps)}</dd>
                    <dt>{t('frameTime')}</dt>
                    <dd>
                      {number(capture.frameTime.median)} {t('millisecondsUnit')} · {t('p95Label')}{' '}
                      {number(capture.frameTime.p95)} {t('millisecondsUnit')}· {t('p99Label')}{' '}
                      {number(capture.frameTimeP99)} {t('millisecondsUnit')}
                    </dd>
                  </dl>
                  <p>
                    {capture.frames} {t('capturedFrames')}
                  </p>
                </article>
              ))}
          </section>
          <section className="surface results-panel">
            <h2>{t('exportMeasurements')}</h2>
            <label className="check-line">
              <Checkbox checked={anonymize} onChange={(e) => setAnonymize(e.target.checked)} />
              {t('anonymizeReport')}
            </label>
            <p className="muted">{t('anonymizeNote')}</p>
            <div className="action-row">
              {(['json', 'html'] as const).map((format) => (
                <button
                  key={format}
                  className={`button ${format === 'json' && p?.phase === 'complete' ? 'primary' : 'secondary'}`}
                  disabled={busy || !p}
                  onClick={() => {
                    void file({ kind: 'export', format, anonymize, english: language === 'en' })
                  }}
                >
                  {t(format === 'json' ? 'exportJson' : 'exportHtml')}
                </button>
              ))}
            </div>
            {saved && (
              <InlineStatus next={t('dismiss')} onNext={() => setSaved(false)}>
                {t('exportDialogDone')}
              </InlineStatus>
            )}
          </section>
        </>
      )}
    </section>
  )
}
