// Result msg struct in response

use model::rut::Rut;

// general response msg struct
#[derive(Deserialize,Serialize,Debug)]
pub struct Msgs {
    pub status: i32,
    pub message: String,
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
