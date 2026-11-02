import { useEffect, useState } from 'react'

// Visual updates only. The Rust collector and an active benchmark keep running.
export function useWindowActivity() {
  const [active, setActive] = useState(
    () => document.visibilityState !== 'hidden' && document.hasFocus(),
  )
  useEffect(() => {
    const focus = () => setActive(document.visibilityState !== 'hidden')
    const blur = () => setActive(false)
    const visibility = () => setActive(document.visibilityState !== 'hidden' && document.hasFocus())
    window.addEventListener('focus', focus)
    window.addEventListener('blur', blur)
    document.addEventListener('visibilitychange', visibility)
    return () => {
      window.removeEventListener('focus', focus)
      window.removeEventListener('blur', blur)
      document.removeEventListener('visibilitychange', visibility)
    }
  }, [])
  return active
}
