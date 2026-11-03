import type { Item, Language, Observation } from '../types'
import { translator } from '../i18n'
import { Icon } from './Icon'
import { Checkbox } from './Checkbox'
import { Switch } from './Switch'
export function ItemRow({
  item,
  language,
  observation,
  checked,
  disabled,
  blocked,
  incompatible,
  onToggle,
  onDetail,
}: {
  item: Item
  language: Language
  observation: Observation | undefined
  checked: boolean
  disabled: boolean
  blocked: boolean
  incompatible: boolean
  onToggle: () => void
  onDetail: () => void
}) {
  const t = translator(language),
    name = item.name[language]
  return (
    <div className={`item-row ${checked ? 'selected-row' : ''}`}>
      <div className="item-select">
        {item.type === 'appx' ? (
          <label className="checkbox-target">
            <Checkbox checked={checked} disabled={disabled} onChange={onToggle} aria-label={name} />
          </label>
        ) : (
          <Switch checked={checked} disabled={disabled} onChange={onToggle} label={name} />
        )}
        <span>
          <strong>{name}</strong>
          <small>{item.description[language]}</small>
        </span>
      </div>
      <div className="item-badges">
        <span className="level-badge">
          {t('level')} {item.level}
        </span>
        <span className={`risk-badge ${item.risk}`}>
          <Icon name={item.risk === 'high' ? 'high' : item.risk === 'low' ? 'warning' : 'safe'} />
          {t(item.risk === 'high' ? 'riskHigh' : item.risk === 'low' ? 'riskLow' : 'riskNone')}
        </span>
        {item.type === 'appx' && (
          <span className="badge badge--warn">
            <Icon name="apps" />
            {t('noUndo')}
          </span>
        )}
        {!item.verified && <span className="pill">{t('unverified')}</span>}
        {blocked && <span className="pill">{t('blocked')}</span>}
        {incompatible && <span className="pill">{t('incompatible')}</span>}
        <span className="state-label">{observation ? t(observation.presence) : '—'}</span>
        <button
          className="detail-button"
          aria-label={`${t('details')}: ${name}`}
          onClick={onDetail}
        >
          <Icon name="arrowUp" />
        </button>
      </div>
    </div>
  )
}
