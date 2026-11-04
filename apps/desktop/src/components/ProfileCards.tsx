import { useRef, type KeyboardEvent } from 'react'
import { translator } from '../i18n'
import { plural } from '../i18n-format'
import type { Language } from '../types'
import { Icon } from './Icon'
const profiles = ['safe', 'balanced', 'aggressive', 'custom'] as const
export function ProfileCards({
  value,
  onChange,
  counts,
  language,
  disabled,
}: {
  value: string
  onChange: (value: string) => void
  counts: Record<string, string[]>
  language: Language
  disabled: boolean
}) {
  const t = translator(language),
    refs = useRef<(HTMLButtonElement | null)[]>([])
  function move(event: KeyboardEvent, index: number) {
    const next =
      event.key === 'Home'
        ? 0
        : event.key === 'End'
          ? 3
          : ['ArrowRight', 'ArrowDown'].includes(event.key)
            ? (index + 1) % 4
            : ['ArrowLeft', 'ArrowUp'].includes(event.key)
              ? (index + 3) % 4
              : -1
    if (next < 0) return
    event.preventDefault()
    onChange(profiles[next])
    refs.current[next]?.focus()
  }
  return (
    <div className="profile-cards" role="radiogroup" aria-label={t('profile')}>
      {profiles.map((profile, index) => (
        <button
          key={profile}
          type="button"
          role="radio"
          aria-label={t(profile)}
          aria-checked={value === profile}
          tabIndex={value === profile ? 0 : -1}
          disabled={disabled}
          ref={(element) => {
            refs.current[index] = element
          }}
          onKeyDown={(event) => move(event, index)}
          onClick={() => onChange(profile)}
          className="profile-card"
        >
          <span className="profile-card__title">
            <Icon
              name={profile === 'aggressive' ? 'warning' : profile === 'custom' ? 'items' : 'safe'}
            />
            <strong>{t(profile)}</strong>
          </span>
          <span className="num">
            {counts[profile]
              ? plural(language, 'profileCount', counts[profile].length)
              : t('loadingData')}
          </span>
          <span>{t(`${profile}Note`)}</span>
          <small>{t(`profileRisk_${profile}`)}</small>
        </button>
      ))}
    </div>
  )
}
