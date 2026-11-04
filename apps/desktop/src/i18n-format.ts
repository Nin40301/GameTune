import type { Language } from './types'
import { translator } from './i18n'
export function number(language: Language, value: number, digits = 0) {
  return new Intl.NumberFormat(language, { maximumFractionDigits: digits }).format(value)
}
export function date(language: Language, value: string) {
  const date = new Date(value)
  return Number.isNaN(date.getTime())
    ? translator(language)('unavailableValue')
    : new Intl.DateTimeFormat(language, { dateStyle: 'short', timeStyle: 'short' }).format(date)
}
export function plural(language: Language, key: string, value: number) {
  // Product voice uses plural for a zero count; other quantities follow Intl.
  const category = value === 0 ? 'other' : new Intl.PluralRules(language).select(value)
  return translator(language)(`${key}_${category}`).replace('{count}', number(language, value))
}

export function day(language: Language, value: string) {
  const parsed = new Date(value)
  return Number.isNaN(parsed.getTime())
    ? translator(language)('noHistoryDay')
    : new Intl.DateTimeFormat(language, { dateStyle: 'full' }).format(parsed)
}
