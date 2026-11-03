import { itemName } from '../i18n'
import { useState } from 'react'
import type { Language, Review, Approval, Observation } from '../types'
import { translator } from '../i18n'
import { Icon } from './Icon'
import { Checkbox } from './Checkbox'
import { ActionBar } from './ActionBar'
import { currentLabel, nextLabel } from '../state/presentation'
export function ReviewScreen({
  review,
  observations = new Map<string, Observation>(),
  language,
  busy,
  onSubmit,
  onBack,
}: {
  review: Review
  observations?: Map<string, Observation>
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
          aria-describedby={`consent-risk-${itemId}`}
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
                  <details className="review-item" key={item.id}>
                    <summary>
                      <span className="timeline-marker">
                        <Icon
                          name={
                            item.type === 'appx'
                              ? 'apps'
                              : item.type === 'scheduled_task'
                                ? 'task'
                                : item.type
                          }
                        />
                      </span>
                      <span id={`review-heading-${item.id}`}>{itemName(item, language)}</span>
                      <Icon name="chevronDown" />
                    </summary>
                    <p id={`review-risk-${item.id}`}>{item.riskNote[language]}</p>
                    <dl className="diff">
                      <div>
                        <dt>{t('current')}</dt>
                        <dd className="num">{currentLabel(observations.get(item.id), t)}</dd>
                      </div>
                      <div className="diff__arrow" aria-hidden="true">
                        →
                      </div>
                      <div>
                        <dt>{t('next')}</dt>
                        <dd className="num">{nextLabel(item, review.kind, t)}</dd>
                      </div>
                    </dl>
                    <p className="revert-note">
                      <Icon name={item.revert === 'auto' ? 'history' : 'warning'} />
                      {t(item.revert === 'auto' ? 'autoRevert' : 'noUndoDetail')}
                    </p>
                    {item.requiresReboot && (
                      <p className="notice warning">
                        <Icon name="warning" />
                        {t('reboot')}
                      </p>
                    )}
                  </details>
                ))}
            </section>
          ))}
      </div>
      {review.kind === 'apply' && (
        <section className="surface next-steps">
          <h2>{t('whatNext')}</h2>
          <ol>
            {['stepRestore', 'stepPermission', 'stepApply'].map((step, index) => (
              <li key={step}>
                <span className="step-number num">{index + 1}</span>
                <p>{t(step)}</p>
              </li>
            ))}
          </ol>
        </section>
      )}
      <section className="final-consent surface" aria-label={t('consentPanel')}>
        <h2>{t('consentPanel')}</h2>
        {review.items
          .filter((item) => removal || item.risk === 'high' || !item.verified)
          .map((item) => (
            <div className="consent-item" key={item.id}>
              <h3>{itemName(item, language)}</h3>
              <p id={`consent-risk-${item.id}`}>{item.riskNote[language]}</p>
              {removal && ack(`undo:${item.id}`, t('ackNoUndo'))}
              {item.risk === 'high' &&
                (removal || review.kind === 'apply') &&
                ack(`risk:${item.id}`, t('ackRisk'))}
              {!item.verified &&
                (removal || review.kind === 'apply') &&
                ack(`unverified:${item.id}`, t('ackUnverified'))}
            </div>
          ))}
        <label className="consent">
          <Checkbox
            checked={confirmed}
            disabled={busy}
            onChange={(e) => setConfirmed(e.target.checked)}
          />
          <strong>{t('ackAll')}</strong>
        </label>
        <p>{t('uac')}</p>
      </section>
      <ActionBar
        summary={<span id="missing-consent">{ready ? t('uac') : t('missingConsent')}</span>}
      >
        <button className="button subtle" disabled={busy} onClick={onBack}>
          {t('back')}
        </button>
        <button
          className={`button ${removal ? 'destructive' : 'primary'}`}
          disabled={!ready || busy}
          aria-describedby={!ready ? 'missing-consent' : undefined}
          onClick={submit}
        >
          {busy ? t('executing') : t(action)}
        </button>
      </ActionBar>
    </>
  )
}
