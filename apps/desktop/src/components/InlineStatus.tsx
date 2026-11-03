import type { ReactNode } from 'react'
import { Button } from './Button'
export function InlineStatus({
  children,
  error = false,
  next,
  onNext,
}: {
  children: ReactNode
  error?: boolean
  next?: string
  onNext?: () => void
}) {
  return (
    <section className={`notice ${error ? 'danger' : 'success'}`} role={error ? 'alert' : 'status'}>
      <div>{children}</div>
      {next && onNext && (
        <Button variant="ghost" onClick={onNext}>
          {next}
        </Button>
      )}
    </section>
  )
}
