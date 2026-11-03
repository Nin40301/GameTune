import { itemName } from '../i18n'
import type { Item, Language, Observation } from '../types'
import { translator, presentationText } from '../i18n'
import { Icon } from './Icon'
import { Checkbox } from './Checkbox'
import { Switch } from './Switch'
export function ItemRow({
  item,
  language,
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
    name = itemName(item, language)
  return (
    <div className={`item-row ${checked ? 'selected-row' : ''}`}>
      {item.type === 'appx' && (
        <span className="application-initials" aria-hidden="true">
          {name
            .replace(/^Microsoft\./, '')
            .slice(0, 2)
            .toLocaleUpperCase(language)}
        </span>
      )}
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
          <small>{presentationText(item.description[language])}</small>
        </span>
      </div>
      <div className="item-badges">
        <span className={`badge ${item.revert === 'auto' ? 'badge--ok' : 'badge--danger'}`}>
          <Icon name={item.revert === 'auto' ? 'history' : 'warning'} />
          {t(item.revert === 'auto' ? 'reversible' : 'notReversible')}
        </span>
        {item.risk === 'high' && (
          <span className="risk-badge high">
            <Icon name="high" />
            {t('riskHigh')}
          </span>
        )}
        {!item.verified && <span className="pill">{t('unverified')}</span>}
        {blocked && <span className="pill">{t('blocked')}</span>}
        {incompatible && <span className="pill">{t('incompatible')}</span>}
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
