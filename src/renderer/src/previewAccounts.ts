import type { AccountLoadResult } from './types'

export const createPreviewAccounts = (): AccountLoadResult => ({
  fileName: 'accounts.txt',
  filePath: '/development-preview/accounts.txt',
  loadedAt: Date.now(),
  state: 'ready',
  fileMessage: '6개 계정 행을 읽고 4개 접속 대상을 선택했습니다.',
  summary: {
    total: 6,
    active: 4,
    error: 1,
    excluded: 1
  },
  rows: [
    {
      id: 'line-4',
      lineNumber: 4,
      enabled: true,
      referenceUrl: 'https://example.com',
      accountLabel: 'use***',
      note: '정상 계정',
      status: 'active',
      decision: '접속 대상',
      validationMessage: '유효한 활성 계정입니다.'
    },
    {
      id: 'line-5',
      lineNumber: 5,
      enabled: true,
      referenceUrl: 'https://shop.example.com',
      accountLabel: 'use***',
      note: '쇼핑몰 계정',
      status: 'active',
      decision: '접속 대상',
      validationMessage: '유효한 활성 계정입니다.'
    },
    {
      id: 'line-6',
      lineNumber: 6,
      enabled: true,
      referenceUrl: 'invalid-url',
      accountLabel: 'use***',
      note: 'URL 오류 예시',
      status: 'error',
      decision: '형식 오류',
      validationMessage: '올바른 URL 형식이 아닙니다.'
    },
    {
      id: 'line-7',
      lineNumber: 7,
      enabled: false,
      referenceUrl: 'https://test.local',
      accountLabel: 'use***',
      note: '수동 제외',
      status: 'excluded',
      decision: '비활성으로 제외',
      validationMessage: 'active 값이 false여서 접속 대상에서 제외했습니다.'
    },
    {
      id: 'line-8',
      lineNumber: 8,
      enabled: true,
      referenceUrl: 'http://example.org',
      accountLabel: 'use***',
      note: 'HTTP 예시',
      status: 'active',
      decision: '접속 대상',
      validationMessage: '유효한 활성 계정입니다.'
    },
    {
      id: 'line-9',
      lineNumber: 9,
      enabled: true,
      referenceUrl: 'https://service.test.local',
      accountLabel: 'use***',
      note: '내부 테스트',
      status: 'active',
      decision: '접속 대상',
      validationMessage: '유효한 활성 계정입니다.'
    }
  ]
})
