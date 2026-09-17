import { useEffect, useState } from 'react'
import { AlertIcon, FileIcon } from './Icons'
import { usePosts } from '../hooks/usePosts'

const formatTime = (value: number): string =>
  new Date(value).toLocaleString('ko-KR', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })

export const PostsView = (): React.JSX.Element => {
  const { posts, loading, error, savePost, deletePost } = usePosts()
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [title, setTitle] = useState('')
  const [body, setBody] = useState('')
  const [dirty, setDirty] = useState(false)
  const [composing, setComposing] = useState(false)

  const selected = composing ? null : (posts.find((post) => post.id === selectedId) ?? null)

  useEffect(() => {
    if (composing) return
    if (!posts.length) {
      setSelectedId(null)
      return
    }
    if (!posts.some((post) => post.id === selectedId)) {
      setSelectedId(posts[0].id)
    }
  }, [composing, posts, selectedId])

  useEffect(() => {
    if (composing) return
    setTitle(selected?.title ?? '')
    setBody(selected?.body ?? '')
    setDirty(false)
  }, [composing, selected])

  const startNewPost = (): void => {
    setComposing(true)
    setSelectedId(null)
    setTitle('')
    setBody('')
    setDirty(false)
  }

  const selectPost = (id: number): void => {
    setComposing(false)
    setSelectedId(id)
  }

  const submit = async (): Promise<void> => {
    const saved = await savePost({ id: selected?.id, title, body })
    if (saved) {
      setComposing(false)
      setSelectedId(saved.id)
      setDirty(false)
    }
  }

  const remove = async (): Promise<void> => {
    if (!selected) return
    await deletePost(selected.id)
  }

  const canSave = title.trim().length > 0 && body.trim().length > 0 && dirty

  return (
    <>
      <main className="workspace">
        <header className="workspace-header">
          <div>
            <h1>게시글</h1>
            <span className="file-state ready">
              <span aria-hidden="true" className="file-state-dot" />
              저장한 글 {posts.length}개
            </span>
          </div>
        </header>

        <section aria-label="저장한 게시글" className="content-area posts-area">
          <div className="table-panel">
            {loading ? (
              <div className="loading-state">게시글을 불러오고 있습니다.</div>
            ) : posts.length === 0 ? (
              <div className="empty-table">저장한 글이 없습니다. 새 글을 작성해 보세요.</div>
            ) : (
              <div className="table-scroll">
                <table>
                  <thead>
                    <tr>
                      <th>번호</th>
                      <th>제목</th>
                      <th>본문 미리보기</th>
                      <th>수정</th>
                    </tr>
                  </thead>
                  <tbody>
                    {posts.map((post) => (
                      <tr
                        aria-selected={post.id === selectedId}
                        className={post.id === selectedId ? 'selected' : ''}
                        key={post.id}
                        onClick={() => selectPost(post.id)}
                        onKeyDown={(event) => {
                          if (event.key === 'Enter' || event.key === ' ') selectPost(post.id)
                        }}
                        tabIndex={0}
                      >
                        <td>{post.id}</td>
                        <td className="url-cell" title={post.title}>
                          {post.title}
                        </td>
                        <td className="note-cell" title={post.body}>
                          {post.body.replace(/\s+/g, ' ').slice(0, 60) || '—'}
                        </td>
                        <td>{formatTime(post.updatedAt)}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>

          {error && (
            <div className="error-banner" role="alert">
              <AlertIcon />
              <div>
                <strong>게시글을 저장하지 못했습니다</strong>
                <p>{error}</p>
              </div>
            </div>
          )}
        </section>

        <footer className="status-bar">
          <div>
            <FileIcon />
            <span>workspace.db</span>
            <span aria-hidden="true">·</span>
            <span>{posts.length}개 글</span>
          </div>
          <button onClick={startNewPost} type="button">
            새 글
          </button>
        </footer>
      </main>

      <aside className="detail-panel">
        <header>
          <h2>{selected ? `${selected.id}번 글 편집` : '새 글 작성'}</h2>
        </header>
        <div className="detail-content">
          <div className="field-group">
            <label htmlFor="post-title">제목</label>
            <input
              id="post-title"
              onChange={(event) => {
                setTitle(event.target.value)
                setDirty(true)
              }}
              placeholder="게시글 제목"
              type="text"
              value={title}
            />
          </div>
          <div className="field-group">
            <label htmlFor="post-body">본문</label>
            <textarea
              id="post-body"
              onChange={(event) => {
                setBody(event.target.value)
                setDirty(true)
              }}
              placeholder="게시할 본문을 입력합니다."
              rows={14}
              value={body}
            />
          </div>
          <div className="editor-actions">
            <button className="primary" disabled={!canSave} onClick={() => void submit()} type="button">
              저장
            </button>
            <button disabled={!selected} onClick={() => void remove()} type="button">
              삭제
            </button>
          </div>
          {selected && (
            <p className="detail-empty">마지막 수정 {formatTime(selected.updatedAt)}</p>
          )}
        </div>
      </aside>
    </>
  )
}
