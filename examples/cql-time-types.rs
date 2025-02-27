// An example showing how to use time related types in queries
// Date, Time, Timestamp

use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use scylla::frame::response::result::CqlValue;
use scylla::frame::value::{CqlDate, CqlTime, CqlTimestamp};
use scylla::transport::session::{IntoTypedRows, Session};
use scylla::SessionBuilder;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    println!("Connecting to {} ...", uri);

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await?;

    // Date
    // Date is a year, month and day in the range -5877641-06-23 to -5877641-06-23

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.dates (d date primary key)",
            &[],
        )
        .await?;

    // If 'chrono' feature is enabled, dates in the range -262145-1-1 to 262143-12-31 can be represented using
    // chrono::NaiveDate
    let chrono_date = NaiveDate::from_ymd_opt(2020, 2, 20).unwrap();

    session
        .query("INSERT INTO ks.dates (d) VALUES (?)", (chrono_date,))
        .await?;

    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows.into_typed::<(NaiveDate,)>() {
            let (read_date,): (NaiveDate,) = match row {
                Ok(read_date) => read_date,
                Err(_) => continue, // We might read a date that does not fit in NaiveDate, skip it
            };

            println!("Parsed a date into chrono::NaiveDate: {:?}", read_date);
        }
    }

    // Alternatively, you can enable 'time' feature and use `time::Date` to represent date. `time::Date` only allows
    // dates in range -9999-1-1 to 9999-12-31. Or, if you have 'time/large-dates' feature enabled, this range changes
    // to -999999-1-1 to 999999-12-31
    let time_date = time::Date::from_calendar_date(2020, time::Month::March, 21).unwrap();

    session
        .query("INSERT INTO ks.dates (d) VALUES (?)", (time_date,))
        .await?;

    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows.into_typed::<(time::Date,)>() {
            let (read_date,) = match row {
                Ok(read_date) => read_date,
                Err(_) => continue, // We might read a date that does not fit in time::Date, skip it
            };

            println!("Parsed a date into time::Date: {:?}", read_date);
        }
    }

    // Dates outside this range must be represented in the raw form - an u32 describing days since -5877641-06-23
    let example_big_date: CqlDate = CqlDate(u32::MAX);
    session
        .query("INSERT INTO ks.dates (d) VALUES (?)", (example_big_date,))
        .await?;

    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows {
            let read_days: u32 = match row.columns[0] {
                Some(CqlValue::Date(CqlDate(days))) => days,
                _ => panic!("oh no"),
            };

            println!("Read a date as raw days: {}", read_days);
        }
    }

    // Time
    // Time is represented as nanosecond count since midnight in range 0..=86399999999999

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.times (t time primary key)",
            &[],
        )
        .await?;

    // Time can be represented using 3 different types, chrono::NaiveTime, time::Time and CqlTime. All types support
    // full value range

    // chrono::NaiveTime
    let chrono_time = NaiveTime::from_hms_nano_opt(1, 2, 3, 456_789_012).unwrap();

    session
        .query("INSERT INTO ks.times (t) VALUES (?)", (chrono_time,))
        .await?;

    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(NaiveTime,)>() {
            let (read_time,) = row?;

            println!("Parsed a time into chrono::NaiveTime: {:?}", read_time);
        }
    }

    // time::Time
    let time_time = time::Time::from_hms_nano(2, 3, 4, 567_890_123).unwrap();

    session
        .query("INSERT INTO ks.times (t) VALUES (?)", (time_time,))
        .await?;

    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(time::Time,)>() {
            let (read_time,) = row?;

            println!("Parsed a time into time::Time: {:?}", read_time);
        }
    }

    // CqlTime
    let time_time = CqlTime(((3 * 60 + 4) * 60 + 5) * 1_000_000_000 + 678_901_234);

    session
        .query("INSERT INTO ks.times (t) VALUES (?)", (time_time,))
        .await?;

    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(CqlTime,)>() {
            let (read_time,) = row?;

            println!("Read a time as raw nanos: {:?}", read_time);
        }
    }

    // Timestamp
    // Timestamp is represented as milliseconds since unix epoch - 1970-01-01. Negative values are also possible

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.timestamps (t timestamp primary key)",
            &[],
        )
        .await?;

    // Timestamp can also be represented using 3 different types, chrono::DateTime<chrono::Utc>, time::OffsetDateTime and
    // CqlTimestamp. Only CqlTimestamp allows full range.

    // chrono::DateTime<chrono::Utc>
    let chrono_datetime = Utc::now();

    session
        .query(
            "INSERT INTO ks.timestamps (t) VALUES (?)",
            (chrono_datetime,),
        )
        .await?;

    if let Some(rows) = session
        .query("SELECT t from ks.timestamps", &[])
        .await?
        .rows
    {
        for row in rows.into_typed::<(DateTime<Utc>,)>() {
            let (read_time,) = row?;

            println!(
                "Parsed a timestamp into chrono::DateTime<chrono::Utc>: {:?}",
                read_time
            );
        }
    }

    // time::OffsetDateTime
    let time_datetime = time::OffsetDateTime::now_utc();

    session
        .query("INSERT INTO ks.timestamps (t) VALUES (?)", (time_datetime,))
        .await?;

    if let Some(rows) = session
        .query("SELECT t from ks.timestamps", &[])
        .await?
        .rows
    {
        for row in rows.into_typed::<(time::OffsetDateTime,)>() {
            let (read_time,) = row?;

            println!(
                "Parsed a timestamp into time::OffsetDateTime: {:?}",
                read_time
            );
        }
    }

    // CqlTimestamp
    let cql_datetime = CqlTimestamp(1 << 31);

    session
        .query("INSERT INTO ks.timestamps (t) VALUES (?)", (cql_datetime,))
        .await?;

    if let Some(rows) = session
        .query("SELECT t from ks.timestamps", &[])
        .await?
        .rows
    {
        for row in rows.into_typed::<(CqlTimestamp,)>() {
            let (read_time,) = row?;

            println!("Read a timestamp as raw millis: {:?}", read_time);
        }
    }

    Ok(())
}
