// Result msg struct in response

use model::rut::Rut;
use model::item::{ Item, Collect };
use model::user::{ User, CheckUser };

// general response msg struct
#[derive(Deserialize,Serialize,Debug)]
pub struct Msgs {
    pub status: i32,
    pub message: String,
}

// msg for login or get user info
#[derive(Deserialize,Serialize,Debug)]
pub struct LoginMsgs {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: CheckUser,
}

// result struct in response a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct RutMsgs {
    pub status: i32,
    pub message: String,
    pub rut: Rut,
}

// result struct in response rut list
#[derive(Deserialize,Serialize,Debug)]
pub struct RutListMsgs {
    pub status: i32,
    pub message: String,
    pub ruts: Vec<Rut>,
    pub count: usize,
}

// result struct in response an item 
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemMsgs {
    pub status: i32,
    pub message: String,
    pub item: Item,
}

// result struct in response item list
#[derive(Deserialize,Serialize,Debug)]
pub struct ItemListMsgs {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub count: usize,
}

// result struct in response an items in a rut 
#[derive(Deserialize,Serialize,Debug)]
pub struct CollectMsgs {
    pub status: i32,
    pub message: String,
    pub rut_id: String,
    pub collects: Vec<Collect>,
}
