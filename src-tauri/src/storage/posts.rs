use crate::now_millis;
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub body_format: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInput {
    pub id: Option<i64>,
    pub title: String,
    pub body: String,
    pub body_format: Option<String>,
}

fn row_to_post(row: &rusqlite::Row<'_>) -> SqlResult<Post> {
    Ok(Post {
        id: row.get("id")?,
        title: row.get("title")?,
        body: row.get("body")?,
        body_format: row.get("body_format")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

pub fn list(connection: &Connection) -> SqlResult<Vec<Post>> {
    let mut statement = connection.prepare(
        "SELECT id, title, body, body_format, created_at, updated_at FROM posts ORDER BY id",
    )?;
    let posts = statement
        .query_map([], |row| row_to_post(row))?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(posts)
}

pub fn get(connection: &Connection, id: i64) -> SqlResult<Option<Post>> {
    connection
        .query_row(
            "SELECT id, title, body, body_format, created_at, updated_at FROM posts WHERE id = ?1",
            [id],
            |row| row_to_post(row),
        )
        .optional()
}

pub fn save(connection: &Connection, input: &PostInput) -> SqlResult<Post> {
    let now = now_millis() as i64;
    let format = input
        .body_format
        .clone()
        .unwrap_or_else(|| "text".to_string());

    let id = match input.id {
        Some(id) => {
            connection.execute(
                "UPDATE posts SET title = ?1, body = ?2, body_format = ?3, updated_at = ?4 WHERE id = ?5",
                params![&input.title, &input.body, &format, now, id],
            )?;
            id
        }
        None => {
            connection.execute(
                "INSERT INTO posts (title, body, body_format, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?4)",
                params![&input.title, &input.body, &format, now],
            )?;
            connection.last_insert_rowid()
        }
    };

    get(connection, id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)
}

/// 실행 이력이 참조하는 글은 지우지 않는다. 지난 결과의 제목이 사라지면 로그를 읽을 수 없다.
pub fn delete(connection: &Connection, id: i64) -> SqlResult<bool> {
    let referenced: i64 = connection.query_row(
        "SELECT COUNT(*) FROM runs WHERE post_id = ?1",
        [id],
        |row| row.get(0),
    )?;
    if referenced > 0 {
        return Ok(false);
    }
    connection.execute("DELETE FROM posts WHERE id = ?1", [id])?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Database;

    fn input(title: &str, body: &str) -> PostInput {
        PostInput {
            id: None,
            title: title.to_string(),
            body: body.to_string(),
            body_format: None,
        }
    }

    #[test]
    fn saving_without_an_id_creates_and_saving_with_one_updates() {
        let database = Database::open_in_memory().unwrap();
        let connection = database.connection();

        let created = save(connection, &input("첫 글", "본문")).unwrap();
        assert_eq!(created.id, 1);

        let updated = save(
            connection,
            &PostInput {
                id: Some(created.id),
                title: "고친 글".to_string(),
                body: "새 본문".to_string(),
                body_format: None,
            },
        )
        .unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.title, "고친 글");
        assert_eq!(list(connection).unwrap().len(), 1);
    }

    #[test]
    fn refuses_to_delete_a_post_that_a_run_still_references() {
        let database = Database::open_in_memory().unwrap();
        let connection = database.connection();
        let post = save(connection, &input("실행에 쓴 글", "본문")).unwrap();

        connection
            .execute(
                "INSERT INTO runs (post_id, started_at) VALUES (?1, 1)",
                [post.id],
            )
            .unwrap();

        assert!(!delete(connection, post.id).unwrap());
        assert_eq!(list(connection).unwrap().len(), 1);
    }

    #[test]
    fn deletes_a_post_that_no_run_references() {
        let database = Database::open_in_memory().unwrap();
        let connection = database.connection();
        let post = save(connection, &input("임시 글", "본문")).unwrap();

        assert!(delete(connection, post.id).unwrap());
        assert!(list(connection).unwrap().is_empty());
    }
}
