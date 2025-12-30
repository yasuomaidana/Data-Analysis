use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use rand::random_range;

pub fn random_birth_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(
        random_range(2000..2018),
        random_range(1..13),
        random_range(1..29),
    )
    .unwrap()
}
