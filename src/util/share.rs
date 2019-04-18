// some sharing helpers

use base64;
use std::collections::HashMap;
use uuid::Uuid;
use deunicode::deunicode_char;

pub fn to_blob(uuid: &Uuid) -> String {
    base64::encode_config(uuid.as_bytes(), base64::URL_SAFE_NO_PAD)
}

// credit to https://github.com/Stebalien/slug-rs/blob/master/src/lib.rs
pub fn slugify<S: AsRef<str>>(s: S) -> String {
    _slugify(s.as_ref())
}

fn _slugify(s: &str) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'...b'z' | b'0'...b'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                }
                b'A'...b'Z' => {
                    prev_is_dash = false;
                    slug.push(x - b'A' + b'a'); // u8
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(b'-');
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    if string.ends_with('-') {
        string.pop();
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}

// slug, better to show url: ty as type, for rut|item|collect
pub fn gen_slug(ty: &str, text: &str, uid: &Uuid) -> String {
    format!("{}-{}-{}", ty, slugify(text), to_blob(uid))
}

// get the value per key
pub fn get_v(map_ref: &HashMap<String, String>, k: &str) -> String { 
    let res = map_ref.get(k);
    
    if let Some(r) = res {
        return r.clone()
    }
    return "".to_string()
}
