import { useEffect, useState } from 'react'
import type { Language, Theme } from '../types'
import { persist, preference } from '../state/preferences'
export function initialLanguage(): Language {
  return navigator.language.toLowerCase().startsWith('pt') ? 'pt-BR' : 'en'
}
export function usePreferences() {
  const [language, setLanguage] = useState<Language>(() =>
    preference('language', initialLanguage()) === 'pt-BR' ? 'pt-BR' : 'en',
  )
  const [theme, setTheme] = useState<Theme>(() =>
    preference('theme', 'light') === 'dark' ? 'dark' : 'light',
  )
  const [effects, setEffects] = useState<'full' | 'reduced'>(() => {
    const stored = preference('effects', '')
    if (stored === 'full' || stored === 'reduced') return stored
    return typeof window.matchMedia === 'function' &&
      window.matchMedia('(prefers-reduced-transparency: reduce)').matches
      ? 'reduced'
      : 'full'
  })
  const [advanced, setAdvanced] = useState(() => preference('advanced', 'false') === 'true')
  const [onboarded, setOnboarded] = useState(() => preference('onboarding', '') === 'done')
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
  const finishOnboarding = () => {
    persist('onboarding', 'done')
    setOnboarded(true)
  }
  return {
    language,
    setLanguage,
    theme,
    setTheme,
    effects,
    setEffects,
    advanced,
    setAdvanced,
    onboarded,
    finishOnboarding,
  }
}
