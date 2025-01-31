/* Several function to retrieve version information.
 */

// use chrono::{prelude::DateTime, Local};
use std::time::{Duration, UNIX_EPOCH};

// = 3.12.0alpha
pub const MAJOR: usize = 3;
pub const MINOR: usize = 12;
pub const MICRO: usize = 0;
pub const RELEASELEVEL: &str = "alpha";
pub const RELEASELEVEL_N: usize = 0xA;
pub const SERIAL: usize = 0;

pub const VERSION_HEX: usize =
    (MAJOR << 24) | (MINOR << 16) | (MICRO << 8) | (RELEASELEVEL_N << 4) | SERIAL;

pub fn get_version() -> String {
    format!(
        "{:.80} ({:.80}) \n[{:.80}]", // \n is PyPy convention
        get_version_number(),
        get_build_info(),
        get_compiler()
    )
}

pub fn get_version_number() -> String {
    format!("{MAJOR}.{MINOR}.{MICRO}{RELEASELEVEL}")
}

pub fn get_winver_number() -> String {
    format!("{MAJOR}.{MINOR}")
}

pub fn get_compiler() -> String {
    format!("rustc {}", env!("RUSTC_VERSION"))
}

pub fn get_build_info() -> String {
    // See: https://reproducible-builds.org/docs/timestamps/
    let git_revision = get_git_revision();
    let separator = if git_revision.is_empty() { "" } else { ":" };

    let git_identifier = get_git_identifier();

    format!(
        "{id}{sep}{revision}, {date:.20}, {time:.9}",
        id = if git_identifier.is_empty() {
            "default".to_owned()
        } else {
            git_identifier
        },
        sep = separator,
        revision = git_revision,
        date = get_git_date(),
        time = get_git_time(),
    )
}

pub fn get_git_revision() -> String {
    option_env!("RUSTPYTHON_GIT_HASH").unwrap_or("").to_owned()
}

pub fn get_git_tag() -> String {
    option_env!("RUSTPYTHON_GIT_TAG").unwrap_or("").to_owned()
}

pub fn get_git_branch() -> String {
    option_env!("RUSTPYTHON_GIT_BRANCH")
        .unwrap_or("")
        .to_owned()
}

pub fn get_git_identifier() -> String {
    let git_tag = get_git_tag();
    let git_branch = get_git_branch();

    if git_tag.is_empty() || git_tag == "undefined" {
        git_branch
    } else {
        git_tag
    }
}

fn get_git_timestamp_datetime() {
    // let timestamp = option_env!("RUSTPYTHON_GIT_TIMESTAMP")
    //     .unwrap_or("")
    //     .to_owned();
    // let timestamp = timestamp.parse::<u64>().unwrap_or(0);

    // let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);

    // datetime.into()

    panic!("Not supported in NEAR");
}

pub fn get_git_date() -> String {
    // let now = std::time::SystemTime::now();
    // let datetime = now
    //     .duration_since(std::time::UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let secs = datetime.as_secs();

    // // Simple formatting without chrono
    // // This is a basic implementation that will show e.g. "Jan 1 2024"
    // let months = [
    //     "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    // ];

    // let days_since_epoch = secs / (24 * 60 * 60);
    // let year = 1970 + (days_since_epoch / 365);
    // let day_of_year = days_since_epoch % 365;

    // // Very basic month calculation
    // let month_idx = (day_of_year / 31) as usize;
    // let day = (day_of_year % 31) + 1;

    // format!("{} {:2} {}", months[month_idx.min(11)], day, year)
    "Jan 1 2024".to_string()
}

pub fn get_git_time() -> String {
    "12:00:00".to_string()
}

pub fn get_git_datetime() -> String {
    let date = get_git_date();
    let time = get_git_time();

    format!("{date} {time}")
}
