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
    riskFilter,
    setRiskFilter,
    revertFilter,
    setRevertFilter,
    selectedOnly,
    setSelectedOnly,
    selectGroup,
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
          <div className="filter-chips" role="group" aria-label={t('filters')}>
            <label>
              {t('riskFilter')}
              <span className="select-field">
                <select value={riskFilter} onChange={(e) => setRiskFilter(e.target.value)}>
                  <option value="all">{t('allRisk')}</option>
                  {['none', 'low', 'high'].map((risk) => (
                    <option value={risk} key={risk}>
                      {t(risk === 'high' ? 'riskHigh' : risk === 'low' ? 'riskLow' : 'riskNone')}
                    </option>
                  ))}
                </select>
                <Icon name="chevronDown" />
              </span>
            </label>
            <label>
              {t('revertFilter')}
              <span className="select-field">
                <select value={revertFilter} onChange={(e) => setRevertFilter(e.target.value)}>
                  <option value="all">{t('allRevert')}</option>
                  <option value="auto">{t('reversible')}</option>
                  <option value="manual">{t('notReversible')}</option>
                </select>
                <Icon name="chevronDown" />
              </span>
            </label>
            <button
              className="button subtle"
              type="button"
              aria-pressed={selectedOnly}
              onClick={() => setSelectedOnly(!selectedOnly)}
            >
              {t('filterSelected')}
            </button>
          </div>
          {!list.length && (
            <div className="empty-state">
              <Icon name="items" />
              <p>{t('selectionEmpty')}</p>
              <button
                className="button secondary"
                onClick={() => {
                  setSearch('')
                  setCategory('all')
                  setRiskFilter('all')
                  setRevertFilter('all')
                  setSelectedOnly(false)
                }}
              >
                {t('clearFilters')}
              </button>
            </div>
          )}
          <div className="item-list surface">
            {[...new Set(list.map((item) => item.category))].map((group) => (
              <details open key={group} className="item-group">
                <summary>
                  <span>{categoryLabel(group)}</span>
                  <span className="num">
                    {list.filter((item) => item.category === group).length}
                  </span>
                  <Icon name="chevronDown" />
                </summary>
                <div className="group-actions">
                  <button
                    type="button"
                    className="button subtle"
                    disabled={busy}
                    title={t('safeGroupOnly')}
                    onClick={() =>
                      selectGroup(
                        list.filter((item) => item.category === group).map((item) => item.id),
                      )
                    }
                  >
                    {t('selectGroup')}
                  </button>
                  <span>{t('safeGroupOnly')}</span>
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
              </details>
            ))}
          </div>
          <ActionBar
            summary={
              <>
                {plural(language, 'selected', selected.length)}
                <span className="action-summary-types">
                  {[
                    ...new Set(
                      (data?.items ?? [])
                        .filter((item) => selected.includes(item.id))
                        .map((item) => item.type),
                    ),
                  ].map((type) => (
                    <span key={type}>
                      {plural(
                        language,
                        `typeCount_${type}`,
                        (data?.items ?? []).filter(
                          (item) => selected.includes(item.id) && item.type === type,
                        ).length,
                      )}
                    </span>
                  ))}
                </span>
              </>
            }
          >
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
