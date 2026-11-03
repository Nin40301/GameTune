import { useState } from 'react'
import { usePreferences } from '../hooks/usePreferences'
import { translator } from '../i18n'
import { Button } from '../components/Button'
import { Segmented } from '../components/Segmented'
import { Switch } from '../components/Switch'
import { Checkbox } from '../components/Checkbox'
import { StatusOrb } from '../components/StatusOrb'
import { Icon } from '../components/Icon'
export default function DesignSystem() {
  const { language, theme, setTheme } = usePreferences(),
    t = translator(language)
  const [checked, setChecked] = useState(false)
  return (
    <main className="design-system app-bg" data-development="AMETISTA_DESIGN_SYSTEM_DEV_ONLY">
      <h1>{t('designSystem')}</h1>
      <p>{t('designOnly')}</p>
      <Segmented
        label={t('theme')}
        value={theme}
        onChange={setTheme}
        options={(['light', 'dark'] as const).map((value) => ({ value, label: t(value) }))}
      />
      <section className="surface">
        <h2>{t('tokens')}</h2>
        <div className="token-swatches">
          {['bg-base', 'ink', 'ink-soft', 'violet-700', 'ok', 'warn', 'danger'].map((token) => (
            <div key={token} className={`swatch swatch--${token}`}>
              <span>{token}</span>
            </div>
          ))}
        </div>
      </section>
      <section className="surface">
        <h2>{t('controls')}</h2>
        <div className="design-states">
          {(['default', 'hover', 'focus', 'active', 'disabled', 'loading', 'error'] as const).map(
            (state) => (
              <div key={state}>
                <h3>{t(`state_${state}`)}</h3>
                <Button
                  className={`state-${state}`}
                  disabled={state === 'disabled'}
                  aria-busy={state === 'loading'}
                  variant={state === 'error' ? 'danger' : 'primary'}
                  icon={<Icon name={state === 'error' ? 'warning' : 'simulate'} />}
                >
                  {t(state === 'loading' ? 'analyzing' : 'simulate')}
                </Button>
                <label className="check-line">
                  <Checkbox
                    checked={checked}
                    disabled={state === 'disabled'}
                    onChange={(event) => setChecked(event.target.checked)}
                  />
                  {t('ackAll')}
                </label>
                <Switch
                  checked={checked}
                  disabled={state === 'disabled'}
                  onChange={setChecked}
                  label={t('enableProfile')}
                />
                <label className="design-input">
                  {t('search')}
                  <input
                    type="search"
                    disabled={state === 'disabled'}
                    aria-invalid={state === 'error'}
                    placeholder={t('searchPlaceholder')}
                  />
                </label>
                <Segmented
                  label={t('profile')}
                  value={checked ? 'balanced' : 'safe'}
                  disabled={state === 'disabled'}
                  onChange={(value) => setChecked(value === 'balanced')}
                  options={['safe', 'balanced'].map((value) => ({ value, label: t(value) }))}
                />
              </div>
            ),
          )}
        </div>
        <Switch checked={checked} onChange={setChecked} label={t('enableProfile')} />
        <label className="check-line">
          <Checkbox checked={checked} onChange={(e) => setChecked(e.target.checked)} />
          {t('ackAll')}
        </label>
        <Segmented
          label={t('profile')}
          value={checked ? 'balanced' : 'safe'}
          onChange={(v) => setChecked(v === 'balanced')}
          options={['safe', 'balanced'].map((value) => ({ value, label: t(value) }))}
        />
      </section>
      <section className="surface">
        <h2>{t('feedback')}</h2>
        <StatusOrb
          value={0.4}
          title={t('designOrbValue')}
          caption={t('designOrbCaption')}
          compact
        />
        <p className="notice success" role="status">
          {t('operationDone')}
        </p>
        <p className="notice danger">{t('operationError')}</p>
        <progress value={40} max={100} aria-label={t('progress')} />
      </section>
    </main>
  )
}
