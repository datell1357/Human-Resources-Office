import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { loadAccountFile } from './account-file'

describe('accounts.txt file loader', () => {
  it('예제 파일을 실제 IPC 응답 형태로 변환하고 비밀번호를 제거한다', async () => {
    const loaded = await loadAccountFile(resolve(process.cwd(), 'accounts.example.txt'))
    const serializedResult = JSON.stringify(loaded.result)

    expect(loaded.result.state).toBe('ready')
    expect(loaded.result.summary).toEqual({ total: 6, active: 4, error: 1, excluded: 1 })
    expect(loaded.connectionTargets).toHaveLength(4)
    expect(serializedResult).not.toContain('change-me')
    expect(serializedResult).not.toContain('user01')
  })
})
