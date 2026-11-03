import pt from './locales/pt-BR.json'
import en from './locales/en.json'
import type { Language } from './types'
declare const __TEST_BACKEND__: boolean
export type Key = keyof typeof pt
export function translator(language: Language) {
  const messages: Record<string, string> = language === 'en' ? en : pt
  return (key: string) => {
    const text = messages[key] ?? messages.operation_failed ?? key
    if (__TEST_BACKEND__ && localStorage.getItem('__ametistaPseudo') === 'true') return pseudo(text)
    return text
  }
}
export function categoryKey(category: string) {
  return category === 'apps' ? 'appCategory' : category
}

// Presentation only: IDs, descriptions and operation semantics stay intact.
export function itemName(
  item: { id: string; name: { 'pt-BR': string; en: string } },
  language: Language,
) {
  const messages: Record<string, string> = language === 'en' ? en : pt
  const text = messages[`itemName_${item.id}`] ?? item.name[language]
  return __TEST_BACKEND__ && localStorage.getItem('__ametistaPseudo') === 'true'
    ? pseudo(text)
    : text
}

export function pseudo(text: string) {
  const expanded = text
    .split(/(\{[^}]+\})/)
    .map((part) =>
      part.startsWith('{')
        ? part
        : part.replace(
            /[aeiouAEIOU]/g,
            (c) =>
              ({ a: 'á', e: 'ë', i: 'ï', o: 'ô', u: 'ü', A: 'Á', E: 'Ë', I: 'Ï', O: 'Ô', U: 'Ü' })[
                c
              ] ?? c,
          ),
    )
    .join('')
  return (
    '[ ' +
    expanded +
    ' ' +
    Array.from({ length: Math.ceil((text.length * 0.4) / 4) }, () => 'áááá').join(' ') +
    ' ]'
  )
}
