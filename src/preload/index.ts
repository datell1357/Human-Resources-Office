import { contextBridge } from 'electron'

contextBridge.exposeInMainWorld('accounts', {
  load: async () => ({ status: 'scaffold' })
})
