pub trait RelativeTime {
    fn relative_time(&self) -> String;
}

pub fn time_to_english(seconds: u64) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let months = days / 30;
    let years = (days as f64) / 365.0;

    if seconds < 30 {
        format!("{} secs ago", seconds)
    } else if seconds < 90 {
        "min ago".to_string()
    } else if minutes < 44 {
        format!("{} mins ago", minutes)
    } else if minutes < 90 {
        "an hr ago".to_string()
    } else if hours < 25 {
        format!("{} hrs ago", hours)
    } else if hours < 42 {
        "a day ago".to_string()
    } else if days < 30 {
        format!("{} days ago", days)
    } else if days < 45 {
        "a month ago".to_string()
    } else if days < 365 {
        format!("{} months ago", months)
    } else if years < 1.5 {
        "one year ago".to_string()
    } else {
        format!("{} years ago", years)
    }
}
