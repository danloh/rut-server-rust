// Result msg typed model

use crate::db::rut::Rut;
use crate::db::item::{ Item, Collect };
use crate::db::user::{ User, CheckUser };
use crate::db::tag::{ Tag };
use crate::db::etc::{ Etc };

// general response msg struct
#[derive(Deserialize,Serialize,Debug)]
pub struct Msg {
    pub status: i32,
    pub message: String,
}

// msg for login
#[derive(Deserialize,Serialize,Debug)]
pub struct AuthMsg {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: CheckUser,
}

// msg for get user info
#[derive(Deserialize,Serialize,Debug)]
pub struct UserMsg {
    pub status: i32,
    pub message: String,
    pub user: CheckUser,
}

// result struct in response a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct RutMsg {
    pub status: i32,
    pub message: String,
    pub rut: Rut,
}

// result struct in response rut list
#[derive(Deserialize,Serialize,Debug)]
pub struct RutListMsg {
    pub status: i32,
    pub message: String,
    pub ruts: Vec<Rut>,
    pub count: usize,
}

// result struct in response an item 
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemMsg {
    pub status: i32,
    pub message: String,
    pub item: Item,
}

// result struct in response item list
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemListMsg {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub count: usize,
}

// result struct in respon the status of star item
#[derive(Deserialize,Serialize,Debug)]
pub struct StarItemMsg {
    pub status: i32,
    pub message: String,
    pub note: String,
    pub when: String,
}

// result struct in response collect in a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct CollectMsg {
    pub status: i32,
    pub message: String,
    pub collect: Collect,
}

// result struct in response collects  
#[derive(Deserialize,Serialize,Debug)]
pub struct CollectsMsg {
    pub status: i32,
    pub message: String,
    pub collects: Vec<Collect>,
}

// result struct in response tag
#[derive(Deserialize,Serialize,Debug)]
pub struct TagMsg {
    pub status: i32,
    pub message: String,
    pub tag: Tag,
}

// result struct in response tag list
#[derive(Deserialize,Serialize,Debug)]
pub struct TagListMsg {
    pub status: i32,
    pub message: String,
    pub tags: Vec<String>, // tag name
    pub count: usize,
}

// result struct in response etc
#[derive(Deserialize,Serialize,Debug)]
pub struct EtcMsg {
    pub status: i32,
    pub message: String,
    pub etc: Etc,
}

// result struct in response etc list
#[derive(Deserialize,Serialize,Debug)]
pub struct EtcListMsg {
    pub status: i32,
    pub message: String,
    pub etcs: Vec<Etc>,
    pub count: usize,
}

// todo
// respon the status of star rut, follow tag, etc.
#[derive(Deserialize,Serialize,Debug)]
pub struct StarStatusMsg {
    pub status: i32,   // response code
    pub message: String,  // star | Unstar
    pub count: i32,    // star num
}
