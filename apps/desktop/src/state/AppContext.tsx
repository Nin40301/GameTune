import { createContext, useContext, type ReactNode } from 'react'
import type { AppModel } from './useAppModel'
const Context = createContext<AppModel | null>(null)
export function AppProvider({ value, children }: { value: AppModel; children: ReactNode }) {
  return <Context.Provider value={value}>{children}</Context.Provider>
}
export function useApp() {
  const model = useContext(Context)
  if (!model) throw new Error('App context unavailable')
  return model
}
