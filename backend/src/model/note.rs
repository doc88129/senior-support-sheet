use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use strum::EnumString;
use surrealdb::opt::auth::Jwt;
use super::{database::Database, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pid: i64,
    created_at: DateTime<Utc>,
    note_type: NoteType,
    notes: Vec<ContentNote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentNote {
    created_at: DateTime<Utc>,
    created_by: i64,
    created_by_name: String,
    content: String,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum NoteType {
    Informational,
    Warning,
    Removal,
    Blacklist,
}

pub struct NoteMac;

impl NoteMac {
    pub async fn create(
        database: &Database,
        token: Jwt,
        creator: User, 
        user: User, 
        note_type: NoteType, 
        content: String
    ) -> Result<Note, surrealdb::Error> {
        database.authenticate(token).await?;
        let note = database
            .create(("note", format!("{}-{}", user.pid, Utc::now().timestamp())))
            .content(Note {
                pid: user.pid,
                created_at: Utc::now(),
                note_type,
                notes: vec![ContentNote {
                    created_at: Utc::now(),
                    created_by: creator.pid,
                    created_by_name: creator.name,
                    content: content.to_string(),
                }],
            })
            .await?;
        Ok(note)
    }

    pub async fn add(
        database: &Database,
        token: Jwt,
        creator: User, 
        user: User, 
        mut note: Note, 
        content: String
    ) -> Result<Note, surrealdb::Error> {
        database.authenticate(token).await?;
        note.notes.push(ContentNote {
            created_at: Utc::now(),
            created_by: creator.pid,
            created_by_name: creator.name,
            content: content.to_string(),
        });
        let note = database
            .update(("note", format!("{}-{}", user.pid, note.created_at.timestamp())))
            .content(note)
            .await?;
        Ok(note)
    }

    pub async fn fetch_one(
        database: &Database, 
        token: Jwt,
        pid: i64, 
        date: DateTime<Utc>
    ) -> Result<Note, surrealdb::Error> {
        database.authenticate(token).await?;
        let note = database
            .select(("note", format!("{}-{}", pid, date.timestamp())))
            .await?;
        Ok(note)
    }

    pub async fn fetch_all(
        database: &Database, 
        token: Jwt,
        pid: i64
    ) -> Result<Vec<Note>, surrealdb::Error> {
        database.authenticate(token).await?;
        let notes = database
            .query("SELECT * FROM note WHERE pid = $pid")
            .bind(("pid", pid))
            .await?
            .take(0)?;
        Ok(notes)
    }

    pub async fn edit(
        database: &Database,
        token: Jwt,
        creator: User,
        date: DateTime<Utc>,
        mut note: Note,
        content: String
    ) -> Result<Note, surrealdb::Error> {
        database.authenticate(token).await?;
        note.notes
            .iter_mut()
            .find(|n| n.created_at == date)
            .unwrap()
            .content = content.to_string();
        let note = database
            .update(("note", format!("{}-{}", creator.pid, note.created_at.timestamp())))
            .content(note)
            .await?;
        Ok(note)
    }
}