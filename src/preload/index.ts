import { contextBridge, ipcRenderer } from 'electron'
import type { AccountLoadResult } from '../shared/accounts'

contextBridge.exposeInMainWorld('accounts', {
  load: (): Promise<AccountLoadResult> => ipcRenderer.invoke('accounts:load')
})
