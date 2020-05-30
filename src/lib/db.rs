use super::event::Event;
use super::opts::{CalOpts, ListOpts};
use chrono::offset::TimeZone;
use chrono::{Datelike, Local, NaiveDateTime};
use rusqlite::{params, Connection, Result, NO_PARAMS};

pub trait Writeable: Sized {
    fn write(&self) -> Result<()>;
    fn read(opts: &ListOpts) -> Result<Vec<Self>>;
    fn create(opts: &CalOpts) -> Result<()>;
    fn remove(opts: &CalOpts) -> Result<()>;
}

impl Writeable for Event {
    fn remove(opts: &CalOpts) -> Result<()> {
        let con = Connection::open("events.db")?;
        con.execute(
            format!("DROP TABLE {};", opts.calendar_name).as_str(),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn create(opts: &CalOpts) -> Result<()> {
        let con = Connection::open("events.db")?;
        con.execute(
            format!(
                "CREATE TABLE {} (
                id INTEGER PRIMARY KEY UNIQUE,
                title TEXT NOT NULL,
                start INTEGER NOT NULL,
                end INTEGER,
                description TEXT,
                UNIQUE(title, start)
            );",
                opts.calendar_name
            )
            .as_str(),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let con = Connection::open("events.db")?;
        let end = match &self.end {
            Some(d) => Some(d.timestamp().to_string()),
            None => None,
        };
        let desc = match &self.description {
            Some(d) => Some(d.to_owned()),
            None => None,
        };
        con.execute(format!(
            "INSERT INTO {} (title, start, end, description)
                    VALUES (?1, ?2, ?3, ?4)", &self.calendar_name).as_str(),
            params![
                &self.title,
                &(self.start).timestamp().to_string(),
                &end,
                &desc
            ],
        )?;
        Ok(())
    }

    fn read(opts: &ListOpts) -> Result<Vec<Self>> {
        let con = Connection::open("events.db")?;
        let mut query = con.prepare(
            format!(
                "SELECT title, start, end, description FROM {}",
                &opts.calendar_name
            )
            .as_str(),
        )?;
        let rows = query
            .query_map(NO_PARAMS, |row| {
                Ok(Event {
                    calendar_name: String::from(&opts.calendar_name),
                    title: row.get(0)?,
                    start: Local
                        .from_local_datetime(&NaiveDateTime::from_timestamp(row.get(1)?, 0))
                        .unwrap(),
                    end: match row.get(2)? {
                        Some(ts) => Some(
                            Local
                                .from_local_datetime(&NaiveDateTime::from_timestamp(ts, 0))
                                .unwrap(),
                        ),
                        None => None,
                    },
                    description: row.get(3)?,
                })
            })?
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<Self>>();
        let today = Local::today();
        match (opts.all, opts.today, opts.week, opts.month) {
            (true, _, _, _) => Ok(rows),
            (false, true, _, _) => Ok(rows
                .into_iter()
                .filter(|x| x.start.date() == today)
                .collect::<Vec<Self>>()),
            (false, false, true, _) => Ok(rows
                .into_iter()
                .filter(|x| {
                    x.start.date().iso_week() == today.iso_week()
                        && x.start.date().year() == today.year()
                })
                .collect::<Vec<Self>>()),
            (false, false, false, true) => Ok(rows
                .into_iter()
                .filter(|x| {
                    x.start.date().month() == today.month()
                        && x.start.date().year() == today.year()})
                .collect::<Vec<Self>>()),
            _ => Ok(rows
                .into_iter()
                .filter(|x| x.start.date() == today)
                .collect::<Vec<Self>>()),
        }
    }
}
