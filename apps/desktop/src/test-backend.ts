// Imported only by the compile-time test branch. Never shipped in release assets.
import catalog from '../../../catalog/items.json'
import gameActions from '../../../catalog/game-actions.json'
import type {
  Analysis,
  Approval,
  Backend,
  Bootstrap,
  Item,
  Observation,
  Review,
  Selection,
  LabStatus,
  GamesView,
} from './types'
declare global {
  interface Window {
    __ametistaTest?: { executions: Approval[]; reviews: Selection[]; cancelled: number }
  }
}
export function createMockBackend(): Backend {
  const scenario = new URLSearchParams(location.search).get('ui-state')
  const items = catalog.items as unknown as Item[]
  const trace = { executions: [] as Approval[], reviews: [] as Selection[], cancelled: 0 }
  window.__ametistaTest = trace
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
    system: { windowsBuild: 26200, edition: 'pro', protection: 'unknown', legacyData: false },
    items,
    presets: {
      safe,
      balanced: [...safe, 'reg.edge_preload', 'reg.edge_background', 'power.high_performance'],
      aggressive: [
        ...safe,
        'reg.edge_preload',
        'reg.edge_background',
        'power.high_performance',
        'svc.wsearch',
      ],
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
        : item.type === 'service'
          ? { type: 'service', start_mode: 3, running: true }
          : item.type === 'scheduled_task'
            ? { type: 'scheduled_task', enabled: true }
            : null,
    currentDword:
      item.type === 'registry'
        ? item.id === 'reg.advertising'
          ? Number(item.target.value)
          : Number(item.target.value) === 0
            ? 1
            : 0
        : null,
    error: null,
    scope: item.type === 'appx' ? 'current_user_only' : 'machine',
    removable: item.type === 'appx' ? installed.includes(item.id) : null,
  }))
  const lab: LabStatus = {
    protocol: {
      version: 1,
      phase: 'before',
      baselineBoot: null,
      powerPlan: null,
      before: [],
      after: [],
      interrupted: false,
      pendingRun: false,
      appliedIds: [],
      presentmon: [],
    },
    latest: {
      uptimeMs: 100000,
      timestampMs: Date.now(),
      totalBytes: 16 * 1073741824,
      availableBytes: 6 * 1073741824,
      cpuPercent: 4.5,
      processCount: 120,
      top: [{ name: 'fixture.exe', count: 2, memoryBytes: 100 * 1048576 }],
      appPrivateBytes: 40 * 1048576,
      appCpuPercent: 0.2,
      appProcesses: 5,
      boot: 'fixture',
      powerPlan: 'fixture-plan',
    },
    monitor: true,
    elapsedSeconds: 0,
    error: null,
    comparisons: [],
    presentmonAvailable: false,
  }
  if (new URLSearchParams(location.search).has('benchmark-complete')) {
    lab.protocol.phase = 'complete'
    lab.protocol.before = Array.from({ length: 5 }, (_, i) => ({
      warmup: i === 0,
      durationMs: 60000,
    }))
    lab.protocol.after = [...lab.protocol.before]
    const d = {
      runs: 4,
      median: 10,
      p5: 9,
      p95: 12,
      minimum: 9,
      maximum: 12,
      mad: 1,
      runMedians: [9, 10, 11, 12],
    }
    lab.comparisons = [
      {
        metric: 'cpu',
        before: d,
        after: d,
        medianDifference: 0,
        noiseBand: 3,
        conclusion: 'inconclusive',
      },
    ]
  }
  const games: GamesView = {
    library: { version: 1, language: 'pt-BR', games: [] },
    candidates: [],
    actions: gameActions.actions,
    monitor: {
      running: false,
      pid: 0,
      instance: 'fixture',
      helperPid: null,
      activeGames: [],
      recoveryPending: false,
      error: null,
      readonly: false,
    },
  }
  if (scenario === 'empty') {
    history.journal = { entries: [], needs_recovery: false, torn_tail: false }
    history.apps = []
    lab.latest = null
    lab.comparisons = []
  }
  return {
    support: async (action) =>
      action.kind === 'preview'
        ? {
            token: 'mock-preview',
            files: [{ name: 'diagnostic.json', bytes: 2, content: '{}' }],
            unavailable: ['logs'],
          }
        : null,
    games: async (action) => {
      if (action.kind === 'add_manual')
        games.library.games.push({
          id: 'fixture-game',
          name: 'Sample.exe',
          exe: 'C:\\Games\\Sample.exe',
          actionIds: [],
          consent: false,
          enabled: false,
        })
      if (action.kind === 'discover')
        games.candidates = [
          { launcher: 'steam', name: 'Sample Steam Game', installDir: 'C:\\Games', exe: null },
        ]
      if (action.kind === 'save') {
        if (action.game.enabled && !action.game.consent) throw 'games_invalid'
        games.library.games = games.library.games.map((g) =>
          g.id === action.game.id ? action.game : g,
        )
      }
      if (action.kind === 'remove' && action.confirm)
        games.library.games = games.library.games.filter((g) => g.id !== action.id)
      if (action.kind === 'start') {
        if (!action.confirm) throw 'games_invalid'
        games.monitor.running = true
        games.monitor.readonly = action.readonly
      }
      if (action.kind === 'stop') games.monitor.running = false
      return structuredClone(games)
    },
    lab: async (action) => {
      // Test-only measured samples advance in time, so visual history can render.
      if (action.kind === 'status' && lab.monitor && lab.latest) lab.latest.timestampMs = Date.now()
      if (action.kind === 'monitor') lab.monitor = action.enabled
      if (action.kind === 'start') {
        if (!Object.values(action.conditions).every(Boolean)) throw 'benchmark_conditions'
        lab.protocol.pendingRun = true
        lab.elapsedSeconds = 1
      }
      if (action.kind === 'cancel') {
        lab.protocol.pendingRun = false
        lab.protocol.interrupted = true
      }
      if (action.kind === 'reset' && action.confirm) {
        lab.protocol.phase = 'before'
        lab.protocol.before = []
        lab.protocol.after = []
        lab.comparisons = []
      }
      return structuredClone(lab)
    },
    labFile: async (action) => {
      if (action.kind === 'choose_presentmon') lab.presentmonAvailable = true
      return structuredClone(lab)
    },
    bootstrap: async () => bootstrap,
    start: async () => {
      if (scenario === 'error') throw Error('operation_failed')
      id++
      completed = 0
      status = 'running'
      review = null
      return id
    },
    state: async () => {
      if (status === 'running' && scenario !== 'loading') {
        completed = Math.min(items.length, completed + 16)
        if (completed === items.length) status = 'complete'
      }
      return {
        id,
        status,
        completed,
        total: items.length,
        itemId: items[Math.max(0, completed - 1)]?.id ?? '',
        observations:
          status === 'complete'
            ? scenario === 'empty'
              ? observations.map((row) => ({
                  ...row,
                  exists: false,
                  removable: false,
                  presence: 'missing' as const,
                }))
              : observations
            : [],
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
