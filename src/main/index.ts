import { app, BrowserWindow, ipcMain } from 'electron'
import { dirname, join, resolve } from 'node:path'
import { existsSync } from 'node:fs'
import { loadAccountFile } from './account-file'

const resolveAccountsPath = (): string => {
  const override = process.env.ACCOUNTS_FILE?.trim()
  if (override) return resolve(override)

  if (!app.isPackaged) return resolve(process.cwd(), 'accounts.txt')

  const portablePath = join(dirname(process.execPath), 'accounts.txt')
  if (existsSync(portablePath)) return portablePath
  return join(app.getPath('userData'), 'accounts.txt')
}

const createWindow = (): void => {
  const window = new BrowserWindow({
    width: 1440,
    height: 900,
    minWidth: 1040,
    minHeight: 680,
    backgroundColor: '#ffffff',
    webPreferences: {
      preload: join(__dirname, '../preload/index.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  if (process.env.ELECTRON_RENDERER_URL) {
    void window.loadURL(process.env.ELECTRON_RENDERER_URL)
  } else {
    void window.loadFile(join(__dirname, '../renderer/index.html'))
  }
}

app.whenReady().then(() => {
  ipcMain.handle('accounts:load', async () => {
    const { result } = await loadAccountFile(resolveAccountsPath())
    return result
  })

  createWindow()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})
