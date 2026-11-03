import { InlineStatus } from '../components/InlineStatus'
import { Checkbox } from '../components/Checkbox'
import { useEffect, useState } from 'react'
import type { Backend, Game, GamesAction, GamesView, Language } from '../types'
import { translator } from '../i18n'
import { Icon } from '../components/Icon'
import { Segmented } from '../components/Segmented'
export function GamesPanel({
  backend,
  language,
  settings = false,
  compatible = {},
}: {
  backend: Backend
  language: Language
  settings?: boolean
  compatible?: Record<string, boolean>
}) {
  const [view, setView] = useState<GamesView | null>(null),
    [busy, setBusy] = useState(false),
    [error, setError] = useState<string | null>(null)
  const [editing, setEditing] = useState<Game | null>(null),
    [ack, setAck] = useState(false),
    [remove, setRemove] = useState(false)
  const t = translator(language)
  useEffect(() => {
    let active = true,
      pending = false
    const poll = () => {
      if (pending || document.visibilityState === 'hidden') return
      pending = true
      void backend
        .games({ kind: 'status' })
        .then((v) => {
          if (active) setView(v)
        })
        .catch((e: unknown) => {
          if (active) setError(typeof e === 'string' ? e : 'games_storage')
        })
        .finally(() => {
          pending = false
        })
    }
    poll()
    const timer = setInterval(poll, 2000)
    return () => {
      active = false
      clearInterval(timer)
    }
  }, [backend])
  async function act(action: GamesAction) {
    setBusy(true)
    setError(null)
    try {
      const result = await backend.games(action)
      setView(result)
      if (action.kind === 'save')
        setEditing(result.library.games.find((g) => g.id === action.game.id) ?? null)
      if (action.kind === 'remove') {
        setEditing(null)
        setRemove(false)
      }
      if (action.kind === 'start') setAck(false)
    } catch (e) {
      setError(typeof e === 'string' ? e : 'games_storage')
    } finally {
      setBusy(false)
    }
  }
  const live = view?.monitor.running ?? false
  return (
    <section className="games-panel" aria-label={t(settings ? 'gameMonitor' : 'games')}>
      {(error || view?.monitor.error) && (
        <InlineStatus error next={t('retry')} onNext={() => void act({ kind: 'status' })}>
          {t(error ?? view?.monitor.error ?? 'games_storage')}
        </InlineStatus>
      )}
      {settings ? (
        <>
          <h2>{t('gameMonitor')}</h2>
          <p>{t('monitorExplanation')}</p>
          <p className="pill" role="status">
            {t(live ? 'monitorRunning' : 'monitorStopped')}
            {view?.monitor.readonly && ` · ${t('observationOnly')}`}
          </p>
          {view?.monitor.recoveryPending && (
            <p className="notice warning">{t('monitorRecovery')}</p>
          )}
          {!live && (
            <label className="check-line">
              <Checkbox checked={ack} onChange={(e) => setAck(e.target.checked)} />
              {t('monitorConsent')}
            </label>
          )}
          <button
            className="button secondary"
            disabled={busy || !view || (!live && !ack)}
            onClick={() => {
              void act(
                live
                  ? { kind: 'stop' }
                  : { kind: 'start', confirm: ack, readonly: false, language },
              )
            }}
          >
            {t(live ? 'stopMonitor' : 'startMonitor')}
          </button>
          <p className="muted small">{t('monitorNoAutostart')}</p>
        </>
      ) : (
        <>
          <p className="intro">{t('gamesIntro')}</p>
          <div className="action-row">
            <button
              className="button secondary"
              disabled={busy}
              onClick={() => {
                void act({ kind: 'discover' })
              }}
            >
              {t('discoverGames')}
            </button>
            <button
              className={`button ${editing ? 'secondary' : 'primary'}`}
              disabled={busy || live}
              onClick={() => {
                void act({ kind: 'add_manual' })
              }}
            >
              {t('addGame')}
            </button>
          </div>
          {live && <p className="notice warning">{t('stopBeforeEdit')}</p>}
          <div className="games-layout">
            <section className="surface game-library">
              <h2>{t('registeredGames')}</h2>
              {!view?.library.games.length && <p>{t('noGames')}</p>}
              <ul>
                {view?.library.games.map((game) => (
                  <li key={game.id}>
                    <button
                      className="game-choice"
                      aria-pressed={editing?.id === game.id}
                      onClick={() => {
                        setEditing({ ...game, actionIds: [...game.actionIds] })
                        setRemove(false)
                      }}
                    >
                      <strong>
                        <Icon name="games" />
                        {game.name}
                      </strong>
                      <span>
                        {t(
                          view.monitor.activeGames.includes(game.id)
                            ? 'gameDetected'
                            : game.enabled
                              ? 'profileEnabled'
                              : 'profileDisabled',
                        )}
                      </span>
                    </button>
                  </li>
                ))}
              </ul>
              <p className="muted small">{t('monitorSettingsHint')}</p>
            </section>
            <section className="surface game-editor glass">
              <h2>{t('temporaryProfile')}</h2>
              {editing ? (
                <>
                  <p className="break-text mono">{editing.exe}</p>
                  <p>{t('temporaryRules')}</p>
                  <fieldset disabled={busy || live}>
                    <legend>{t('gameActions')}</legend>
                    {view?.actions.map((action) => (
                      <label className="game-action" key={action.id}>
                        <Checkbox
                          checked={editing.actionIds.includes(action.id)}
                          disabled={compatible[action.id] === false}
                          onChange={(e) =>
                            setEditing({
                              ...editing,
                              actionIds: e.target.checked
                                ? [...editing.actionIds, action.id]
                                : editing.actionIds.filter((id) => id !== action.id),
                            })
                          }
                        />
                        <span>
                          <strong>{action.name[language]}</strong>
                          <small>{action.description[language]}</small>
                          {compatible[action.id] === false && <small>{t('incompatible')}</small>}
                        </span>
                      </label>
                    ))}
                    <label className="check-line">
                      <Checkbox
                        checked={editing.consent}
                        onChange={(e) => setEditing({ ...editing, consent: e.target.checked })}
                      />
                      {t('gameConsent')}
                    </label>
                    <div className="game-profile-segment">
                      <span>{t('enableProfile')}</span>
                      <Segmented
                        label={t('enableProfile')}
                        value={editing.enabled ? 'on' : 'off'}
                        onChange={(value) => setEditing({ ...editing, enabled: value === 'on' })}
                        options={[
                          { value: 'off', label: t('profileOff') },
                          { value: 'on', label: t('profileOn') },
                        ]}
                        disabled={busy || live}
                      />
                    </div>
                  </fieldset>
                  <button
                    className="button primary"
                    disabled={busy || live || (editing.enabled && !editing.consent)}
                    onClick={() => {
                      void act({ kind: 'save', game: editing, language })
                    }}
                  >
                    {t('saveGame')}
                  </button>
                  <details className="remove-game">
                    <summary>{t('removeGame')}</summary>
                    <label className="check-line">
                      <Checkbox
                        checked={remove}
                        disabled={live}
                        onChange={(e) => setRemove(e.target.checked)}
                      />
                      {t('removeGameConsent')}
                    </label>
                    <button
                      className="button secondary"
                      disabled={!remove || live || busy}
                      onClick={() => {
                        void act({ kind: 'remove', id: editing.id, confirm: remove })
                      }}
                    >
                      {t('removeGame')}
                    </button>
                  </details>
                </>
              ) : (
                <p>{t('chooseGame')}</p>
              )}
            </section>
          </div>
          <section className="surface game-candidates">
            <h2>{t('discoveredGames')}</h2>
            <p>{t('discoveryLimits')}</p>
            {view?.candidates.map((candidate, index) => (
              <article key={`${candidate.launcher}-${index}`}>
                <div>
                  <h3>
                    <Icon name={candidate.launcher === 'minecraft' ? 'minecraft' : 'games'} />
                    {candidate.name}
                  </h3>
                  <p className="muted small">{t(`launcher_${candidate.launcher}`)}</p>
                  <p className="mono break-text">{candidate.exe ?? candidate.installDir}</p>
                </div>
                <button
                  className="button secondary"
                  disabled={busy || live || !candidate.exe}
                  onClick={() => {
                    void act({ kind: 'add_candidate', index })
                  }}
                >
                  {t(candidate.exe ? 'registerCandidate' : 'selectExeManually')}
                </button>
              </article>
            ))}
          </section>
          <section className="notice">
            <h2>{t('gameDetectionLimits')}</h2>
            <p>{t('gameDetectionExplanation')}</p>
          </section>
        </>
      )}
    </section>
  )
}
