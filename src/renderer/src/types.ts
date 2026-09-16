export type AccountStatus = 'active' | 'excluded' | 'error'

export interface AccountRow {
  id: string
  lineNumber: number
  enabled: boolean | null
  referenceUrl: string
  accountLabel: string
  note: string
  status: AccountStatus
  decision: string
  validationMessage: string
}

export interface AccountSummary {
  total: number
  active: number
  error: number
  excluded: number
}

export interface AccountLoadResult {
  fileName: string
  filePath: string
  loadedAt: number
  state: 'ready' | 'missing' | 'unreadable'
  fileMessage: string
  summary: AccountSummary
  rows: AccountRow[]
}
