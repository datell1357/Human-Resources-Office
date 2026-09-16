import type { AccountLoadResult } from '../../shared/accounts'

declare global {
  interface Window {
    accounts: {
      load: () => Promise<AccountLoadResult>
    }
  }
}

export {}
