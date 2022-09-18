use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub(crate) static ref NUM_BULLET_RE: Regex = Regex::new(r"^[0-9]\.").unwrap();
    pub(crate) static ref CODE_BEGIN_RE: Regex = Regex::new(r"^```\w").unwrap();
    pub(crate) static ref CODE_END_RE: Regex = Regex::new(r"^```\r?$").unwrap();
}
