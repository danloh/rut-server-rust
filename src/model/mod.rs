// type model mod 

pub mod msg;
pub mod user;
pub mod rut;
pub mod item;
pub mod tag;
pub mod etc;

use actix_web::Error;
use regex::Regex;

// for validate request content input
pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}


// re test
// for re test uname
pub fn re_test_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^[\w-]{3,42}$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}

// for re test psw
pub fn re_test_psw(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^[\w#@~%^$&*-]{8,18}$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}

// for re test url
pub fn re_test_url(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^(https?)://([^/:]+)(:[0-9]+)?(/.*)?$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}

pub fn re_test_img_url(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = 
            Regex::new(r"^https?://.+\.(jpg|gif|png|svg)$").unwrap(); // let fail in test
    }
    RE.is_match(text)
}

pub fn replace_sep(text: &str, rep: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[-|,_]").unwrap(); // let fail in test
    }
    RE.replace_all(text, rep).into_owned()
}

pub fn trim_url_qry(text: &str, rep: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/ref=.*").unwrap(); // let fail in test
    }
    RE.replace_all(text, rep).into_owned()
}

pub fn test_len_limit(text: &str, min: usize, max: usize) -> bool {
    let l = text.trim().len();
    l >= min && l <= max
}

// some const to eliminate magic number
pub const PER_PAGE: i32 = 20;    // for paging
pub const TITLE_LEN: usize = 256;
pub const URL_LEN: usize = 256;
pub const UIID_LEN: usize = 32;
pub const TAG_LEN: usize = 42;
pub const ST_LEN: usize = 16;  // for some short input: category
pub const MID_LEN: usize = 32;  // for some mid input: lcoation
pub const LG_LEN: usize = 64;   // for sone longer input: 

