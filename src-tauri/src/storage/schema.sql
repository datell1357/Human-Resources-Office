-- 작업 대상 관리 저장소 스키마
-- 등록 요청 성공(submitted)과 실제 열람 확인(verified)을 서로 다른 상태로 보존한다.

CREATE TABLE IF NOT EXISTS sites (
    id            INTEGER PRIMARY KEY,
    name          TEXT    NOT NULL,
    url           TEXT    NOT NULL UNIQUE,
    category      TEXT    NOT NULL DEFAULT '',
    join_mode     TEXT    NOT NULL DEFAULT 'existing_account',
    adapter       TEXT    NOT NULL DEFAULT 'generic',
    delay_minutes INTEGER NOT NULL DEFAULT 0,
    enabled       INTEGER NOT NULL DEFAULT 1,
    created_at    INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
    id            INTEGER PRIMARY KEY,
    site_id       INTEGER NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    username      TEXT    NOT NULL,
    secret_ref    TEXT    NOT NULL,
    status        TEXT    NOT NULL DEFAULT 'active',
    note          TEXT    NOT NULL DEFAULT '',
    source_line   INTEGER,
    last_login_at INTEGER,
    created_at    INTEGER NOT NULL,
    UNIQUE (site_id, username)
);

CREATE TABLE IF NOT EXISTS posts (
    id          INTEGER PRIMARY KEY,
    title       TEXT    NOT NULL,
    body        TEXT    NOT NULL,
    body_format TEXT    NOT NULL DEFAULT 'text',
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS runs (
    id          INTEGER PRIMARY KEY,
    post_id     INTEGER NOT NULL REFERENCES posts(id),
    state       TEXT    NOT NULL DEFAULT 'running',
    started_at  INTEGER NOT NULL,
    finished_at INTEGER
);

CREATE TABLE IF NOT EXISTS run_items (
    id           INTEGER PRIMARY KEY,
    run_id       INTEGER NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
    site_id      INTEGER NOT NULL REFERENCES sites(id),
    account_id   INTEGER NOT NULL REFERENCES accounts(id),
    seq          INTEGER NOT NULL,
    state        TEXT    NOT NULL DEFAULT 'queued',
    posted_url   TEXT,
    submitted_at INTEGER,
    verified_at  INTEGER,
    visibility   TEXT,
    failure_kind TEXT,
    message      TEXT    NOT NULL DEFAULT '',
    UNIQUE (run_id, site_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_run_items_run_state ON run_items (run_id, state);
CREATE INDEX IF NOT EXISTS idx_accounts_site ON accounts (site_id);
