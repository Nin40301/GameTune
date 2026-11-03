import { useEffect, useRef, useState } from 'react'
/** Changes in status announce immediately; progress announces at most every 3 s. */
export function useAnnouncement(status: string, text: string) {
  const [announcement, setAnnouncement] = useState(text)
  const last = useRef(0)
  const previous = useRef('')
  useEffect(() => {
    const now = Date.now()
    if (previous.current !== status || now - last.current >= 3000) {
      previous.current = status
      last.current = now
      const task = setTimeout(() => setAnnouncement(text), 0)
      return () => clearTimeout(task)
    }
  }, [status, text])
  return announcement
}
