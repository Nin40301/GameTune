import { ActionBar } from '../components/ActionBar'
import { plural } from '../i18n-format'
import { useApp } from '../state/AppContext'
import { ItemRow } from '../components/ItemRow'
import { Icon } from '../components/Icon'
export function ItemsPage() {
  const {
    page,
    t,
    search,
    setSearch,
    category,
    setCategory,
    data,
    categoryLabel,
    list,
    rows,
    selected,
    canSelect,
    busy,
    toggle,
    setDetail,
    ready,
    prepare,
    language,
  } = useApp()
  return (
    <>
      {page === 'items' && (
        <>
          <div className="toolbar glass">
            <label className="search-label">
              <span>{t('search')}</span>
              <span className="search-field">
                <Icon name="search" />
                <input
                  type="search"
                  placeholder={t('searchPlaceholder')}
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
              </span>
            </label>
            <label>
              {t('category')}
              <span className="select-field">
                <select value={category} onChange={(e) => setCategory(e.target.value)}>
                  <option value="all">{t('all')}</option>
                  {[
                    ...new Set(
                      (data?.items ?? [])
                        .filter((item) => item.type !== 'appx')
                        .map((item) => item.category),
                    ),
                  ].map((value) => (
                    <option key={value} value={value}>
                      {categoryLabel(value)}
                    </option>
                  ))}
                </select>
                <Icon name="chevronDown" />
              </span>
            </label>
          </div>
          <div className="item-list surface">
            {[...new Set(list.map((item) => item.category))].map((group) => (
              <section key={group} aria-label={categoryLabel(group)}>
                <div className="group-heading">
                  <h2>{categoryLabel(group)}</h2>
                  <span className="mono">
                    {list.filter((item) => item.category === group).length}
                  </span>
                </div>
                {list
                  .filter((item) => item.category === group)
                  .map((item) => (
                    <ItemRow
                      key={item.id}
                      item={item}
                      language={language}
                      observation={rows.get(item.id)}
                      checked={selected.includes(item.id)}
                      disabled={!canSelect(item) || busy}
                      blocked={data?.blockedIds.includes(item.id) ?? false}
                      incompatible={data?.compatible[item.id] === false}
                      onToggle={() => toggle(item.id)}
                      onDetail={() => setDetail(item)}
                    />
                  ))}
              </section>
            ))}
          </div>
          <ActionBar summary={plural(language, 'selected', selected.length)}>
            <button
              className="button primary"
              disabled={!ready || !selected.length || busy}
              onClick={() => void prepare('apply', selected)}
            >
              {t('reviewSelection')}
              <Icon name="arrowRight" />
            </button>
          </ActionBar>
        </>
      )}
    </>
  )
}
