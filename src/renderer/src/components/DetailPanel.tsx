import type { AccountRow, AccountStatus } from '../types'

interface DetailPanelProps {
  row: AccountRow | null
}

const statusLabel: Record<AccountStatus, string> = {
  active: '활성',
  error: '오류',
  excluded: '제외'
}

export const DetailPanel = ({ row }: DetailPanelProps): React.JSX.Element => (
  <aside className="detail-panel">
    <header>
      <h2>대상 상세</h2>
    </header>
    {row ? (
      <div className="detail-content">
        <div className="field-group">
          <span>참조 주소</span>
          <div className="readonly-field url-detail">{row.referenceUrl || '입력 없음'}</div>
        </div>
        <div className="field-group">
          <span>계정 (마스킹)</span>
          <div className="readonly-field">{row.accountLabel}</div>
        </div>
        <dl>
          <div>
            <dt>상태</dt>
            <dd className={`detail-status ${row.status}`}>
              <span aria-hidden="true" className="status-dot" />
              {statusLabel[row.status]}
            </dd>
          </div>
          <div>
            <dt>판단</dt>
            <dd>{row.decision}</dd>
          </div>
          <div>
            <dt>메모</dt>
            <dd>{row.note || '메모 없음'}</dd>
          </div>
          <div>
            <dt>검증 메시지</dt>
            <dd className="validation-copy">{row.validationMessage}</dd>
          </div>
          <div>
            <dt>원본 줄 번호</dt>
            <dd>{row.lineNumber}</dd>
          </div>
        </dl>
      </div>
    ) : (
      <p className="detail-empty">목록에서 계정을 선택하면 판독 결과가 표시됩니다.</p>
    )}
  </aside>
)
