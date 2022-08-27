use crate::model::DataAccessObject;
use crate::model::Entry;

use sqlite::{open, Connection, Row, Value};

pub struct SQLiteDAO {
    path: String,
}

impl SQLiteDAO {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }

    fn make_connection(&self) -> Result<Connection, sqlite::Error> {
        open(&self.path)
    }
}

impl Default for SQLiteDAO {
    fn default() -> Self {
        Self {
            path: ":memory:".to_owned(),
        }
    }
}

const ENTRY_TABLE_NAME: &'static str = "entry";

fn row_to_entry(row: Row) -> Result<Entry, sqlite::Error> {
    Ok(Entry {
        id: row.try_get(0)?,
        timestamp: row.try_get::<i64, _>(1)? as u32,
        content: row.try_get(2)?,
        priority: row.try_get::<i64, _>(3)? as u16,
    })
}

impl DataAccessObject<Entry> for SQLiteDAO {
    type Identity = String;
    type Error = sqlite::Error;

    fn get_one(&self, id: Self::Identity) -> Result<Option<Entry>, Self::Error> {
        self.make_connection()?
            .prepare(format!("SELECT * FROM {} WHERE id = ?;", ENTRY_TABLE_NAME))?
            .into_cursor()
            .bind(&[Value::String(id)])?
            .next()
            .transpose()?
            .map(row_to_entry)
            .transpose()
    }

    fn get_all(&self) -> Result<Vec<Entry>, Self::Error> {
        self.make_connection()?
            .prepare(format!("SELECT * FROM {};", ENTRY_TABLE_NAME))?
            .into_cursor()
            .map(|rr| match rr {
                Ok(r) => Ok(row_to_entry(r)?), // makes it fail early
                Err(err) => Err(err),
            })
            .collect()
    }

    fn save(&self, item: Entry) -> Result<Self::Identity, Self::Error> {
        let Entry {
            id,
            timestamp,
            content,
            priority,
        } = item;

        self.make_connection()?
            .prepare(format!(
                "INSERT INTO {} VALUES (?, ?, ?, ?);",
                ENTRY_TABLE_NAME
            ))?  
            .into_cursor()
            .bind(&[
                Value::String(id.clone()),
                Value::Integer(timestamp as i64),
                Value::String(content),
                Value::Integer(priority as i64),
            ])?
            .last();

        todo!("or update");
        Ok(id)
    }
}
