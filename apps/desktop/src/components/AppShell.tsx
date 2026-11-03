import { StatusStrip } from './StatusStrip'
import { number } from '../i18n-format'
import { Onboarding } from '../pages/Onboarding'
import { ResultsPage } from '../pages/ResultsPage'
import { InlineStatus } from './InlineStatus'
import { ScanStatus } from './ScanStatus'
import { useApp } from '../state/AppContext'
import { navigation, primaryNavigation } from '../state/navigation'
import type { Page } from '../types'
import { version } from '../../package.json'
import { BrandMark, Icon } from '../components/Icon'
import { Button } from '../components/Button'
import { Details } from '../components/Details'
import { LegacyGuidance } from '../components/SupportPanel'
import { HomePage } from '../pages/HomePage'
import { ItemsPage } from '../pages/ItemsPage'
import { AppsPage } from '../pages/AppsPage'
import { ReviewPage } from '../pages/ReviewPage'
import { HistoryPage } from '../pages/HistoryPage'
import { MeasurementsPage } from '../pages/MeasurementsPage'
import { GamesPage } from '../pages/GamesPage'
import { SettingsPage } from '../pages/SettingsPage'
function CurrentPage() {
  const { page } = useApp()
  switch (page) {
    case 'results':
      return <ResultsPage />

    case 'home':
      return <HomePage />
    case 'items':
      return <ItemsPage />
    case 'apps':
      return <AppsPage />
    case 'review':
      return <ReviewPage />
    case 'history':
      return <HistoryPage />
    case 'benchmark':
      return <MeasurementsPage />
    case 'games':
      return <GamesPage />
    case 'settings':
      return <SettingsPage />
  }
}
export function AppShell() {
  const {
    t,
    page,
    busy,
    review,
    navigate,
    selected,
    compactNavigation,
    moreMenu,
    title,
    heading,
    data,
    scanning,
    analyze,
    ready,
    error,
    gameRecovery,
    setGameRecovery,
    setPage,
    analysis,
    advanced,
    language,
    detail,
    setDetail,
    rows,
    onboarded,
  } = useApp()
  const navPage = page === 'results' ? 'history' : page
  function navigationButton(item: Page) {
    return (
      <button
        type="button"
        aria-label={t(item)}
        aria-current={navPage === item ? 'page' : undefined}
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
          <span className="nav-count">{number(language, selected.length)}</span>
        )}
      </button>
    )
  }
  if (!onboarded) return <Onboarding />
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
        <nav className="nav" data-index={navigation.indexOf(navPage)} aria-label={t('nav')}>
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
                  <summary
                    aria-label={t('more')}
                    data-active={!primaryNavigation.includes(navPage)}
                  >
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
          <span className="version">
            {t('versionPrefix')}
            {version}
          </span>
        </div>
      </aside>
      <div className="workspace">
        <StatusStrip />
        <main id="main" tabIndex={-1} aria-busy={busy}>
          <div key={page} className={`page page--${page}`}>
            {import.meta.env.MODE === 'test' && <div className="test-mode">{t('testMode')}</div>}
            <div className="page-heading">
              <div>
                <h1 ref={heading} tabIndex={-1}>
                  {title}
                </h1>
                <span className="sr-only" role="status" aria-live="polite">
                  {title}
                </span>
              </div>
              {page === 'items' && (
                <div className="heading-actions">
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
            {data?.system.legacyData && <LegacyGuidance language={language} />}
            {error && (
              <InlineStatus error next={t('retryAnalysis')} onNext={() => void analyze()}>
                {t(error)}
              </InlineStatus>
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
            <ScanStatus />
            {ready &&
              ['home', 'items', 'apps'].includes(page) &&
              analysis.observations.some((row) => row.error) && (
                <p className="notice warning" role="status">
                  {t('readError')}
                </p>
              )}
            <CurrentPage />
            <footer className="content-footer">
              <span>
                {t('brand')} <span className="mono">{version}</span>
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
