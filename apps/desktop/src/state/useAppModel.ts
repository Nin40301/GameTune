import { useEffect, useRef, useState, useMemo } from 'react'
import type {
  Analysis,
  Approval,
  Backend,
  Bootstrap,
  History,
  Item,
  Page,
  Reply,
  Review,
  ReviewKind,
} from '../types'
import { categoryKey, translator } from '../i18n'
import { useWindowActivity } from '../hooks/useWindowActivity'
import { usePreferences } from '../hooks/usePreferences'
import { errorKey } from './errors'
const emptyAnalysis: Analysis = {
  id: 0,
  status: 'idle',
  completed: 0,
  total: 0,
  itemId: '',
  observations: [],
  error: null,
}
export function useAppModel(backend: Backend) {
  const active = useWindowActivity()
  useEffect(() => {
    document.documentElement.dataset.windowActive = String(active)
  }, [active])
  const {
    language,
    setLanguage,
    theme,
    setTheme,
    effects,
    setEffects,
    textSize,
    setTextSize,
    motion,
    setMotion,
    advanced,
    setAdvanced,
    onboarded,
    finishOnboarding,
  } = usePreferences()
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
  const scanning = analysis.status === 'running' || analysis.status === 'cancelling'
  const ready = analysis.status === 'complete'
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
  const previousPage = useRef(page),
    previousOnboarding = useRef(onboarded)
  useEffect(() => {
    if (previousPage.current !== page || (!previousOnboarding.current && onboarded))
      heading.current?.focus()
    previousPage.current = page
    previousOnboarding.current = onboarded
  }, [page, onboarded])
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
  const rows = useMemo(
    () => new Map(analysis.observations.map((row) => [row.itemId, row])),
    [analysis.observations],
  )
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
    const kind = review?.kind
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
      setPage(kind === 'history' || kind === 'reinstall' ? 'history' : 'results')
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
  return {
    active,
    language,
    setLanguage,
    theme,
    setTheme,
    effects,
    setEffects,
    textSize,
    setTextSize,
    motion,
    setMotion,
    advanced,
    setAdvanced,
    onboarded,
    finishOnboarding,
    compactNavigation,
    moreMenu,
    gameRecovery,
    setGameRecovery,
    page,
    setPage,
    data,
    history,
    analysis,
    setAnalysis,
    profile,
    selected,
    appIds,
    search,
    setSearch,
    category,
    setCategory,
    detail,
    setDetail,
    review,
    setReview,
    error,
    setError,
    busy,
    reply,
    heading,
    t,
    scanning,
    ready,
    rows,
    name,
    categoryLabel,
    chooseProfile,
    toggle,
    analyze,
    prepare,
    execute,
    list,
    apps,
    title,
    canSelect,
    activeHistory,
    navigate,
    backend,
  }
}
export type AppModel = ReturnType<typeof useAppModel>
