import { createPortal } from 'react-dom'
import type { ReactNode } from 'react'
export function ActionBar({ summary, children }: { summary: ReactNode; children: ReactNode }) {
  return createPortal(
    <div className="action-bar sticky-actions">
      <div className="action-bar__inner">
        <p className="action-bar__summary">{summary}</p>
        <div className="action-bar__actions">{children}</div>
      </div>
    </div>,
    document.body,
  )
}
