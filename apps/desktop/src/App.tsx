import { lazy, Suspense } from 'react'
import type { Backend } from './types'
import { useAppModel } from './state/useAppModel'
import { AppProvider } from './state/AppContext'
import { AppShell } from './components/AppShell'
const DevDesignSystem = import.meta.env.DEV ? lazy(() => import('./dev/DesignSystem')) : null
export function App({ backend }: { backend: Backend }) {
  if (DevDesignSystem && location.pathname === '/design-system')
    return (
      <Suspense>
        <DevDesignSystem />
      </Suspense>
    )
  return <Application backend={backend} />
}
function Application({ backend }: { backend: Backend }) {
  const model = useAppModel(backend)
  return (
    <AppProvider value={model}>
      <AppShell />
    </AppProvider>
  )
}
