import { useApp } from '../state/AppContext'
import { Measurements } from './Measurements'
export function MeasurementsPage() {
  const { page, backend, language } = useApp()
  return <>{page === 'benchmark' && <Measurements backend={backend} language={language} />}</>
}
