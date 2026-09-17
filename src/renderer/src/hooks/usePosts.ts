import { useCallback, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { isPreview } from '../runtime'
import type { Post, PostInput } from '../types'

let previewPosts: Post[] = [
  {
    id: 1,
    title: '주간 구인 공고',
    body: '이번 주 모집 인원과 근무 조건을 안내합니다.\n\n- 근무지: 서울 강서구\n- 근무 시간: 08:00 ~ 17:00\n- 문의: 사무실 대표번호',
    bodyFormat: 'text',
    createdAt: Date.now() - 86_400_000,
    updatedAt: Date.now() - 3_600_000
  },
  {
    id: 2,
    title: '현장 안전 교육 일정',
    body: '다음 주 안전 교육 일정과 준비물을 정리했습니다.',
    bodyFormat: 'text',
    createdAt: Date.now() - 172_800_000,
    updatedAt: Date.now() - 172_800_000
  }
]

export const usePosts = (): {
  posts: Post[]
  loading: boolean
  error: string
  reload: () => Promise<void>
  savePost: (input: PostInput) => Promise<Post | null>
  deletePost: (id: number) => Promise<boolean>
} => {
  const [posts, setPosts] = useState<Post[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  const reload = useCallback(async (): Promise<void> => {
    setLoading(true)
    setError('')
    try {
      setPosts(isPreview() ? [...previewPosts] : await invoke<Post[]>('list_posts'))
    } catch (cause) {
      setError(String(cause))
    } finally {
      setLoading(false)
    }
  }, [])

  const savePost = useCallback(async (input: PostInput): Promise<Post | null> => {
    setError('')
    try {
      if (isPreview()) {
        const now = Date.now()
        const saved: Post = {
          id: input.id ?? Math.max(0, ...previewPosts.map((post) => post.id)) + 1,
          title: input.title,
          body: input.body,
          bodyFormat: 'text',
          createdAt: previewPosts.find((post) => post.id === input.id)?.createdAt ?? now,
          updatedAt: now
        }
        previewPosts = previewPosts.some((post) => post.id === saved.id)
          ? previewPosts.map((post) => (post.id === saved.id ? saved : post))
          : [...previewPosts, saved]
        setPosts([...previewPosts])
        return saved
      }
      const saved = await invoke<Post>('save_post', { input })
      await reload()
      return saved
    } catch (cause) {
      setError(String(cause))
      return null
    }
  }, [reload])

  const deletePost = useCallback(async (id: number): Promise<boolean> => {
    setError('')
    try {
      if (isPreview()) {
        previewPosts = previewPosts.filter((post) => post.id !== id)
        setPosts([...previewPosts])
        return true
      }
      const removed = await invoke<boolean>('delete_post', { id })
      if (!removed) {
        setError('이미 실행에 사용한 글이라 지울 수 없습니다.')
      }
      await reload()
      return removed
    } catch (cause) {
      setError(String(cause))
      return false
    }
  }, [reload])

  useEffect(() => {
    void reload()
  }, [reload])

  return { posts, loading, error, reload, savePost, deletePost }
}
