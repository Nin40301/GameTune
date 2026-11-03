import { itemName } from '../i18n'
import { useState } from 'react'
import type { Language, Review, Approval } from '../types'
import { translator } from '../i18n'
import { Icon } from './Icon'
import { Checkbox } from './Checkbox'
export function ReviewScreen({
  review,
  language,
  busy,
  onSubmit,
  onBack,
}: {
  review: Review
  language: Language
  busy: boolean
  onSubmit: (approval: Approval) => void
  onBack: () => void
}) {
  const t = translator(language),
    [confirmed, setConfirmed] = useState(false),
    [acks, setAcks] = useState<Record<string, boolean>>({})
  const removal = review.kind === 'remove_apps'
  const required = review.items.flatMap((item) => [
    ...(removal ? [`undo:${item.id}`] : []),
    ...(item.risk === 'high' && (removal || review.kind === 'apply') ? [`risk:${item.id}`] : []),
    ...(!item.verified && (removal || review.kind === 'apply') ? [`unverified:${item.id}`] : []),
  ])
  const ready = confirmed && required.every((id) => acks[id])
  const action =
    review.kind === 'apply'
      ? 'apply'
      : removal
        ? 'remove'
        : review.kind === 'history'
          ? 'authorizeHistory'
          : review.kind === 'reinstall'
            ? 'reinstall'
            : 'undo'
  function ack(id: string, label: string) {
    const itemId = id.slice(id.indexOf(':') + 1)
    return (
      <label className="consent" key={id}>
        <Checkbox
          checked={acks[id] ?? false}
          aria-describedby={`review-heading-${itemId} review-risk-${itemId}`}
          disabled={busy}
          onChange={(e) => setAcks((old) => ({ ...old, [id]: e.target.checked }))}
        />
        <span>{label}</span>
      </label>
    )
  }
  function submit() {
    onSubmit({
      ticket: review.ticket,
      confirm: confirmed,
      highRiskIds: review.items.filter((item) => acks[`risk:${item.id}`]).map((item) => item.id),
      unverifiedIds: review.items
        .filter((item) => acks[`unverified:${item.id}`])
        .map((item) => item.id),
      apps: removal
        ? review.items.map((item) => ({
            itemId: item.id,
            noUndo: !!acks[`undo:${item.id}`],
            gamingRisk: !!acks[`risk:${item.id}`],
            unverified: !!acks[`unverified:${item.id}`],
          }))
        : [],
    })
  }
  return (
    <>
      <p className="intro">{t('reviewIntro')}</p>
      {removal && (
        <div className="notice danger">
          <strong>{t('noUndo')}</strong>
          <p>{t('noUndoDetail')}</p>
          <p>{t('scopeNote')}</p>
        </div>
      )}
      {review.kind === 'history' && <p>{t('historyReview')}</p>}
      {review.kind === 'reinstall' && <p>{t('reinstallReview')}</p>}
      {(review.kind === 'undo' || review.kind === 'undo_all') && <p>{t('undoReview')}</p>}
      <section className="surface review-summary glass" aria-label={t('byType')}>
        <dl className="review-metrics">
          {Object.entries(review.counts).map(([kind, count]) => (
            <div key={kind}>
              <dt>
                <Icon
                  name={
                    kind === 'appx'
                      ? 'apps'
                      : kind === 'service'
                        ? 'service'
                        : kind === 'scheduled_task'
                          ? 'task'
                          : kind === 'power'
                            ? 'power'
                            : kind === 'startup'
                              ? 'startup'
                              : 'registry'
                  }
                />
                {t(kind)}
              </dt>
              <dd>{count}</dd>
            </div>
          ))}
        </dl>
      </section>
      <div className="review-timeline">
        {(['none', 'low', 'high'] as const)
          .filter((risk) => review.items.some((item) => item.risk === risk))
          .map((risk) => (
            <section
              className={`review-risk-group ${risk}`}
              key={risk}
              aria-label={t(risk === 'high' ? 'riskHigh' : risk === 'low' ? 'riskLow' : 'riskNone')}
            >
              <h2 className="risk-group-heading">
                <Icon name={risk === 'high' ? 'high' : 'safe'} />
                {t(risk === 'high' ? 'riskHigh' : risk === 'low' ? 'riskLow' : 'riskNone')}
                <span className="num">
                  {review.items.filter((item) => item.risk === risk).length}
                </span>
              </h2>
              {review.items
                .filter((item) => item.risk === risk)
                .map((item) => (
                  <section className="review-item" key={item.id}>
                    <span className="timeline-marker">
                      <Icon
                        name={
                          item.type === 'appx'
                            ? 'apps'
                            : item.type === 'power'
                              ? 'power'
                              : item.type === 'service'
                                ? 'service'
                                : item.type === 'scheduled_task'
                                  ? 'task'
                                  : item.type === 'startup'
                                    ? 'startup'
                                    : 'registry'
                        }
                      />
                    </span>
                    <div className="section-top">
                      <h2 id={`review-heading-${item.id}`}>{itemName(item, language)}</h2>
                    </div>
                    <p id={`review-risk-${item.id}`}>{item.riskNote[language]}</p>
                    {item.requiresReboot && (
                      <p className="notice warning">
                        <Icon name="warning" />
                        {t('reboot')}
                      </p>
                    )}
                    {removal && ack(`undo:${item.id}`, t('ackNoUndo'))}
                    {item.risk === 'high' &&
                      (removal || review.kind === 'apply') &&
                      ack(`risk:${item.id}`, t('ackRisk'))}
                    {!item.verified &&
                      (removal || review.kind === 'apply') &&
                      ack(`unverified:${item.id}`, t('ackUnverified'))}
                  </section>
                ))}
            </section>
          ))}
      </div>
      <div className="final-consent">
        <label className="consent">
          <Checkbox
            checked={confirmed}
            disabled={busy}
            onChange={(e) => setConfirmed(e.target.checked)}
          />
          <strong>{t('ackAll')}</strong>
        </label>
        <p>{t('uac')}</p>
        <div className="actions">
          <button
            className={`button ${removal ? 'destructive' : 'primary'}`}
            disabled={!ready || busy}
            onClick={submit}
          >
            {busy ? t('executing') : t(action)}
          </button>
          <button className="button subtle" disabled={busy} onClick={onBack}>
            {t('back')}
          </button>
        </div>
      </div>
    </>
  )
}
