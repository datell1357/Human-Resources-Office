import { useState } from 'react'
import { PostsView } from './components/PostsView'
import { Sidebar, type ViewKey } from './components/Sidebar'
import { TargetsView } from './components/TargetsView'

export const App = (): React.JSX.Element => {
  const [view, setView] = useState<ViewKey>('targets')

  return (
    <div className="app-shell">
      <Sidebar active={view} onSelect={setView} />
      {view === 'posts' ? <PostsView /> : <TargetsView />}
    </div>
  )
}
