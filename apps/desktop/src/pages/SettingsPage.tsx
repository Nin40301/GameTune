import { useApp } from '../state/AppContext'
import type { Language } from '../types'
import { SupportPanel } from '../components/SupportPanel'
import { GamesPanel } from './GamesPanel'
import { Segmented } from '../components/Segmented'
import { Switch } from '../components/Switch'
export function SettingsPage() {
  const {
    page,
    backend,
    language,
    data,
    t,
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
  } = useApp()
  return (
    <>
      {page === 'settings' && (
        <section className="surface settings-panel glass">
          <SupportPanel backend={backend} language={language} system={data?.system} />
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
              options={(['system', 'light', 'dark'] as const).map((value) => ({
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
              options={(['full', 'reduced', 'auto'] as const).map((value) => ({
                value,
                label: t(`effects_${value}`),
              }))}
              value={effects}
              onChange={setEffects}
              label={t('effects')}
            />
          </div>
          <div className="setting-row">
            <label htmlFor="text-size">
              {t('textSize')}{' '}
              <output>
                {new Intl.NumberFormat(language, { style: 'percent' }).format(textSize / 100)}
              </output>
            </label>
            <input
              id="text-size"
              type="range"
              min={90}
              max={130}
              step={10}
              value={textSize}
              onChange={(e) => setTextSize(Number(e.target.value))}
            />
          </div>
          <div className="setting-row">
            <span>{t('motion')}</span>
            <Segmented
              label={t('motion')}
              value={motion}
              onChange={setMotion}
              options={(['system', 'reduced'] as const).map((value) => ({
                value,
                label: t(`motion_${value}`),
              }))}
            />
          </div>
          <div className="setting-row">
            <div>
              <label htmlFor="advanced">{t('advanced')}</label>
              <p className="muted small">{t('advancedNote')}</p>
            </div>
            <Switch id="advanced" label={t('advanced')} checked={advanced} onChange={setAdvanced} />
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
    </>
  )
}
