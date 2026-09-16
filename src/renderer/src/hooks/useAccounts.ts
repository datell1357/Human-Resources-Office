import { useCallback, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { createPreviewAccounts } from '../previewAccounts'
import type { AccountLoadResult } from '../types'

export const useAccounts = (): {
  result: AccountLoadResult | null
  loading: boolean
  unexpectedError: string
  reload: () => Promise<void>
} => {
  const [result, setResult] = useState<AccountLoadResult | null>(null)
  const [loading, setLoading] = useState(true)
  const [unexpectedError, setUnexpectedError] = useState('')

  const reload = useCallback(async (): Promise<void> => {
    setLoading(true)
    setUnexpectedError('')
    try {
      if (import.meta.env.DEV && !('__TAURI_INTERNALS__' in window)) {
        setResult(createPreviewAccounts())
        return
      }
      setResult(await invoke<AccountLoadResult>('load_accounts'))
    } catch {
      setUnexpectedError('프로그램과 파일 판독기 사이의 연결에 실패했습니다.')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void reload()
  }, [reload])

  return { result, loading, unexpectedError, reload }
}
