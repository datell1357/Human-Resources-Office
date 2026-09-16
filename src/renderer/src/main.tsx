import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

const App = (): React.JSX.Element => (
  <main>
    <h1>작업 대상 관리</h1>
    <p>accounts.txt 읽기 기능을 준비하고 있습니다.</p>
  </main>
)

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
