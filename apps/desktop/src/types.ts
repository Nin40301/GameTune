export type Language = 'pt-BR' | 'en'
export type Theme = 'dark' | 'light'
export type Kind = 'registry' | 'service' | 'scheduled_task' | 'appx' | 'startup' | 'power'
export type Page = 'home' | 'items' | 'apps' | 'review' | 'history' | 'settings'
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
  bootstrap(): Promise<Bootstrap>
  start(): Promise<number>
  state(): Promise<Analysis>
  cancel(): Promise<void>
  review(selection: Selection): Promise<Review>
  execute(approval: Approval): Promise<Reply>
}
