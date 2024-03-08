use chrono::DateTime;

pub fn parse_date(date_string: &str) -> String {
    let parsed_date = DateTime::parse_from_str(date_string, "%a %b %d %Y")
        .expect("Failed to parse date");

    parsed_date.format("%Y-%m-%d").to_string()
}