// model mod

// build db table struct(msg, Queryable, insertable) for crud 
// Message: as msg to send/recv btw api/handler, - Actor
// Table strcut + Dsl trait: use in building SQL query

pub mod user;
pub mod msg;
pub mod rut;
pub mod item;
pub mod tag;
pub mod etc;
