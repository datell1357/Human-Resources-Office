import type { AccountStatus, AccountSummary } from '../types'

export type StatusFilter = 'all' | AccountStatus

interface SummaryStripProps {
  summary: AccountSummary
  selected: StatusFilter
  onSelect: (filter: StatusFilter) => void
}

const items: Array<{ key: StatusFilter; label: string }> = [
  { key: 'all', label: '전체' },
  { key: 'active', label: '활성' },
  { key: 'error', label: '오류' },
  { key: 'excluded', label: '제외' }
]

export const SummaryStrip = ({ summary, selected, onSelect }: SummaryStripProps): React.JSX.Element => (
  <div aria-label="상태별 대상 필터" className="summary-strip" role="group">
    {items.map(({ key, label }) => (
      <button
        aria-pressed={selected === key}
        className={`summary-item ${key} ${selected === key ? 'selected' : ''}`}
        key={key}
        onClick={() => onSelect(key)}
        type="button"
      >
        <span>{label}</span>
        <strong>{key === 'all' ? summary.total : summary[key]}</strong>
      </button>
    ))}
  </div>
)
