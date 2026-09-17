/** Tauri 런타임 없이 브라우저에서 열었을 때만 미리보기 데이터를 쓴다. */
export const isPreview = (): boolean =>
  import.meta.env.DEV && !('__TAURI_INTERNALS__' in window)
