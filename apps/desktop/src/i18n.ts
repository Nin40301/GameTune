import pt from './locales/pt-BR.json'
import en from './locales/en.json'
import type { Language } from './types'
export type Key = keyof typeof pt
export function translator(language: Language) {
  const messages: Record<string, string> = language === 'en' ? en : pt
  return (key: string) => messages[key] ?? messages.operation_failed ?? key
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
  return messages[`itemName_${item.id}`] ?? item.name[language]
}
