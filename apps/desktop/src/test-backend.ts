// Imported only by the compile-time test branch. Never shipped in release assets.
import catalog from '../../../catalog/items.json'
import type {
  Analysis,
  Approval,
  Backend,
  Bootstrap,
  Item,
  Observation,
  Review,
  Selection,
} from './types'
declare global {
  interface Window {
    __gametuneTest?: { executions: Approval[]; reviews: Selection[]; cancelled: number }
  }
}
export function createMockBackend(): Backend {
  const items = catalog.items as unknown as Item[]
  const trace = { executions: [] as Approval[], reviews: [] as Selection[], cancelled: 0 }
  window.__gametuneTest = trace
  let id = 0,
    completed = 0,
    status: Analysis['status'] = 'idle',
    review: Review | null = null
  const installed = [
    'appx.microsoft.copilot',
    'appx.microsoft.549981c3f5f10',
    'appx.microsoft.gamingservices',
    'appx.microsoft.xboxidentityprovider',
  ]
  const safe = [
    'reg.advertising',
    'reg.copilot',
    'reg.cortana',
    'reg.widgets',
    'svc.diagtrack',
    'svc.fax',
    'svc.retaildemo',
  ]
  const history: Bootstrap['history'] = {
    available: true,
    journal: {
      entries: [
        { itemId: 'reg.advertising', status: 'applied', appliedAt: '2026-10-31T12:00:00Z' },
      ],
      needs_recovery: false,
      torn_tail: false,
    },
    apps: [
      {
        itemId: 'appx.microsoft.copilot',
        name: 'Microsoft.Copilot',
        packageFamilyNames: ['Microsoft.Copilot_8wekyb3d8bbwe'],
        status: 'removed',
        attemptedAt: '2026-10-31T12:00:00Z',
      },
    ],
  }
  const bootstrap: Bootstrap = {
    items,
    presets: {
      safe,
      balanced: [...safe, 'reg.edge_preload', 'reg.edge_background', 'power.high_performance'],
      aggressive: [...safe, 'svc.wsearch'],
    },
    compatible: Object.fromEntries(items.map((i) => [i.id, true])),
    blockedIds: ['reg.recall', 'svc.icssvc', 'svc.wmpnetworksvc', 'svc.xboxnetapisvc'],
    history,
  }
  const observations: Observation[] = items.map((item) => ({
    itemId: item.id,
    presence:
      item.type === 'appx'
        ? installed.includes(item.id)
          ? 'exists'
          : 'missing'
        : item.id === 'reg.advertising'
          ? 'already_applied'
          : 'exists',
    exists: item.type !== 'appx' || installed.includes(item.id),
    alreadyApplied: item.id === 'reg.advertising',
    current:
      item.type === 'appx'
        ? { type: 'appx', packages: [`${item.id}_test`], provisioned: [], families: [] }
        : null,
    currentDword: null,
    error: null,
    scope: item.type === 'appx' ? 'current_user_only' : 'machine',
    removable: item.type === 'appx' ? installed.includes(item.id) : null,
  }))
  return {
    bootstrap: async () => bootstrap,
    start: async () => {
      id++
      completed = 0
      status = 'running'
      review = null
      return id
    },
    state: async () => {
      if (status === 'running') {
        completed = Math.min(items.length, completed + 16)
        if (completed === items.length) status = 'complete'
      }
      return {
        id,
        status,
        completed,
        total: items.length,
        itemId: items[Math.max(0, completed - 1)]?.id ?? '',
        observations: status === 'complete' ? observations : [],
        error: null,
      }
    },
    cancel: async () => {
      status = 'cancelled'
      trace.cancelled++
    },
    review: async (selection) => {
      trace.reviews.push(selection)
      const selected = selection.ids.map((id) => {
        const item = items.find((i) => i.id === id)
        if (!item) throw Error('invalid_item')
        return item
      })
      review = {
        ...selection,
        ticket: crypto.randomUUID(),
        items: selected,
        counts: selected.reduce<Review['counts']>(
          (counts, item) => ({ ...counts, [item.type]: (counts[item.type] ?? 0) + 1 }),
          {},
        ),
      }
      return review
    },
    execute: async (approval) => {
      if (!review || review.ticket !== approval.ticket) throw Error('review_required')
      if (!approval.confirm) throw Error('confirmation_required')
      if (
        review.kind === 'remove_apps' &&
        review.items.some(
          (item) =>
            !approval.apps.some(
              (app) =>
                app.itemId === item.id && app.noUndo && (item.risk !== 'high' || app.gamingRisk),
            ),
        )
      )
        throw Error('confirmation_required')
      review = null
      trace.executions.push(approval)
      if (new URLSearchParams(location.search).has('uac-cancel')) throw Error('uac_cancelled')
      return {
        ok: true,
        result: {
          operation: {
            items: approval.apps.length
              ? approval.apps.map((app) => ({ itemId: app.itemId, outcome: 'removed' }))
              : [{ itemId: 'reg.advertising', outcome: 'applied' }],
          },
          journal: history.journal,
          apps: history.apps,
        },
        error: null,
      }
    },
  }
}
