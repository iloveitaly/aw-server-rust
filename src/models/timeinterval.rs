use std::fmt;

use serde::de::{self, Visitor, Deserialize, Deserializer};

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

#[derive(Clone,Debug)]
pub struct TimeInterval {
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

#[derive(Debug)]
pub enum TimeIntervalError {
    ParseError()
}

impl TimeInterval {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> TimeInterval {
        TimeInterval {
            start: start,
            end: end
        }
    }

    pub fn new_from_string(period: &str) -> Result<TimeInterval, TimeIntervalError> {
        let splits = period.split("/").collect::<Vec<&str>>();
        if splits.len() != 2 {
            return Err(TimeIntervalError::ParseError());
        }
        let start = match DateTime::parse_from_rfc3339(splits[0]) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_e) => return Err(TimeIntervalError::ParseError())
        };
        let end = match DateTime::parse_from_rfc3339(splits[1]) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_e) => return Err(TimeIntervalError::ParseError())
        };

        Ok(TimeInterval::new(start, end))
    }

    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.start.to_rfc3339(), self.end.to_rfc3339())
    }
}

struct TimeIntervalVisitor;

impl<'de> Visitor<'de> for TimeIntervalVisitor {
    type Value = TimeInterval;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string in ISO timeinterval format (such as 2000-01-01T00:00:00+01:00/2001-02-02T01:01:01+01:00)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // TODO: do not unwrap and return proper error
        println!("asdiuwehfiewuhf: {}", value);
        Ok(TimeInterval::new_from_string(&value).unwrap())
    }
}

impl<'de> Deserialize<'de> for TimeInterval {
    fn deserialize<D>(deserializer: D) -> Result<TimeInterval, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeIntervalVisitor)
    }
}
