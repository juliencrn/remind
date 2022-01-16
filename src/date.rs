use chrono::prelude::*;
use std::convert::TryFrom;

pub const DAY: u64 = 24 * 60 * 60 * 1000;

pub fn to_timestamp(date: &DateTime<Utc>) -> u64 {
    u64::try_from(date.timestamp()).unwrap()
}

pub fn from_timestamp(ts: u64) -> DateTime<Utc> {
    Utc.timestamp(i64::try_from(ts).unwrap(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_date() -> DateTime<Utc> {
        "2017-07-14T02:40:00.000Z".parse::<DateTime<Utc>>().unwrap()
    }

    #[test]
    fn test_to_timestamp() {
        let dt = get_date();

        assert_eq!(to_timestamp(&dt), 1_500_000_000);
    }

    #[test]
    fn test_from_timestamp() {
        let dt = from_timestamp(1_500_000_000);

        assert_eq!(dt, get_date());
    }
}
