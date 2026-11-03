import { SecurityCard } from '../components/SecurityCard'
import { useApp } from '../state/AppContext'
import { Measurements } from './Measurements'
import { ProfileCards } from '../components/ProfileCards'
import { Button } from '../components/Button'
import { Icon } from '../components/Icon'
export function HomePage() {
  const {
    page,
    t,
    backend,
    language,
    selected,
    profile,
    chooseProfile,
    busy,
    data,
    scanning,
    analyze,
    ready,
    prepare,
    setPage,
  } = useApp()
  return (
    <>
      {page === 'home' && (
        <>
          <p className="intro">{t('homeIntro')}</p>
          <Measurements
            backend={backend}
            language={language}
            dashboard
            safetyCard={<SecurityCard />}
            profileControls={
              <>
                <div className="section-top">
                  <h2>{t('profile')}</h2>
                  <span className="pill accent">
                    <Icon name="safe" />
                    {t('simulated')}
                  </span>
                </div>
                <ProfileCards
                  value={profile}
                  onChange={chooseProfile}
                  counts={{ ...data?.presets, custom: selected }}
                  language={language}
                  disabled={busy}
                />
                {profile === 'aggressive' && (
                  <p className="notice warning">
                    <Icon name="warning" />
                    {t('aggressiveCaution')}
                  </p>
                )}
                <div className="actions">
                  <Button
                    icon={<Icon name="simulate" />}
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
    </>
  )
}
