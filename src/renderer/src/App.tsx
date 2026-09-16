import { useEffect, useMemo, useState } from 'react'
import type { AccountRow } from './types'
import { AlertIcon, FileIcon } from './components/Icons'
import { DetailPanel } from './components/DetailPanel'
import { Sidebar } from './components/Sidebar'
import { SummaryStrip, type StatusFilter } from './components/SummaryStrip'
import { TargetTable } from './components/TargetTable'
import { useAccounts } from './hooks/useAccounts'

export const App = (): React.JSX.Element => {
  const { result, loading, unexpectedError, reload } = useAccounts()
  const [filter, setFilter] = useState<StatusFilter>('all')
  const [selectedId, setSelectedId] = useState('')

  const filteredRows = useMemo(() => {
    if (!result) return []
    return filter === 'all' ? result.rows : result.rows.filter((row) => row.status === filter)
  }, [filter, result])

  const selectedRow = result?.rows.find((row) => row.id === selectedId) ?? null
  const firstError = result?.rows.find((row) => row.status === 'error') ?? null

  useEffect(() => {
    if (!result?.rows.length) {
      setSelectedId('')
      return
    }
    const currentExists = result.rows.some((row) => row.id === selectedId)
    if (!currentExists) {
      setSelectedId((result.rows.find((row) => row.status === 'active') ?? result.rows[0]).id)
    }
  }, [result, selectedId])

  const selectFilter = (nextFilter: StatusFilter): void => {
    setFilter(nextFilter)
    const nextRows = nextFilter === 'all'
      ? result?.rows
      : result?.rows.filter((row) => row.status === nextFilter)
    if (nextRows?.length) setSelectedId(nextRows[0].id)
  }

  const selectRow = (row: AccountRow): void => setSelectedId(row.id)

  return (
    <div className="app-shell">
      <Sidebar />
      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>접속 대상</h1>
            <span className={`file-state ${result?.state ?? 'loading'}`}>
              <span aria-hidden="true" className="file-state-dot" />
              {loading ? 'accounts.txt 읽는 중' : result?.state === 'ready' ? 'accounts.txt 읽음' : '파일 확인 필요'}
            </span>
          </div>
          {result?.loadedAt && (
            <time dateTime={new Date(result.loadedAt).toISOString()}>
              마지막 판독 {new Date(result.loadedAt).toLocaleTimeString('ko-KR', { hour: '2-digit', minute: '2-digit' })}
            </time>
          )}
        </header>

        <section aria-label="계정 파일 판독 결과" className="content-area">
          {result && <SummaryStrip onSelect={selectFilter} selected={filter} summary={result.summary} />}

          <div className="table-panel">
            {loading && !result ? (
              <div className="loading-state">accounts.txt를 읽고 있습니다.</div>
            ) : result?.state !== 'ready' || unexpectedError ? (
              <div className="file-error-state">
                <AlertIcon />
                <h2>계정 파일을 읽지 못했습니다</h2>
                <p>{unexpectedError || result?.fileMessage}</p>
                {result?.filePath && <code>{result.filePath}</code>}
              </div>
            ) : (
              <TargetTable onSelect={selectRow} rows={filteredRows} selectedId={selectedId} />
            )}
          </div>

          {firstError && result?.state === 'ready' && filter !== 'active' && filter !== 'excluded' && (
            <div className="error-banner" role="alert">
              <AlertIcon />
              <div>
                <strong>{firstError.lineNumber}번째 줄 판독 오류</strong>
                <p>
                  {firstError.referenceUrl ? `'${firstError.referenceUrl}' — ` : ''}
                  {firstError.validationMessage}
                </p>
              </div>
              <span>라인 {firstError.lineNumber}</span>
            </div>
          )}
        </section>

        <footer className="status-bar">
          <div title={result?.filePath}>
            <FileIcon />
            <span>{result?.fileName ?? 'accounts.txt'}</span>
            <span aria-hidden="true">·</span>
            <span>{result?.summary.total ?? 0}개 계정 행</span>
          </div>
          <button disabled={loading} onClick={() => void reload()} type="button">
            {loading ? '읽는 중…' : '파일 다시 읽기'}
          </button>
        </footer>
      </main>
      <DetailPanel row={selectedRow} />
    </div>
  )
}
