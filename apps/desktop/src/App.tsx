import { useWindowActivity } from './hooks/useWindowActivity'
import { Checkbox } from './components/Checkbox'
import { useEffect, useRef, useState } from 'react'
import type {
  Analysis,
  Approval,
  Backend,
  Bootstrap,
  History,
  Item,
  Language,
  Observation,
  Page,
  Reply,
  Review,
  ReviewKind,
  Theme,
} from './types'
import { categoryKey, translator } from './i18n'
import { Measurements } from './Measurements'
import { GamesPanel } from './Games'
import { version } from '../package.json'
import { Button } from './components/Button'
import { Segmented } from './components/Segmented'
import { Switch } from './components/Switch'
import { BrandMark, Icon } from './components/Icon'

const emptyAnalysis: Analysis = {
  id: 0,
  status: 'idle',
  completed: 0,
  total: 0,
  itemId: '',
  observations: [],
  error: null,
}
const primaryNavigation: Page[] = ['home', 'items', 'benchmark', 'games']
const navigation: Page[] = [
  'home',
  'items',
  'apps',
  'review',
  'history',
  'benchmark',
  'games',
  'settings',
]
function preference(key: string, fallback: string) {
  try {
    return localStorage.getItem(`ametista.${key}`) ?? fallback
  } catch {
    return fallback
  }
}
function persist(key: string, value: string) {
  try {
    localStorage.setItem(`ametista.${key}`, value)
  } catch {
    /* Preferences never authorize operations. */
  }
}
function errorKey(error: unknown) {
  return error instanceof Error
    ? error.message
    : typeof error === 'string'
      ? error
      : 'operation_failed'
}
export function App({ backend }: { backend: Backend }) {
  const active = useWindowActivity()
  useEffect(() => {
    document.documentElement.dataset.windowActive = String(active)
  }, [active])
  const [language, setLanguage] = useState<Language>(() =>
    preference('language', 'pt-BR') === 'en' ? 'en' : 'pt-BR',
  )
  const [theme, setTheme] = useState<Theme>(() =>
    preference('theme', 'light') === 'dark' ? 'dark' : 'light',
  )
  const [effects, setEffects] = useState<'full' | 'reduced'>(() =>
    preference('effects', 'reduced') === 'full' ? 'full' : 'reduced',
  )
  const [advanced, setAdvanced] = useState(() => preference('advanced', 'false') === 'true')
  const [compactNavigation, setCompactNavigation] = useState(
    () =>
      typeof window.matchMedia === 'function' && window.matchMedia('(max-width: 900px)').matches,
  )
  const moreMenu = useRef<HTMLDetailsElement>(null)
  useEffect(() => {
    if (typeof window.matchMedia !== 'function') return
    const media = window.matchMedia('(max-width: 900px)')
    const update = () => setCompactNavigation(media.matches)
    media.addEventListener('change', update)
    return () => media.removeEventListener('change', update)
  }, [])
  const [gameRecovery, setGameRecovery] = useState(false)
  const [page, setPage] = useState<Page>('home'),
    [data, setData] = useState<Bootstrap | null>(null)
  const [history, setHistory] = useState<History>({ available: false, journal: null, apps: [] })
  const [analysis, setAnalysis] = useState<Analysis>(emptyAnalysis),
    [profile, setProfile] = useState('safe')
  const [selected, setSelected] = useState<string[]>([]),
    [appIds, setAppIds] = useState<string[]>([])
  const [search, setSearch] = useState(''),
    [category, setCategory] = useState('all')
  const [detail, setDetail] = useState<Item | null>(null),
    [review, setReview] = useState<Review | null>(null)
  const [error, setError] = useState<string | null>(null),
    [busy, setBusy] = useState(false),
    [reply, setReply] = useState<Reply | null>(null)
  const heading = useRef<HTMLHeadingElement>(null),
    t = translator(language)
  const previousPage = useRef<Page>('home')
  const scanning = analysis.status === 'running' || analysis.status === 'cancelling'
  const ready = analysis.status === 'complete'
  useEffect(() => {
    document.documentElement.lang = language
    persist('language', language)
  }, [language])
  useEffect(() => {
    document.documentElement.dataset.theme = theme
    persist('theme', theme)
  }, [theme])
  useEffect(() => {
    document.documentElement.dataset.effects = effects
    persist('effects', effects)
  }, [effects])
  useEffect(() => {
    persist('advanced', String(advanced))
  }, [advanced])
  useEffect(() => {
    let alive = true
    void backend
      .games({ kind: 'status' })
      .then((view) => {
        if (alive) setGameRecovery(view.monitor.recoveryPending && !view.monitor.running)
      })
      .catch(() => {})
    void backend
      .bootstrap()
      .then((value) => {
        if (alive) {
          setData(value)
          setHistory(value.history)
          setSelected(value.presets.safe ?? [])
        }
      })
      .catch((e) => {
        if (alive) setError(errorKey(e))
      })
    return () => {
      alive = false
    }
  }, [backend])
  useEffect(() => {
    if (previousPage.current !== page) {
      heading.current?.focus()
      previousPage.current = page
    }
  }, [page])
  useEffect(() => {
    if (!scanning) return
    let active = true,
      pending = false
    const timer = setInterval(() => {
      if (pending) return
      pending = true
      void backend
        .state()
        .then((value) => {
          if (active) setAnalysis(value)
        })
        .catch((e) => {
          if (active) {
            setError(errorKey(e))
            setAnalysis((old) => ({ ...old, status: 'error' }))
          }
        })
        .finally(() => {
          pending = false
        })
    }, 200)
    return () => {
      active = false
      clearInterval(timer)
    }
  }, [backend, scanning])
  const rows = new Map(analysis.observations.map((row) => [row.itemId, row]))
  const name = (item: Item) => item.name[language]
  const categoryLabel = (value: string) => t(categoryKey(value))
  function chooseProfile(value: string) {
    setProfile(value)
    setSelected(data?.presets[value] ?? [])
    setReview(null)
  }
  function toggle(id: string, apps = false) {
    const update = (old: string[]) =>
      old.includes(id) ? old.filter((value) => value !== id) : [...old, id]
    if (apps) setAppIds(update)
    else {
      setSelected(update)
      setProfile('custom')
    }
    setReview(null)
  }
  async function analyze() {
    setError(null)
    setReply(null)
    setReview(null)
    setAppIds([])
    try {
      const id = await backend.start()
      setAnalysis({ ...emptyAnalysis, id, status: 'running' })
    } catch (e) {
      setError(errorKey(e))
    }
  }
  async function prepare(kind: ReviewKind, ids: string[]) {
    setError(null)
    setBusy(true)
    try {
      setReview(await backend.review({ kind, ids }))
      setPage('review')
    } catch (e) {
      setError(errorKey(e))
    } finally {
      setBusy(false)
    }
  }
  async function execute(approval: Approval) {
    setBusy(true)
    setError(null)
    try {
      const value = await backend.execute(approval)
      setReply(value)
      if (value.result?.journal)
        setHistory({
          available: true,
          journal: value.result.journal,
          apps: value.result.apps ?? [],
        })
      if (!value.ok) setError(value.error ?? 'operation_failed')
      setReview(null)
      setPage('history')
      setAnalysis(emptyAnalysis)
    } catch (e) {
      setError(errorKey(e))
      setReview(null)
      setAnalysis(emptyAnalysis)
    } finally {
      setBusy(false)
    }
  }
  const list = (data?.items ?? []).filter(
    (item) =>
      item.type !== 'appx' &&
      (advanced || item.verified) &&
      (!search ||
        `${name(item)} ${item.description[language]}`
          .toLocaleLowerCase()
          .includes(search.toLocaleLowerCase())) &&
      (category === 'all' || item.category === category),
  )
  const apps = (data?.items ?? []).filter(
    (item) =>
      item.type === 'appx' &&
      rows.get(item.id)?.exists === true &&
      rows.get(item.id)?.removable === true,
  )
  const title =
    page === 'home'
      ? t('homeTitle')
      : page === 'apps'
        ? t('appsTitle')
        : page === 'review'
          ? t('reviewTitle')
          : page === 'settings'
            ? t('settingsTitle')
            : t(page)
  const canSelect = (item: Item) =>
    !data?.blockedIds.includes(item.id) &&
    data?.compatible[item.id] !== false &&
    item.revert === 'auto' &&
    !(item.type === 'startup' && !item.target.entry)
  const activeHistory =
    history.journal?.entries.filter((entry) => entry.status !== 'reverted') ?? []
  function navigate(item: Page) {
    setPage(item)
    setSearch('')
    setCategory('all')
    if (moreMenu.current) moreMenu.current.open = false
  }
  function navigationButton(item: Page) {
    return (
      <button
        type="button"
        aria-label={t(item)}
        aria-current={page === item ? 'page' : undefined}
        disabled={busy || (item === 'review' && !review)}
        onClick={() => navigate(item)}
      >
        <span className="nav-symbol" aria-hidden="true">
          <Icon name={item} />
        </span>
        <span className="nav-label">{t(item)}</span>
        <span className="nav-short" aria-hidden="true">
          {t(`short_${item}`)}
        </span>
        {item === 'items' && selected.length > 0 && (
          <span className="nav-count">{selected.length}</span>
        )}
      </button>
    )
  }
  return (
    <div className="app-shell app-bg">
      <a className="skip-link" href="#main">
        {t('skip')}
      </a>
      <aside className="sidebar glass">
        <div className="brand">
          <BrandMark />
          <div>
            <strong>{t('brand')}</strong>
            <small>{t('tagline')}</small>
          </div>
        </div>
        <nav className="nav" data-index={navigation.indexOf(page)} aria-label={t('nav')}>
          <span className="nav__pill" aria-hidden="true" />
          <ul>
            {(compactNavigation ? primaryNavigation : navigation).map((item) => (
              <li key={item}>{navigationButton(item)}</li>
            ))}
            {compactNavigation && (
              <li className="more-navigation">
                <details
                  ref={moreMenu}
                  onKeyDown={(event) => {
                    if (event.key === 'Escape') {
                      event.currentTarget.open = false
                      event.currentTarget.querySelector('summary')?.focus()
                    }
                  }}
                >
                  <summary aria-label={t('more')} data-active={!primaryNavigation.includes(page)}>
                    <Icon name="more" />
                    <span>{t('more')}</span>
                  </summary>
                  <div className="more-panel" aria-label={t('more')}>
                    {navigation
                      .filter((item) => !primaryNavigation.includes(item))
                      .map((item) => (
                        <div key={item}>{navigationButton(item)}</div>
                      ))}
                  </div>
                </details>
              </li>
            )}
          </ul>
        </nav>
        <div className="sidebar-footer">
          <span className="version">v{version}</span>
        </div>
      </aside>
      <div className="workspace">
        <header className="topbar glass">
          <span className="topbar-location">
            <span className="mobile-brand">
              <BrandMark />
              {t('brand')}
            </span>
          </span>
          <div>
            <span className="pill">
              <span className="status-dot" />
              {t('noNetwork')}
            </span>
            <span className="mode-label">
              {t(busy && page === 'review' ? 'working' : 'readOnly')}
            </span>
          </div>
        </header>
        <main id="main" tabIndex={-1} aria-busy={busy}>
          <div key={page} className="page">
            {import.meta.env.MODE === 'test' && <div className="test-mode">{t('testMode')}</div>}
            <div className="page-heading">
              <div>
                <h1 ref={heading} tabIndex={-1}>
                  {title}
                </h1>
              </div>
              {page === 'items' && (
                <div className="heading-actions">
                  <span className="counter">
                    {selected.length} {t('selected').toLocaleLowerCase()}
                  </span>
                  <Button
                    variant="glass"
                    icon={<Icon name="search" />}
                    disabled={!data || scanning || busy}
                    onClick={() => void analyze()}
                  >
                    {t('analyze')}
                  </Button>
                </div>
              )}
            </div>
            {error && (
              <div className="notice danger" role="alert">
                {t(error)}
              </div>
            )}
            {gameRecovery && page !== 'settings' && (
              <div className="notice warning" role="status">
                <p>{t('monitorRecovery')}</p>
                <button
                  className="button secondary"
                  onClick={() => {
                    setPage('settings')
                    setGameRecovery(false)
                  }}
                >
                  {t('reviewGameRecovery')}
                </button>
              </div>
            )}
            {(scanning ||
              ((analysis.status === 'cancelled' || ready) &&
                ['home', 'items', 'apps'].includes(page))) && (
              <section className="scan-status" aria-label={t('progress')}>
                <div>
                  <strong role="status" aria-live="polite">
                    {t(
                      analysis.status === 'running'
                        ? 'analyzing'
                        : analysis.status === 'cancelling'
                          ? 'cancelling'
                          : analysis.status === 'cancelled'
                            ? 'cancelled'
                            : 'complete',
                    )}
                  </strong>
                  <span className="mono">
                    {analysis.completed} / {analysis.total || '—'}
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
                      <span className="mono">
                        {advanced
                          ? analysis.itemId
                          : (data?.items.find((item) => item.id === analysis.itemId)?.name[
                              language
                            ] ?? t('analyzing'))}
                      </span>
                      <button
                        className="button subtle"
                        disabled={analysis.status === 'cancelling'}
                        onClick={() => {
                          void backend
                            .cancel()
                            .then(() => setAnalysis((old) => ({ ...old, status: 'cancelling' })))
                            .catch((e) => setError(errorKey(e)))
                        }}
                      >
                        {t('cancel')}
                      </button>
                    </div>
                  </>
                )}
              </section>
            )}
            {ready &&
              ['home', 'items', 'apps'].includes(page) &&
              analysis.observations.some((row) => row.error) && (
                <p className="notice warning" role="status">
                  {t('readError')}
                </p>
              )}
            {page === 'home' && (
              <>
                <p className="intro">{t('homeIntro')}</p>
                <Measurements
                  backend={backend}
                  language={language}
                  dashboard
                  selectedCount={selected.length}
                  profileControls={
                    <>
                      <div className="section-top">
                        <h2>{t('profile')}</h2>
                        <span className="pill accent">
                          <Icon name="safe" />
                          {t('simulated')}
                        </span>
                      </div>
                      <Segmented
                        options={['safe', 'balanced', 'aggressive', 'custom'].map((value) => ({
                          value,
                          label: t(value),
                        }))}
                        value={profile}
                        onChange={chooseProfile}
                        label={t('profile')}
                        disabled={busy}
                      />
                      <p className="profile-note">{t(`${profile}Note`)}</p>
                      <div className="actions">
                        <Button
                          icon={<Icon name="search" />}
                          disabled={!data || scanning || busy}
                          onClick={() => void analyze()}
                        >
                          {ready ? t('analyzeAgain') : t('simulate')}
                        </Button>
                        <Button
                          variant="glass"
                          disabled={!ready || !selected.length || busy}
                          onClick={() => void prepare('apply', selected)}
                        >
                          {t('reviewSelection')}
                          <Icon name="arrowRight" />
                        </Button>
                      </div>
                      <p className="calm-note">
                        <Icon name="safe" />
                        {t('simulateNote')}
                      </p>
                    </>
                  }
                />
                <section className="next-step">
                  <Icon name="items" className="mini-symbol" />
                  <div>
                    <h2>{t('quickReview')}</h2>
                    <p>{ready ? t('quickReviewNote') : t('idle')}</p>
                  </div>
                  <button className="button subtle" onClick={() => setPage('items')}>
                    {t('viewItems')}
                    <Icon name="arrowRight" />
                  </button>
                </section>
              </>
            )}
            {page === 'items' && (
              <>
                <div className="toolbar glass">
                  <label className="search-label">
                    <span>{t('search')}</span>
                    <span className="search-field">
                      <Icon name="search" />
                      <input
                        type="search"
                        placeholder={t('searchPlaceholder')}
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                      />
                    </span>
                  </label>
                  <label>
                    {t('category')}
                    <span className="select-field">
                      <select value={category} onChange={(e) => setCategory(e.target.value)}>
                        <option value="all">{t('all')}</option>
                        {[
                          ...new Set(
                            (data?.items ?? [])
                              .filter((item) => item.type !== 'appx')
                              .map((item) => item.category),
                          ),
                        ].map((value) => (
                          <option key={value} value={value}>
                            {categoryLabel(value)}
                          </option>
                        ))}
                      </select>
                      <Icon name="chevronDown" />
                    </span>
                  </label>
                </div>
                <div className="item-list surface">
                  {[...new Set(list.map((item) => item.category))].map((group) => (
                    <section key={group} aria-label={categoryLabel(group)}>
                      <div className="group-heading">
                        <h2>{categoryLabel(group)}</h2>
                        <span className="mono">
                          {list.filter((item) => item.category === group).length}
                        </span>
                      </div>
                      {list
                        .filter((item) => item.category === group)
                        .map((item) => (
                          <ItemRow
                            key={item.id}
                            item={item}
                            language={language}
                            observation={rows.get(item.id)}
                            checked={selected.includes(item.id)}
                            disabled={!canSelect(item) || busy}
                            blocked={data?.blockedIds.includes(item.id) ?? false}
                            incompatible={data?.compatible[item.id] === false}
                            onToggle={() => toggle(item.id)}
                            onDetail={() => setDetail(item)}
                          />
                        ))}
                    </section>
                  ))}
                </div>
                <div className="sticky-actions">
                  <span>
                    {selected.length} {t('selected').toLocaleLowerCase()}
                  </span>
                  <button
                    className="button primary"
                    disabled={!ready || !selected.length || busy}
                    onClick={() => void prepare('apply', selected)}
                  >
                    {t('reviewSelection')}
                    <Icon name="arrowRight" />
                  </button>
                </div>
              </>
            )}
            {page === 'apps' && (
              <>
                <p className="intro">{t('appsIntro')}</p>
                <div className="notice warning">
                  <strong>{t('noUndo')}</strong>
                  <p>{t('noUndoDetail')}</p>
                </div>
                <p className="muted small">{t('scopeNote')}</p>
                {!ready ? (
                  <section className="empty-state">
                    <Icon name="apps" />
                    <p>{t('notInstalled')}</p>
                    <button
                      className="button primary"
                      disabled={scanning || !data}
                      onClick={() => void analyze()}
                    >
                      {t('analyze')}
                    </button>
                  </section>
                ) : apps.length === 0 ? (
                  <p className="empty-state">{t('noApps')}</p>
                ) : (
                  <div className="app-grid">
                    {apps.map((item) => (
                      <ItemRow
                        key={item.id}
                        item={item}
                        language={language}
                        observation={rows.get(item.id)}
                        checked={appIds.includes(item.id)}
                        disabled={busy}
                        blocked={false}
                        incompatible={false}
                        onToggle={() => toggle(item.id, true)}
                        onDetail={() => setDetail(item)}
                      />
                    ))}
                  </div>
                )}
                <div className="sticky-actions">
                  <span>
                    {appIds.length} {t('selected').toLocaleLowerCase()}
                  </span>
                  <button
                    className="button primary"
                    disabled={!ready || !appIds.length || busy}
                    onClick={() => void prepare('remove_apps', appIds)}
                  >
                    {t('reviewApps')}
                    <Icon name="arrowRight" />
                  </button>
                </div>
              </>
            )}
            {page === 'review' &&
              (review ? (
                <ReviewScreen
                  key={review.ticket}
                  review={review}
                  language={language}
                  busy={busy}
                  onSubmit={(approval) => void execute(approval)}
                  onBack={() => {
                    setReview(null)
                    setPage(review.kind === 'remove_apps' ? 'apps' : 'items')
                  }}
                />
              ) : (
                <p className="empty-state">{t('reviewRequired')}</p>
              ))}
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
                        className="button"
                        disabled={!activeHistory.length || busy}
                        onClick={() => void prepare('undo_all', [])}
                      >
                        {t('undoAll')}
                      </button>
                    </div>
                    {!history.journal?.entries.length ? (
                      <p className="empty-state">{t('emptyHistory')}</p>
                    ) : (
                      <ul className="history-list">
                        {history.journal.entries.map((entry) => (
                          <li key={entry.itemId}>
                            <div>
                              <strong>
                                {data?.items.find((item) => item.id === entry.itemId)?.name[
                                  language
                                ] ?? entry.itemId}
                              </strong>
                              <time dateTime={entry.appliedAt}>
                                {new Date(entry.appliedAt).toLocaleString(language)}
                              </time>
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
                      {history.apps.map((app, index) => (
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
            {page === 'benchmark' && <Measurements backend={backend} language={language} />}
            {page === 'games' && (
              <GamesPanel
                backend={backend}
                language={language}
                compatible={data?.compatible ?? {}}
              />
            )}
            {page === 'settings' && (
              <section className="surface settings-panel glass">
                <GamesPanel backend={backend} language={language} settings />
                <div className="setting-row">
                  <label htmlFor="language">{t('language')}</label>
                  <select
                    id="language"
                    value={language}
                    onChange={(e) => setLanguage(e.target.value as Language)}
                  >
                    <option value="pt-BR">{t('ptName')}</option>
                    <option value="en">{t('enName')}</option>
                  </select>
                </div>
                <div className="setting-row">
                  <span>{t('theme')}</span>
                  <Segmented
                    options={(['light', 'dark'] as const).map((value) => ({
                      value,
                      label: t(value),
                    }))}
                    value={theme}
                    onChange={setTheme}
                    label={t('theme')}
                  />
                </div>
                <div className="setting-row">
                  <div>
                    <span>{t('effects')}</span>
                    <p>{t('effectsNote')}</p>
                  </div>
                  <Segmented
                    options={(['full', 'reduced'] as const).map((value) => ({
                      value,
                      label: t(`effects_${value}`),
                    }))}
                    value={effects}
                    onChange={setEffects}
                    label={t('effects')}
                  />
                </div>
                <div className="setting-row">
                  <div>
                    <label htmlFor="advanced">{t('advanced')}</label>
                    <p className="muted small">{t('advancedNote')}</p>
                  </div>
                  <Switch
                    id="advanced"
                    label={t('advanced')}
                    checked={advanced}
                    onChange={setAdvanced}
                  />
                </div>
                <div className="setting-row">
                  <div>
                    <h2>{t('privacy')}</h2>
                    <p>{t('privacyNote')}</p>
                  </div>
                  <span className="pill">{t('offline')}</span>
                </div>
                <p className="muted small">{t('localPrefs')}</p>
              </section>
            )}
            <footer className="content-footer">
              <span>
                {t('brand')} <span className="mono">{version}</span>
              </span>
              <span>
                {t('offline')} · {t('localOnly')}
              </span>
            </footer>
          </div>
        </main>
      </div>
      {detail && (
        <Details
          item={detail}
          language={language}
          observation={rows.get(detail.id)}
          advanced={advanced}
          onClose={() => setDetail(null)}
        />
      )}
    </div>
  )
}

function ItemRow({
  item,
  language,
  observation,
  checked,
  disabled,
  blocked,
  incompatible,
  onToggle,
  onDetail,
}: {
  item: Item
  language: Language
  observation: Observation | undefined
  checked: boolean
  disabled: boolean
  blocked: boolean
  incompatible: boolean
  onToggle: () => void
  onDetail: () => void
}) {
  const t = translator(language),
    name = item.name[language]
  return (
    <div className={`item-row ${checked ? 'selected-row' : ''}`}>
      <div className="item-select">
        {item.type === 'appx' ? (
          <label className="checkbox-target">
            <Checkbox checked={checked} disabled={disabled} onChange={onToggle} aria-label={name} />
          </label>
        ) : (
          <Switch checked={checked} disabled={disabled} onChange={onToggle} label={name} />
        )}
        <span>
          <strong>{name}</strong>
          <small>{item.description[language]}</small>
        </span>
      </div>
      <div className="item-badges">
        <span className="level-badge">
          {t('level')} {item.level}
        </span>
        <span className={`risk-badge ${item.risk}`}>
          <Icon name={item.risk === 'high' ? 'high' : item.risk === 'low' ? 'warning' : 'safe'} />
          {t(item.risk === 'high' ? 'riskHigh' : item.risk === 'low' ? 'riskLow' : 'riskNone')}
        </span>
        {item.type === 'appx' && (
          <span className="badge badge--warn">
            <Icon name="apps" />
            {t('noUndo')}
          </span>
        )}
        {!item.verified && <span className="pill">{t('unverified')}</span>}
        {blocked && <span className="pill">{t('blocked')}</span>}
        {incompatible && <span className="pill">{t('incompatible')}</span>}
        <span className="state-label">{observation ? t(observation.presence) : '—'}</span>
        <button
          className="detail-button"
          aria-label={`${t('details')}: ${name}`}
          onClick={onDetail}
        >
          <Icon name="arrowUp" />
        </button>
      </div>
    </div>
  )
}
function Details({
  item,
  language,
  observation,
  advanced,
  onClose,
}: {
  item: Item
  language: Language
  observation: Observation | undefined
  advanced: boolean
  onClose: () => void
}) {
  const ref = useRef<HTMLDialogElement>(null),
    t = translator(language)
  useEffect(() => {
    const previous = document.activeElement
    ref.current?.showModal()
    return () => {
      if (previous instanceof HTMLElement) previous.focus()
    }
  }, [])
  return (
    <dialog
      className="detail-dialog"
      ref={ref}
      aria-labelledby="detail-title"
      onCancel={onClose}
      onClose={onClose}
    >
      <div className="section-top">
        <span className="eyebrow">{t('details')}</span>
        <button className="detail-button" aria-label={t('closeDetails')} onClick={onClose}>
          <Icon name="close" />
        </button>
      </div>
      <h2 id="detail-title">{item.name[language]}</h2>
      <h3>{t('what')}</h3>
      <p>{item.description[language]}</p>
      <h3>{t('risk')}</h3>
      <p>{item.riskNote[language]}</p>
      {item.requiresReboot && (
        <p className="notice warning">
          <Icon name="warning" />
          {t('reboot')}
        </p>
      )}
      <h3>{t('revert')}</h3>
      <p>{t(item.revert === 'auto' ? 'autoRevert' : 'manualRevert')}</p>
      <h3>{t('state')}</h3>
      <p>
        {observation ? t(observation.presence) : '—'}
        {observation?.currentDword !== null && observation?.currentDword !== undefined
          ? ` · ${observation.currentDword}`
          : ''}
      </p>
      {advanced && (
        <>
          <h3>{t('technical')}</h3>
          <pre>{JSON.stringify(observation?.current ?? item.target, null, 2)}</pre>
        </>
      )}
      <h3>{t('sources')}</h3>
      <ul className="sources">
        {item.sources.map((source) => (
          <li key={source}>{source}</li>
        ))}
      </ul>
    </dialog>
  )
}
function ReviewScreen({
  review,
  language,
  busy,
  onSubmit,
  onBack,
}: {
  review: Review
  language: Language
  busy: boolean
  onSubmit: (approval: Approval) => void
  onBack: () => void
}) {
  const t = translator(language),
    [confirmed, setConfirmed] = useState(false),
    [acks, setAcks] = useState<Record<string, boolean>>({})
  const removal = review.kind === 'remove_apps'
  const required = review.items.flatMap((item) => [
    ...(removal ? [`undo:${item.id}`] : []),
    ...(item.risk === 'high' && (removal || review.kind === 'apply') ? [`risk:${item.id}`] : []),
    ...(!item.verified && (removal || review.kind === 'apply') ? [`unverified:${item.id}`] : []),
  ])
  const ready = confirmed && required.every((id) => acks[id])
  const action =
    review.kind === 'apply'
      ? 'apply'
      : removal
        ? 'remove'
        : review.kind === 'history'
          ? 'authorizeHistory'
          : review.kind === 'reinstall'
            ? 'reinstall'
            : 'undo'
  function ack(id: string, label: string) {
    const itemId = id.slice(id.indexOf(':') + 1)
    return (
      <label className="consent" key={id}>
        <Checkbox
          checked={acks[id] ?? false}
          aria-describedby={`review-heading-${itemId} review-risk-${itemId}`}
          disabled={busy}
          onChange={(e) => setAcks((old) => ({ ...old, [id]: e.target.checked }))}
        />
        <span>{label}</span>
      </label>
    )
  }
  function submit() {
    onSubmit({
      ticket: review.ticket,
      confirm: confirmed,
      highRiskIds: review.items.filter((item) => acks[`risk:${item.id}`]).map((item) => item.id),
      unverifiedIds: review.items
        .filter((item) => acks[`unverified:${item.id}`])
        .map((item) => item.id),
      apps: removal
        ? review.items.map((item) => ({
            itemId: item.id,
            noUndo: !!acks[`undo:${item.id}`],
            gamingRisk: !!acks[`risk:${item.id}`],
            unverified: !!acks[`unverified:${item.id}`],
          }))
        : [],
    })
  }
  return (
    <>
      <p className="intro">{t('reviewIntro')}</p>
      {removal && (
        <div className="notice danger">
          <strong>{t('noUndo')}</strong>
          <p>{t('noUndoDetail')}</p>
          <p>{t('scopeNote')}</p>
        </div>
      )}
      {review.kind === 'history' && <p>{t('historyReview')}</p>}
      {review.kind === 'reinstall' && <p>{t('reinstallReview')}</p>}
      {(review.kind === 'undo' || review.kind === 'undo_all') && <p>{t('undoReview')}</p>}
      <section className="surface review-summary glass" aria-label={t('byType')}>
        <dl className="review-metrics">
          {Object.entries(review.counts).map(([kind, count]) => (
            <div key={kind}>
              <dt>
                <Icon
                  name={
                    kind === 'appx'
                      ? 'apps'
                      : kind === 'service'
                        ? 'service'
                        : kind === 'scheduled_task'
                          ? 'task'
                          : kind === 'power'
                            ? 'power'
                            : kind === 'startup'
                              ? 'startup'
                              : 'registry'
                  }
                />
                {t(kind)}
              </dt>
              <dd>{count}</dd>
            </div>
          ))}
        </dl>
      </section>
      <div className="review-timeline">
        {(['none', 'low', 'high'] as const)
          .filter((risk) => review.items.some((item) => item.risk === risk))
          .map((risk) => (
            <section
              className={`review-risk-group ${risk}`}
              key={risk}
              aria-label={t(risk === 'high' ? 'riskHigh' : risk === 'low' ? 'riskLow' : 'riskNone')}
            >
              <h2 className="risk-group-heading">
                <Icon name={risk === 'high' ? 'high' : 'safe'} />
                {t(risk === 'high' ? 'riskHigh' : risk === 'low' ? 'riskLow' : 'riskNone')}
                <span className="num">
                  {review.items.filter((item) => item.risk === risk).length}
                </span>
              </h2>
              {review.items
                .filter((item) => item.risk === risk)
                .map((item) => (
                  <section className="review-item" key={item.id}>
                    <span className="timeline-marker">
                      <Icon
                        name={
                          item.type === 'appx'
                            ? 'apps'
                            : item.type === 'power'
                              ? 'power'
                              : item.type === 'service'
                                ? 'service'
                                : item.type === 'scheduled_task'
                                  ? 'task'
                                  : item.type === 'startup'
                                    ? 'startup'
                                    : 'registry'
                        }
                      />
                    </span>
                    <div className="section-top">
                      <h2 id={`review-heading-${item.id}`}>{item.name[language]}</h2>
                    </div>
                    <p id={`review-risk-${item.id}`}>{item.riskNote[language]}</p>
                    {item.requiresReboot && (
                      <p className="notice warning">
                        <Icon name="warning" />
                        {t('reboot')}
                      </p>
                    )}
                    {removal && ack(`undo:${item.id}`, t('ackNoUndo'))}
                    {item.risk === 'high' &&
                      (removal || review.kind === 'apply') &&
                      ack(`risk:${item.id}`, t('ackRisk'))}
                    {!item.verified &&
                      (removal || review.kind === 'apply') &&
                      ack(`unverified:${item.id}`, t('ackUnverified'))}
                  </section>
                ))}
            </section>
          ))}
      </div>
      <div className="final-consent">
        <label className="consent">
          <Checkbox
            checked={confirmed}
            disabled={busy}
            onChange={(e) => setConfirmed(e.target.checked)}
          />
          <strong>{t('ackAll')}</strong>
        </label>
        <p>{t('uac')}</p>
        <div className="actions">
          <button
            className={`button ${removal ? 'destructive' : 'primary'}`}
            disabled={!ready || busy}
            onClick={submit}
          >
            {busy ? t('executing') : t(action)}
          </button>
          <button className="button subtle" disabled={busy} onClick={onBack}>
            {t('back')}
          </button>
        </div>
      </div>
    </>
  )
}
function OperationResult({
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
  const record = (value: unknown): Record<string, unknown> | null =>
    value !== null && typeof value === 'object' && !Array.isArray(value)
      ? (value as Record<string, unknown>)
      : null
  const operation = record(reply.result?.operation),
    report = record(operation?.report)
  const candidate = operation?.items ?? report?.items ?? operation?.apps
  const rows = Array.isArray(candidate) ? candidate.map(record).filter((row) => row !== null) : []
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
