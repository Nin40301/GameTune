import { useEffect, useRef, useState } from 'react'
import { useApp } from '../state/AppContext'
import { BrandMark } from '../components/Icon'
import { Button } from '../components/Button'
import { Segmented } from '../components/Segmented'
import { InlineStatus } from '../components/InlineStatus'
export function Onboarding() {
  const { t, language, setLanguage, backend, data, profile, chooseProfile, finishOnboarding } =
    useApp()
  const [step, setStep] = useState(0),
    [error, setError] = useState(false)
  const title = useRef<HTMLHeadingElement>(null)
  useEffect(() => title.current?.focus(), [step])
  const open = async () => {
    try {
      await backend.support({ kind: 'open', tool: 'protection' })
      setError(false)
    } catch {
      setError(true)
    }
  }
  return (
    <main className="onboarding app-bg" id="main">
      <section className="glass glass--strong glass--pad">
        <BrandMark />
        <ol className="onboarding-steps" aria-label={t('onboardingProgress')}>
          {['onboardingScope', 'systemProtection', 'profile'].map((key, index) => (
            <li key={key} aria-current={index === step ? 'step' : undefined}>
              {new Intl.NumberFormat(language).format(index + 1)}
              <span>{t(key)}</span>
            </li>
          ))}
        </ol>
        <h1 ref={title} tabIndex={-1}>
          {t(`onboardingTitle${step}`)}
        </h1>
        {step === 0 && (
          <>
            <p>{t('onboardingDoes')}</p>
            <p>{t('onboardingDoesNot')}</p>
            <Segmented
              label={t('language')}
              options={[
                { value: 'pt-BR', label: t('ptName') },
                { value: 'en', label: t('enName') },
              ]}
              value={language}
              onChange={setLanguage}
            />
          </>
        )}
        {step === 1 && (
          <>
            <p>{t(`protection_${data?.system.protection ?? 'unknown'}`)}</p>
            <p>{t('protectionWhy')}</p>
            <Button variant="glass" onClick={() => void open()}>
              {t('openProtection')}
            </Button>
            <p>{t('onboardingReadOnly')}</p>
          </>
        )}
        {step === 2 && (
          <>
            <p>{t('onboardingProfile')}</p>
            <Segmented
              label={t('profile')}
              options={['safe', 'balanced', 'aggressive', 'custom'].map((value) => ({
                value,
                label: t(value),
              }))}
              value={profile}
              onChange={chooseProfile}
            />
            <p>{t(`${profile}Note`)}</p>
          </>
        )}
        {error && (
          <InlineStatus error next={t('retry')} onNext={() => void open()}>
            {t('operation_failed')}
          </InlineStatus>
        )}
        <div className="actions">
          <Button
            disabled={!data}
            onClick={() => (step < 2 ? setStep(step + 1) : finishOnboarding())}
          >
            {t(step < 2 ? 'continue' : 'beginSimulation')}
          </Button>
          {step > 0 && (
            <Button variant="ghost" onClick={() => setStep(step - 1)}>
              {t('backStep')}
            </Button>
          )}
        </div>
      </section>
    </main>
  )
}
