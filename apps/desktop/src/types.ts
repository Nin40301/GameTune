export type Language = 'pt-BR' | 'en'
export type Theme = 'dark' | 'light'
export type Kind = 'registry' | 'service' | 'scheduled_task' | 'appx' | 'startup' | 'power'
export type Page =
  'home' | 'items' | 'apps' | 'review' | 'history' | 'settings' | 'benchmark' | 'games'
export type ReviewKind = 'apply' | 'remove_apps' | 'undo' | 'undo_all' | 'history' | 'reinstall'
export interface Localized {
  'pt-BR': string
  en: string
}
export interface Item {
  id: string
  type: Kind
  category: string
  level: number
  risk: 'none' | 'low' | 'high'
  riskNote: Localized
  name: Localized
  description: Localized
  verified: boolean
  revert: 'auto' | 'manual'
  requiresReboot: boolean
  target: Record<string, unknown>
  sources: string[]
  minBuild: number
}
export interface Observation {
  itemId: string
  presence: 'exists' | 'missing' | 'already_applied' | 'unknown' | 'discovery'
  exists: boolean | null
  alreadyApplied: boolean | null
  current: unknown
  currentDword: number | null
  error: unknown
  scope: string
  removable: boolean | null
}
export interface JournalEntry {
  itemId: string
  status: string
  appliedAt: string
}
export interface RemovedApp {
  itemId: string
  name: string
  packageFamilyNames: string[]
  status: string
  attemptedAt: string
}
export interface Journal {
  entries: JournalEntry[]
  needs_recovery: boolean
  torn_tail: boolean
}
export interface History {
  available: boolean
  journal: Journal | null
  apps: RemovedApp[]
}
export interface Bootstrap {
  system: {
    windowsBuild: number
    edition: string
    protection: 'unknown' | 'disabled_by_policy'
    legacyData: boolean
  }
  items: Item[]
  presets: Record<string, string[]>
  compatible: Record<string, boolean>
  blockedIds: string[]
  history: History
}
export interface Analysis {
  id: number
  status: 'idle' | 'running' | 'cancelling' | 'cancelled' | 'complete' | 'error'
  completed: number
  total: number
  itemId: string
  observations: Observation[]
  error: string | null
}
export interface Selection {
  kind: ReviewKind
  ids: string[]
}
export interface Review extends Selection {
  ticket: string
  counts: Partial<Record<Kind, number>>
  items: Item[]
}
export interface AppConsent {
  itemId: string
  noUndo: boolean
  gamingRisk: boolean
  unverified: boolean
}
export interface Approval {
  ticket: string
  confirm: boolean
  highRiskIds: string[]
  unverifiedIds: string[]
  apps: AppConsent[]
}
export interface Reply {
  ok: boolean
  result: { operation?: unknown; journal?: Journal | null; apps?: RemovedApp[] } | null
  error: string | null
}
export interface Backend {
  support(action: SupportAction): Promise<DiagnosticPreview | null>
  games(action: GamesAction): Promise<GamesView>
  lab(action: LabAction): Promise<LabStatus>
  labFile(action: LabFile): Promise<LabStatus>
  bootstrap(): Promise<Bootstrap>
  start(): Promise<number>
  state(): Promise<Analysis>
  cancel(): Promise<void>
  review(selection: Selection): Promise<Review>
  execute(approval: Approval): Promise<Reply>
}
export interface Game {
  id: string
  name: string
  exe: string
  actionIds: string[]
  consent: boolean
  enabled: boolean
}
export interface GameCandidate {
  launcher: string
  name: string
  installDir: string
  exe: string | null
}
export interface GameAction {
  id: string
  pauseService: boolean
  name: Localized
  description: Localized
}
export interface GameMonitor {
  running: boolean
  pid: number
  instance: string
  helperPid: number | null
  activeGames: string[]
  recoveryPending: boolean
  error: string | null
  readonly: boolean
}
export interface GamesView {
  library: { version: number; games: Game[]; language: string }
  candidates: GameCandidate[]
  actions: GameAction[]
  monitor: GameMonitor
}
export type GamesAction =
  | { kind: 'status' | 'discover' | 'add_manual' | 'stop' }
  | { kind: 'add_candidate'; index: number }
  | { kind: 'save'; game: Game; language: string }
  | { kind: 'remove'; id: string; confirm: boolean }
  | { kind: 'start'; confirm: boolean; readonly: boolean; language: string }
export interface MetricSample {
  uptimeMs: number
  timestampMs: number
  totalBytes: number
  availableBytes: number
  cpuPercent: number | null
  processCount: number
  top: { name: string; count: number; memoryBytes: number }[]
  appPrivateBytes: number
  appCpuPercent: number | null
  appProcesses: number
  boot: string
  powerPlan: string
}
export interface Conditions {
  appsClosed: boolean
  sameWorkload: boolean
  cooled: boolean
  samePower: boolean
}
export type LabAction =
  | { kind: 'status' | 'cancel' }
  | { kind: 'monitor'; enabled: boolean }
  | { kind: 'start'; conditions: Conditions }
  | { kind: 'reset'; confirm: boolean }
export type LabFile =
  | { kind: 'choose_presentmon' | 'import_capture' }
  | { kind: 'export'; format: 'json' | 'html'; anonymize: boolean; english: boolean }
export interface Distribution {
  runs: number
  median: number
  p5: number
  p95: number
  minimum: number
  maximum: number
  mad: number
  runMedians: number[]
}
export interface Comparison {
  metric: string
  before: Distribution
  after: Distribution
  medianDifference: number
  noiseBand: number
  conclusion: 'inconclusive' | 'difference_observed'
}
export interface FrameCapture {
  application: string
  frames: number
  averageFps: number
  onePercentLowFps: number
  frameTime: Pick<Distribution, 'median' | 'p5' | 'p95' | 'minimum' | 'maximum' | 'mad'>
  frameTimeP99: number
  intervalColumn: string
}
export interface LabStatus {
  protocol: {
    version: number
    phase: 'before' | 'await_apply' | 'await_reboot' | 'after' | 'complete'
    baselineBoot: string | null
    powerPlan: string | null
    before: { warmup: boolean; durationMs: number }[]
    after: { warmup: boolean; durationMs: number }[]
    interrupted: boolean
    pendingRun: boolean
    appliedIds: string[]
    presentmon: FrameCapture[]
  }
  latest: MetricSample | null
  monitor: boolean
  elapsedSeconds: number
  error: string | null
  comparisons: Comparison[]
  presentmonAvailable: boolean
}

export type SupportTool = 'protection' | 'logs' | 'restart_dialog'
export type SupportAction =
  | { kind: 'preview' }
  | { kind: 'save'; token: string; confirm: boolean }
  | { kind: 'open'; tool: SupportTool }
export interface DiagnosticPreview {
  token: string
  files: { name: string; bytes: number; content: string }[]
  unavailable: string[]
}
