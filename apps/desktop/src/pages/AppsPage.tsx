import { ActionBar } from '../components/ActionBar'
import { plural } from '../i18n-format'
import { useApp } from '../state/AppContext'
import { ItemRow } from '../components/ItemRow'
import { Icon } from '../components/Icon'
export function AppsPage() {
  const {
    page,
    t,
    ready,
    scanning,
    data,
    analyze,
    apps,
    language,
    rows,
    appIds,
    busy,
    toggle,
    setDetail,
    prepare,
  } = useApp()
  return (
    <>
      {page === 'apps' && (
        <>
          <p className="intro">{t('appsIntro')}</p>
          <div className="notice warning">
            <strong>{t('noUndo')}</strong>
            <p>{t('noUndoDetail')}</p>
          </div>
          <p className="muted small">{t('scopeNote')}</p>
          {!ready ? (
            <section className="empty-state">
              <Icon name="apps" />
              <p>{t('notInstalled')}</p>
              <button
                className="button primary"
                disabled={scanning || !data}
                onClick={() => void analyze()}
              >
                {t('analyze')}
              </button>
            </section>
          ) : apps.length === 0 ? (
            <p className="empty-state">{t('noApps')}</p>
          ) : (
            <div className="app-grid">
              {apps.map((item) => (
                <ItemRow
                  key={item.id}
                  item={item}
                  language={language}
                  observation={rows.get(item.id)}
                  checked={appIds.includes(item.id)}
                  disabled={busy}
                  blocked={false}
                  incompatible={false}
                  onToggle={() => toggle(item.id, true)}
                  onDetail={() => setDetail(item)}
                />
              ))}
            </div>
          )}
          <ActionBar summary={plural(language, 'selected', appIds.length)}>
            <button
              className="button primary"
              disabled={!ready || !appIds.length || busy}
              onClick={() => void prepare('remove_apps', appIds)}
            >
              {t('reviewApps')}
              <Icon name="arrowRight" />
            </button>
          </ActionBar>
        </>
      )}
    </>
  )
}
