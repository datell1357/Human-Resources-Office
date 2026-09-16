import { readFile } from 'node:fs/promises'
import { basename } from 'node:path'
import {
  parseAccounts,
  summarizeAccounts,
  toAccountRows,
  type AccountLoadResult,
  type ParsedAccount
} from '../shared/accounts'

export interface LoadedAccountFile {
  result: AccountLoadResult
  connectionTargets: ParsedAccount[]
}

const emptySummary = { total: 0, active: 0, error: 0, excluded: 0 }

export const loadAccountFile = async (filePath: string): Promise<LoadedAccountFile> => {
  const base = {
    fileName: basename(filePath),
    filePath,
    loadedAt: new Date().toISOString()
  }

  try {
    const content = await readFile(filePath, 'utf8')
    const accounts = parseAccounts(content)
    const connectionTargets = accounts.filter((account) => account.status === 'active')

    return {
      connectionTargets,
      result: {
        ...base,
        state: 'ready',
        fileMessage: `${accounts.length}개 계정 행을 읽었습니다.`,
        summary: summarizeAccounts(accounts),
        rows: toAccountRows(accounts)
      }
    }
  } catch (error) {
    const missing = error instanceof Error && 'code' in error && error.code === 'ENOENT'
    return {
      connectionTargets: [],
      result: {
        ...base,
        state: missing ? 'missing' : 'unreadable',
        fileMessage: missing
          ? 'accounts.txt를 찾지 못했습니다.'
          : 'accounts.txt를 읽는 중 오류가 발생했습니다.',
        summary: emptySummary,
        rows: []
      }
    }
  }
}
