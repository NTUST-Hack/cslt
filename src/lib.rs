pub mod client;
pub mod login;
pub mod page;

pub use client::Client;
pub use client::ClientBuilder;

pub mod blocking;

use std::time::Duration;

pub const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub const DEFAULT_CHOOSE_LIST_URL: &'static str =
    "https://courseselection.ntust.edu.tw/ChooseList/D01/D01";

pub const SELECT_COURSE_PAGE_URL_PRE: &'static str =
    "https://courseselection.ntust.edu.tw/First/A06/A06";
pub const SELECT_COURSE_PAGE_URL_STARTED: &'static str =
    "https://courseselection.ntust.edu.tw/AddAndSub/B01/B01";

pub const SELECT_COURSE_API_URL_PRE: &'static str =
    "https://courseselection.ntust.edu.tw/First/A06/ExtraJoin";
pub const SELECT_COURSE_API_URL_STARTED: &'static str =
    "https://courseselection.ntust.edu.tw/AddAndSub/B01/ExtraJoin";

pub const DEFAULT_LOGIN_PAGE_URL: &'static str =
    "https://courseselection.ntust.edu.tw/Account/SingleSignOnLogin";
