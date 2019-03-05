// some sharing helpers

use std::collections::HashMap;

// get the value per key
pub fn get_v(map_ref: &HashMap<String, String>, k: &str) -> String { 
    let res = map_ref.get(k);
    
    if let Some(r) = res {
        return r.clone()
    }
    return "".to_string()
}
