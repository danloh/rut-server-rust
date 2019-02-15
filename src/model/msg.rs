// Result msg struct in response

use model::rut::Rut;
use model::user::User;

// general response msg struct
#[derive(Deserialize,Serialize,Debug)]
pub struct Msgs {
    pub status: i32,
    pub message: String,
}

// msg for login
#[derive(Deserialize,Serialize,Debug)]
pub struct LoginMsgs {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: User,
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
