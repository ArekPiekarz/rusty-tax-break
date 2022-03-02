use time::{Date, OffsetDateTime, UtcOffset};
use tz::TimeZone;

pub(crate) type MonthInt = u8;

const SECONDS_IN_MINUTE: i32 = 60;


pub fn getCurrentDate() -> Date
{
    let localTimeZone = TimeZone::local().unwrap();
    let localTimeType = localTimeZone.find_current_local_time_type().unwrap();
    let timeZoneOffsetSeconds = localTimeType.ut_offset();
    let timeZoneOffset = UtcOffset::from_whole_seconds(timeZoneOffsetSeconds).unwrap();
    let utcDateTime = OffsetDateTime::now_utc();
    let localDateTime = utcDateTime.to_offset(timeZoneOffset);
    localDateTime.date()
}

pub(crate) fn makeDateTime(inputTime: &git2::Time) -> OffsetDateTime
{
    let timeZoneOffset = UtcOffset::from_whole_seconds(inputTime.offset_minutes() * SECONDS_IN_MINUTE).unwrap();
    OffsetDateTime::from_unix_timestamp(inputTime.seconds()).unwrap().to_offset(timeZoneOffset)
}
