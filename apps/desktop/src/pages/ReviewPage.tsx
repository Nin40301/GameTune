import { useApp } from '../state/AppContext'
import { ReviewScreen } from '../components/ReviewScreen'
export function ReviewPage() {
  const { page, t, review, language, busy, execute, rows, setReview, setPage } = useApp()
  return (
    <>
      {page === 'review' &&
        (review ? (
          <ReviewScreen
            key={review.ticket}
            review={review}
            observations={rows}
            language={language}
            busy={busy}
            onSubmit={(approval) => void execute(approval)}
            onBack={() => {
              setReview(null)
              setPage(review.kind === 'remove_apps' ? 'apps' : 'items')
            }}
          />
        ) : (
          <p className="empty-state">{t('reviewRequired')}</p>
        ))}
    </>
  )
}
