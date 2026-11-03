import type { Reply } from '../types'
function record(value: unknown): Record<string, unknown> | null {
  return value !== null && typeof value === 'object' && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null
}
export function operationRows(reply: Reply) {
  const operation = record(reply.result?.operation),
    report = record(operation?.report)
  const candidate = operation?.items ?? report?.items ?? operation?.apps
  return Array.isArray(candidate) ? candidate.map(record).filter((row) => row !== null) : []
}
