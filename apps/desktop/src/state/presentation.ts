import type { Item, Observation } from '../types'
export function currentLabel(row: Observation | undefined, t: (key: string) => string): string {
  if (!row || row.error || row.exists === null) return t('valueUnknown')
  if (row.exists === false) return t('missing')
  if (row.currentDword !== null) return String(row.currentDword)
  const current = row.current as Record<string, unknown> | null
  if (!current) return t('valueUnknown')
  if (current.type === 'registry' && current.value === null) return t('valueAbsent')
  if (current.type === 'service')
    return (
      t(`serviceMode_${current.start_mode}`) +
      (typeof current.running === 'boolean'
        ? ` · ${t(current.running ? 'serviceRunning' : 'serviceStopped')}`
        : '')
    )
  if (typeof current.enabled === 'boolean')
    return t(current.enabled ? 'nextEnabled' : 'nextDisabled')
  if (typeof current.active_guid === 'string') return current.active_guid
  return t(row.presence)
}
// Describes the fixed catalog target; never executes or derives authorization.
export function nextLabel(
  item: Pick<Item, 'type' | 'target'>,
  kind: string,
  t: (key: string) => string,
): string {
  if (kind === 'undo' || kind === 'undo_all') return t('nextRestore')
  if (item.type === 'registry') return String(item.target.value)
  if (item.type === 'service')
    return t(item.target.mode === 'manual' ? 'nextManual' : 'nextDisabled')
  if (item.type === 'power') return t('nextHighPower')
  if (item.type === 'appx') return t('nextAppRemoval')
  return t('nextDisabled')
}
