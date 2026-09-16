import { ClockIcon, SettingsIcon, TableIcon } from './Icons'

export const Sidebar = (): React.JSX.Element => (
  <aside className="sidebar">
    <div className="brand">작업 대상 관리</div>
    <nav aria-label="주요 메뉴" className="navigation">
      <button aria-current="page" className="nav-item active" type="button">
        <TableIcon />
        <span>대상 목록</span>
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
      <p>파일 판독만 수행하며 외부 사이트에는 접속하지 않습니다.</p>
    </div>
  </aside>
)
