import { useApp } from '../state/AppContext'
import { GamesPanel } from './GamesPanel'
export function GamesPage() {
  const { page, backend, language, data, setPage } = useApp()
  return (
    <>
      {page === 'games' && (
        <GamesPanel
          onMonitorSettings={() => setPage('settings')}
          backend={backend}
          language={language}
          compatible={data?.compatible ?? {}}
        />
      )}
    </>
  )
}
