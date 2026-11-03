import { useApp } from '../state/AppContext'
import { GamesPanel } from './GamesPanel'
export function GamesPage() {
  const { page, backend, language, data } = useApp()
  return (
    <>
      {page === 'games' && (
        <GamesPanel backend={backend} language={language} compatible={data?.compatible ?? {}} />
      )}
    </>
  )
}
