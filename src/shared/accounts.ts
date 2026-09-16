export type AccountStatus = 'active' | 'excluded' | 'error'

export interface ParsedAccount {
  lineNumber: number
  enabled: boolean | null
  referenceUrl: string
  username: string
  password: string
  note: string
  status: AccountStatus
  decision: string
  validationMessage: string
}

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
  loadedAt: string
  state: 'ready' | 'missing' | 'unreadable'
  fileMessage: string
  summary: AccountSummary
  rows: AccountRow[]
}

const TRUE_VALUES = new Set(['true', 'yes', 'y', '1', 'active', 'on'])
const FALSE_VALUES = new Set(['false', 'no', 'n', '0', 'inactive', 'off'])

const parseEnabled = (value: string): boolean | null => {
  const normalized = value.trim().toLowerCase()
  if (TRUE_VALUES.has(normalized)) return true
  if (FALSE_VALUES.has(normalized)) return false
  return null
}

const validateUrl = (value: string): string | null => {
  try {
    const parsed = new URL(value)
    if (!['http:', 'https:'].includes(parsed.protocol)) {
      return '참조 주소는 http:// 또는 https://로 시작해야 합니다.'
    }
    if (!parsed.hostname) return '참조 주소에 호스트 이름이 없습니다.'
    return null
  } catch {
    return '올바른 URL 형식이 아닙니다.'
  }
}

const duplicateKey = (referenceUrl: string, username: string): string => {
  try {
    return `${new URL(referenceUrl).href}|${username}`
  } catch {
    return `${referenceUrl}|${username}`
  }
}

export const maskAccount = (username: string): string => {
  if (!username) return '—'
  if (username.length === 1) return '*'
  const visibleLength = Math.min(3, Math.max(1, username.length - 2))
  return `${username.slice(0, visibleLength)}***`
}

export const parseAccounts = (content: string): ParsedAccount[] => {
  const seenActiveAccounts = new Set<string>()
  const parsed: ParsedAccount[] = []

  content
    .replace(/^\uFEFF/, '')
    .split(/\r?\n/)
    .forEach((rawLine, index) => {
      const lineNumber = index + 1
      const line = rawLine.trim()
      if (!line || line.startsWith('#')) return

      const fields = line.split('|').map((field) => field.trim())
      const [enabledText = '', referenceUrl = '', username = '', password = '', ...noteParts] = fields
      const note = noteParts.join(' | ')
      const enabled = parseEnabled(enabledText)
      let validationMessage = ''

      if (fields.length < 4) {
        validationMessage = '필드는 active | reference_url | username | password 순서로 4개 이상 필요합니다.'
      } else if (enabled === null) {
        validationMessage = 'active 값은 true 또는 false 형식이어야 합니다.'
      } else {
        validationMessage = validateUrl(referenceUrl) ?? ''
        if (!validationMessage && !username) validationMessage = '계정 아이디가 비어 있습니다.'
        if (!validationMessage && !password) validationMessage = '비밀번호가 비어 있습니다.'
      }

      let status: AccountStatus = 'error'
      let decision = '형식 오류'

      if (!validationMessage && enabled === false) {
        status = 'excluded'
        decision = '비활성으로 제외'
        validationMessage = 'active 값이 false여서 접속 대상에서 제외했습니다.'
      } else if (!validationMessage && enabled === true) {
        const key = duplicateKey(referenceUrl, username)
        if (seenActiveAccounts.has(key)) {
          validationMessage = '앞선 활성 행과 참조 주소·계정이 중복됩니다.'
          decision = '중복 오류'
        } else {
          seenActiveAccounts.add(key)
          status = 'active'
          decision = '접속 대상'
          validationMessage = '유효한 활성 계정입니다.'
        }
      }

      parsed.push({
        lineNumber,
        enabled,
        referenceUrl,
        username,
        password,
        note,
        status,
        decision,
        validationMessage
      })
    })

  return parsed
}

export const toAccountRows = (accounts: ParsedAccount[]): AccountRow[] =>
  accounts.map(({ password: _password, username, ...account }) => ({
    ...account,
    id: `line-${account.lineNumber}`,
    accountLabel: maskAccount(username)
  }))

export const summarizeAccounts = (accounts: ParsedAccount[]): AccountSummary => ({
  total: accounts.length,
  active: accounts.filter((account) => account.status === 'active').length,
  error: accounts.filter((account) => account.status === 'error').length,
  excluded: accounts.filter((account) => account.status === 'excluded').length
})

export const getConnectionTargets = (accounts: ParsedAccount[]): ParsedAccount[] =>
  accounts.filter((account) => account.status === 'active')
