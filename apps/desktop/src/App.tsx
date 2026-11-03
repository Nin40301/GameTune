import type { Backend } from './types'
import { useAppModel } from './state/useAppModel'
import { AppProvider } from './state/AppContext'
import { AppShell } from './components/AppShell'
export function App({ backend }: { backend: Backend }) {
  const model = useAppModel(backend)
  return (
    <AppProvider value={model}>
      <AppShell />
    </AppProvider>
  )
}
