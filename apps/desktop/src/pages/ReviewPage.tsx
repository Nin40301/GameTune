import { useApp } from '../state/AppContext'
import { ReviewScreen } from '../components/ReviewScreen'
export function ReviewPage() {
  const { page, t, review, language, busy, execute, setReview, setPage } = useApp()
  return (
    <>
      {page === 'review' &&
        (review ? (
          <ReviewScreen
            key={review.ticket}
            review={review}
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
