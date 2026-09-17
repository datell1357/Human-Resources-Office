import { ClockIcon, FileIcon, SettingsIcon, TableIcon } from './Icons'

export type ViewKey = 'targets' | 'posts'

interface SidebarProps {
  active: ViewKey
  onSelect: (view: ViewKey) => void
}

export const Sidebar = ({ active, onSelect }: SidebarProps): React.JSX.Element => (
  <aside className="sidebar">
    <div className="brand">작업 대상 관리</div>
    <nav aria-label="주요 메뉴" className="navigation">
      <button
        aria-current={active === 'targets' ? 'page' : undefined}
        className={`nav-item ${active === 'targets' ? 'active' : ''}`}
        onClick={() => onSelect('targets')}
        type="button"
      >
        <TableIcon />
        <span>대상 목록</span>
      </button>
      <button
        aria-current={active === 'posts' ? 'page' : undefined}
        className={`nav-item ${active === 'posts' ? 'active' : ''}`}
        onClick={() => onSelect('posts')}
        type="button"
      >
        <FileIcon />
        <span>게시글</span>
      </button>
      <button className="nav-item" disabled type="button">
        <ClockIcon />
        <span>실행 기록</span>
        <small>준비 중</small>
      </button>
      <button className="nav-item" disabled type="button">
        <SettingsIcon />
        <span>설정</span>
        <small>준비 중</small>
      </button>
    </nav>
    <div className="sidebar-note">
      <span>초안 모드</span>
      <p>파일 판독과 게시글 보관만 수행하며 외부 사이트에는 접속하지 않습니다.</p>
    </div>
  </aside>
)
