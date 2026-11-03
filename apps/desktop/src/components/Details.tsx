import { useEffect, useRef } from 'react'
import type { Item, Language, Observation } from '../types'
import { translator } from '../i18n'
import { Icon } from './Icon'
export function Details({
  item,
  language,
  observation,
  advanced,
  onClose,
}: {
  item: Item
  language: Language
  observation: Observation | undefined
  advanced: boolean
  onClose: () => void
}) {
  const ref = useRef<HTMLDialogElement>(null),
    t = translator(language)
  useEffect(() => {
    const previous = document.activeElement
    ref.current?.showModal()
    return () => {
      if (previous instanceof HTMLElement) previous.focus()
    }
  }, [])
  return (
    <dialog
      className="detail-dialog"
      ref={ref}
      aria-labelledby="detail-title"
      onCancel={onClose}
      onClose={onClose}
    >
      <div className="section-top">
        <span className="eyebrow">{t('details')}</span>
        <button className="detail-button" aria-label={t('closeDetails')} onClick={onClose}>
          <Icon name="close" />
        </button>
      </div>
      <h2 id="detail-title">{item.name[language]}</h2>
      <h3>{t('what')}</h3>
      <p>{item.description[language]}</p>
      <h3>{t('risk')}</h3>
      <p>{item.riskNote[language]}</p>
      {item.requiresReboot && (
        <p className="notice warning">
          <Icon name="warning" />
          {t('reboot')}
        </p>
      )}
      <h3>{t('revert')}</h3>
      <p>{t(item.revert === 'auto' ? 'autoRevert' : 'manualRevert')}</p>
      <h3>{t('state')}</h3>
      <p>
        {observation ? t(observation.presence) : '—'}
        {observation?.currentDword !== null && observation?.currentDword !== undefined
          ? ` · ${observation.currentDword}`
          : ''}
      </p>
      {advanced && (
        <>
          <h3>{t('technical')}</h3>
          <pre>{JSON.stringify(observation?.current ?? item.target, null, 2)}</pre>
        </>
      )}
      <h3>{t('sources')}</h3>
      <ul className="sources">
        {item.sources.map((source) => (
          <li key={source}>{source}</li>
        ))}
      </ul>
    </dialog>
  )
}
