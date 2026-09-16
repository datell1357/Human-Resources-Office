import { describe, expect, it } from 'vitest'
import {
  getConnectionTargets,
  parseAccounts,
  summarizeAccounts,
  toAccountRows
} from './accounts'

describe('accounts.txt parser', () => {
  it('활성·제외·오류 행을 분리해 접속 대상만 반환한다', () => {
    const accounts = parseAccounts(`
# comment
true | https://example.com | worker01 | secret-one | 정상
false | https://disabled.example | worker02 | secret-two | 제외
true | invalid-url | worker03 | secret-three | 오류
`)

    expect(summarizeAccounts(accounts)).toEqual({ total: 3, active: 1, error: 1, excluded: 1 })
    expect(getConnectionTargets(accounts).map((account) => account.referenceUrl)).toEqual([
      'https://example.com'
    ])
  })

  it('필수 필드와 활성 값, 프로토콜을 검사한다', () => {
    const accounts = parseAccounts(`
maybe | https://example.com | user | password
true | ftp://example.com | user | password
true | https://example.com | | password
true | https://example.com | user |
`)

    expect(accounts.every((account) => account.status === 'error')).toBe(true)
    expect(accounts.map((account) => account.validationMessage)).toEqual([
      'active 값은 true 또는 false 형식이어야 합니다.',
      '참조 주소는 http:// 또는 https://로 시작해야 합니다.',
      '계정 아이디가 비어 있습니다.',
      '비밀번호가 비어 있습니다.'
    ])
  })

  it('같은 활성 URL과 계정의 중복을 두 번째 행에서 제외한다', () => {
    const accounts = parseAccounts(`
true | https://example.com | worker | first-secret
true | https://example.com | worker | second-secret
`)

    expect(accounts.map((account) => account.status)).toEqual(['active', 'error'])
    expect(accounts[1].validationMessage).toContain('중복')
  })

  it('렌더러로 보내는 행에는 아이디 원문과 비밀번호를 포함하지 않는다', () => {
    const rows = toAccountRows(
      parseAccounts('true | https://example.com | private-user | never-expose-this')
    )
    const serialized = JSON.stringify(rows)

    expect(rows[0].accountLabel).toBe('pri***')
    expect(serialized).not.toContain('private-user')
    expect(serialized).not.toContain('never-expose-this')
  })
})
