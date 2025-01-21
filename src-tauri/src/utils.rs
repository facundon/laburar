use crate::error::Error;
use chrono::{NaiveDate, NaiveDateTime};

const DATE_FORMAT: &str = "%Y-%m-%d";
const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn parse_date_helper<T, F>(date_str: Option<&str>, parser: F) -> Result<Option<T>, Error>
where
    F: FnOnce(&str) -> Result<T, chrono::ParseError>,
{
    match date_str {
        Some(date) => Ok(Some(parser(date).map_err(|e| Error::DateParse(e))?)),
        None => Ok(None),
    }
}

pub fn parse_date_option(date_str: Option<&str>) -> Result<Option<NaiveDate>, Error> {
    parse_date_helper(date_str, |date| {
        NaiveDate::parse_from_str(date, DATE_FORMAT)
    })
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, Error> {
    NaiveDate::parse_from_str(date_str, DATE_FORMAT).map_err(|e| Error::DateParse(e))
}

pub fn parse_date_time_option(date_str: Option<&str>) -> Result<Option<NaiveDateTime>, Error> {
    parse_date_helper(date_str, |date| {
        NaiveDateTime::parse_from_str(date, DATE_TIME_FORMAT)
    })
}

pub fn parse_date_time(date_str: &str) -> Result<NaiveDateTime, Error> {
    NaiveDateTime::parse_from_str(date_str, DATE_TIME_FORMAT).map_err(|e| Error::DateParse(e))
}
