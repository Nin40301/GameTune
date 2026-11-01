import { invoke } from '@tauri-apps/api/core'
import type { Backend } from './types'
declare const __TEST_BACKEND__: boolean
export async function createBackend(): Promise<Backend> {
  if (__TEST_BACKEND__) return (await import('./test-backend')).createMockBackend()
  return {
    bootstrap: () => invoke('bootstrap'),
    start: () => invoke('analysis_start'),
    state: () => invoke('analysis_state'),
    cancel: () => invoke('analysis_cancel'),
    review: (selection) => invoke('prepare_review', { selection }),
    execute: (approval) => invoke('execute_review', { approval }),
  }
}
