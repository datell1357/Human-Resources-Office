import type { AccountRow, AccountStatus } from '../types'

interface TargetTableProps {
  rows: AccountRow[]
  selectedId: string
  onSelect: (row: AccountRow) => void
}

const statusLabel: Record<AccountStatus, string> = {
  active: '활성',
  error: '오류',
  excluded: '제외'
}

export const TargetTable = ({ rows, selectedId, onSelect }: TargetTableProps): React.JSX.Element => {
  if (rows.length === 0) {
    return <div className="empty-table">선택한 상태에 해당하는 계정이 없습니다.</div>
  }

  return (
    <div className="table-scroll">
      <table>
        <thead>
          <tr>
            <th>상태</th>
            <th>참조 주소</th>
            <th>계정</th>
            <th>판단</th>
            <th>메모</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr
              aria-selected={selectedId === row.id}
              className={selectedId === row.id ? 'selected' : ''}
              key={row.id}
              onClick={() => onSelect(row)}
              tabIndex={0}
              onKeyDown={(event) => {
                if (event.key === 'Enter' || event.key === ' ') onSelect(row)
              }}
            >
              <td>
                <span className={`status-label ${row.status}`}>
                  <span aria-hidden="true" className="status-dot" />
                  {statusLabel[row.status]}
                </span>
              </td>
              <td className="url-cell" title={row.referenceUrl || '주소 없음'}>
                {row.referenceUrl || '—'}
              </td>
              <td>{row.accountLabel}</td>
              <td className={`decision ${row.status}`}>{row.decision}</td>
              <td className="note-cell" title={row.note || '메모 없음'}>
                {row.note || '—'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
