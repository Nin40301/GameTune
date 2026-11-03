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
  const [theme, setTheme] = useState<Theme>(
    () =>
      (['system', 'dark', 'light'].includes(preference('theme', 'light'))
        ? preference('theme', 'light')
        : 'light') as Theme,
  )
  const [effects, setEffects] = useState<'full' | 'reduced' | 'auto'>(() => {
    const stored = preference('effects', '')
    if (stored === 'full' || stored === 'reduced' || stored === 'auto') return stored
    return typeof window.matchMedia === 'function' &&
      window.matchMedia('(prefers-reduced-transparency: reduce)').matches
      ? 'reduced'
      : 'full'
  })
  const [textSize, setTextSize] = useState(
    () =>
      Math.round(Math.min(130, Math.max(90, Number(preference('textSize', '100')) || 100)) / 10) *
      10,
  )
  const [motion, setMotion] = useState<'system' | 'reduced'>(() =>
    preference('motion', 'system') === 'reduced' ? 'reduced' : 'system',
  )
  const [advanced, setAdvanced] = useState(() => preference('advanced', 'false') === 'true')
  const [onboarded, setOnboarded] = useState(() => preference('onboarding', '') === 'done')
  useEffect(() => {
    document.documentElement.lang = language
    persist('language', language)
  }, [language])
  useEffect(() => {
    const fallback = { matches: false, addEventListener: () => {}, removeEventListener: () => {} }
    const dark = window.matchMedia?.('(prefers-color-scheme: dark)') ?? fallback
    const transparency = window.matchMedia?.('(prefers-reduced-transparency: reduce)') ?? fallback
    const resolve = () => {
      document.documentElement.dataset.theme =
        theme === 'system' ? (dark.matches ? 'dark' : 'light') : theme
      document.documentElement.dataset.effects =
        effects === 'auto' ? (transparency.matches ? 'reduced' : 'full') : effects
    }
    resolve()
    dark.addEventListener('change', resolve)
    transparency.addEventListener('change', resolve)
    persist('theme', theme)
    persist('effects', effects)
    return () => {
      dark.removeEventListener('change', resolve)
      transparency.removeEventListener('change', resolve)
    }
  }, [theme, effects])
  useEffect(() => {
    document.documentElement.dataset.textSize = String(textSize)
    persist('textSize', String(textSize))
  }, [textSize])
  useEffect(() => {
    document.documentElement.dataset.motion = motion
    persist('motion', motion)
  }, [motion])
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
    textSize,
    setTextSize,
    motion,
    setMotion,
    advanced,
    setAdvanced,
    onboarded,
    finishOnboarding,
  }
}
