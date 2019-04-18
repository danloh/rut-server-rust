// rut msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, dsl::any, PgTextExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::rut::{
    Rut, NewRut, CreateRut, RutID, RutsPerID, UpdateRut, 
    StarRut, RutStar, StarOrRut, StarRutStatus
};
use model::msg::{ Msg, RutMsg, RutListMsg };
use PER_PAGE;

// rut model

use db::schema::{ ruts, starruts };
use actix_web::{ Error, actix::Message };
use chrono::{Utc, NaiveDateTime};
use model::msg::{ Msg, RutMsg, RutListMsg };

// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="ruts"]
pub struct Rut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub create_at: NaiveDateTime,
    pub renew_at: NaiveDateTime,
    pub author_id: String,
    pub uname: String,     // as who post
    pub credential: String,
    pub logo: String,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
    pub vote: i32,       // cal per star, comment
    // pub slug: String,  // to do
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="ruts"]
pub struct NewRut<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub content: &'a str,
    pub create_at: NaiveDateTime,
    pub renew_at: NaiveDateTime,
    pub author_id: &'a str,
    pub uname: &'a str,
    pub credential: &'a str,
    pub logo: &'a str,
    pub item_count: i32,
    pub comment_count: i32,
    pub star_count: i32,
}

// as msg in create new
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateRut {
    pub title: String,
    pub url: String,
    pub content: String,
    pub uname: String,
    pub author_id: String,
    pub credential: String,
}

impl Message for CreateRut {
    type Result = Result<RutMsg, Error>;
}

// as msg in update rut
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="ruts"]
pub struct UpdateRut {
    pub id: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub author_id: String,
    pub credential: String,
}

impl Message for UpdateRut {
    type Result = Result<RutMsg, Error>;
}

// as msg in select by id
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutID {
    pub rut_id: String,
}

impl Message for RutID {
    type Result = Result<RutMsg, Error>;
}

// as msg to get  rut list, + paging
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RutsPerID {
    Index(String),
    UserID(String, String, i32), // uname, create/star, paging
    ItemID(String, i32),
    TagID(String, i32),
    KeyID(String, String, String, i32), // keyword, per, perid(uname|item|tname), paging
}

impl Message for RutsPerID {
    type Result = Result<RutListMsg, Error>;
}

// Rut's constructor
impl Rut {
    pub fn new() -> Rut {
        Rut {
            id: "".to_owned(),
            title: "".to_owned(),
            url: "".to_owned(),
            content: "".to_owned(),
            create_at: Utc::now().naive_utc(),
            renew_at: Utc::now().naive_utc(),
            uname: "".to_owned(),
            author_id: "".to_owned(),
            credential: "".to_owned(),
            logo: "".to_owned(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
            vote: 0,
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="starruts"]
pub struct StarRut {
    pub id: String,
    pub uname: String,
    pub rut_id: String,
    pub star_at: NaiveDateTime,
    pub note: String,
}

// use to build insert query
#[derive(Debug,Clone,Serialize,Deserialize,Insertable)]
#[table_name="starruts"]
pub struct RutStar<'a> {
    pub id: &'a str,
    pub uname: &'a str,
    pub rut_id: &'a str,
    pub star_at: NaiveDateTime,
    pub note: &'a str,
}

// as msg in star or unstar rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarOrRut {
    pub rut_id: String,
    pub uname: String,
    pub note: String,
    pub action: u8,  // 0- unstar, 1- star
}

impl Message for StarOrRut {
    type Result = Result<Msg, Error>;
}

// as msg to check if star a rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarRutStatus {
    pub uname: String,
    pub rut_id: String,
}

impl Message for StarRutStatus {
    type Result = Result<Msg, Error>;
}

// handle msg from api::rut.new_rut
impl Handler<CreateRut> for Dba {
    type Result = Result<RutMsg, Error>;

    fn handle(&mut self, new_rut: CreateRut, _: &mut Self::Context) -> Self::Result {
        // import table, column
        use db::schema::ruts::dsl::*;
        // retrieve a connecion from pool
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        // check if existing per url
        let new_url = &new_rut.url;
        if new_url.trim() != "" {
            let check_q = ruts.filter(&url.eq(new_url))
                .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?.pop();
            if let Some(r) = check_q {
                return Ok( RutMsg { 
                    status: 422, 
                    message: "Existing".to_string(),
                    rut: r,
                })
            }
        }

        let uid = format!("{}", uuid::Uuid::new_v4());
        let newrut = NewRut {
            id: &uid,
            title: &new_rut.title,
            url: &new_rut.url,
            content: &new_rut.content,
            uname: &new_rut.uname,
            author_id: &new_rut.author_id,
            credential: &new_rut.credential,
            logo: "",
            create_at: Utc::now().naive_utc(),
            renew_at: Utc::now().naive_utc(),
            item_count: 0,
            comment_count: 0,
            star_count: 0,
        };
        let rut_new = diesel::insert_into(ruts)
            .values(&newrut)
            .get_result::<Rut>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( RutMsg { 
            status: 201, 
            message: "Created".to_string(),
            rut: rut_new.clone(),
        })
    }
}

// handle msg from api::rut.get_rut
impl Handler<RutID> for Dba {
    type Result = Result<RutMsg, Error>;

    fn handle(&mut self, rid: RutID, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let rut_query = ruts.filter(&id.eq(&rid.rut_id))
            .get_result::<Rut>(conn)
            .map_err(error::ErrorInternalServerError)?;
    
        Ok( RutMsg { 
            status: 200, 
            message: "Success".to_string(),
            rut: rut_query.clone(),
        })
    }
}

// handle msg from api::rut.get_rut_list
impl Handler<RutsPerID> for Dba {
    type Result = Result<RutListMsg, Error>;

    fn handle(&mut self, per: RutsPerID, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let mut id_list: Vec<String> = Vec::new();
        let mut rut_list: Vec<Rut> = Vec::new();
        let mut rut_num = 0;  // total
        
        // build id_list per query type
        match per {
            RutsPerID::Index(_) => {
                id_list = ruts.select(id)
                    .order(renew_at.desc())
                    .order(vote.desc()).limit(20)
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            RutsPerID::UserID(u,f,p) => {
                if &f == "create" {
                    let query = ruts.filter(uname.eq(u));
                    rut_num = query.clone().count().get_result(conn)
                        .map_err(error::ErrorInternalServerError)?;
                    rut_list = if p < 1 {  // no limit, hope never use
                        query.order(create_at.desc())
                        .load::<Rut>(conn)
                        .map_err(error::ErrorInternalServerError)?
                    } else {
                        query.order(create_at.desc())
                        .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                        .load::<Rut>(conn)
                        .map_err(error::ErrorInternalServerError)?
                    };
                } else {
                    use db::schema::starruts::dsl::*;
                    let query = starruts.filter(uname.eq(u));
                    rut_num = query.clone().count().get_result(conn)
                        .map_err(error::ErrorInternalServerError)?;
                    id_list = if p < 1 { // no limit, hope never use
                        query.order(star_at.desc())
                        .select(rut_id).load::<String>(conn)
                        .map_err(error::ErrorInternalServerError)?
                    } else {
                        query.order(star_at.desc())
                        .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                        .select(rut_id).load::<String>(conn)
                        .map_err(error::ErrorInternalServerError)?
                    };
                }
            },
            RutsPerID::ItemID(i,p) => {
                use db::schema::collects::dsl::*;
                let query = collects.filter(item_id.eq(i));
                rut_num = query.clone().count().get_result(conn)
                    .map_err(error::ErrorInternalServerError)?;
                id_list = if p < 1 { // no limit, hope never use
                    query.order(collect_at.desc())
                    .select(rut_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                } else {
                    query.order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(rut_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                };
            },
            RutsPerID::TagID(t,p) => {
                use db::schema::tagruts::dsl::*;
                let query = tagruts.filter(tname.eq(t));
                rut_num = query.clone().count().get_result(conn)
                    .map_err(error::ErrorInternalServerError)?;
                id_list = if p < 1 { // no limit, hope never use
                    query.order(count.desc())
                    .select(rut_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                } else {
                    query.order(count.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(rut_id).load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?
                };
            },
            RutsPerID::KeyID(k,f,i,p) => { // per keyword from taged, created, collected
                let fr = f.trim();
                match fr {
                    "user" => {  // just use this arm
                        rut_list = ruts.filter(&uname.eq(&i)).filter(&title.ilike(&k))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
                    },
                    "tag" => {  // hope never use, to optimaze
                        use db::schema::tagruts::dsl::{tagruts, tname, rut_id};
                        let ids = tagruts.filter(&tname.eq(&i)).select(rut_id)
                            .load::<String>(conn).map_err(error::ErrorInternalServerError)?;
                        rut_list = ruts.filter(&title.ilike(&k)).filter(&id.eq(any(&ids)))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
                    },
                    "item" => { // hope never use, to optimaze
                        use db::schema::collects::dsl::{collects, item_id, rut_id};
                        let ids = collects.filter(&item_id.eq(&i)).select(rut_id)
                            .load::<String>(conn).map_err(error::ErrorInternalServerError)?;
                        rut_list = ruts.filter(&title.ilike(&k)).filter(&id.eq(any(&ids)))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
                    },
                    _ => { // just query per keyword, hope never use 
                        rut_list = ruts.filter(&title.ilike(&k))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
                    },
                }
            },
        }
        // build rut_list
        if id_list.len() > 0  {
            let mut rut_query = ruts.filter(&id.eq(any(&id_list)))
                .load::<Rut>(conn).map_err(error::ErrorInternalServerError)?;
            rut_list.append(&mut rut_query);
        }

        Ok( RutListMsg { 
            status: 200, 
            message: "Success".to_string(),
            ruts: rut_list.clone(),
            count: rut_num as usize,
        })
    }
}

// handle msg from api::rut.update_rut
impl Handler<UpdateRut> for Dba {
    type Result = Result<RutMsg, Error>;

    fn handle(&mut self, rut: UpdateRut, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let rut_update = diesel::update(ruts.filter(&id.eq(&rut.id)))
            .set((
                title.eq(rut.title.clone()),
                url.eq(rut.url.clone()),
                content.eq(rut.content.clone()),
                author_id.eq(rut.author_id.clone()),
                credential.eq(rut.credential.clone()),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<Rut>(conn)
            .map_err(error::ErrorInternalServerError)?;

        Ok( RutMsg { 
            status: 201, 
            message: "Updated".to_string(),
            rut: rut_update.clone(),
        })
    }
}

// handle msg from api::rut.star_unstar_rut
impl Handler<StarOrRut> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, act: StarOrRut, _: &mut Self::Context) -> Self::Result {
        use db::schema::starruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        match act.action {
            1  => {
                let uid = format!("{}", uuid::Uuid::new_v4());
                let new_star = RutStar {
                    id: &uid,
                    uname: &act.uname,
                    rut_id: &act.rut_id,
                    star_at: Utc::now().naive_utc(),
                    note: &act.note,
                };
                diesel::insert_into(starruts).values(&new_star)
                        .execute(conn).map_err(error::ErrorInternalServerError)?;
                // to update star_count + 1 in rut
                use db::schema::ruts::dsl::{
                    ruts, id as rid, star_count, item_count, vote, comment_count
                };
                diesel::update(ruts.filter(&rid.eq(&act.rut_id)))
                    .set((
                        star_count.eq(star_count + 1),
                        // cal vote, to be task
                        vote.eq(item_count * 2 + comment_count + star_count)
                    ))
                    .execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "star".to_string(),})
            },
            0 => {
                diesel::delete(
                    starruts.filter(&rut_id.eq(&act.rut_id))
                            .filter(&uname.eq(&act.uname))
                )
                .execute(conn).map_err(error::ErrorInternalServerError)?;
                // to update the star_count - 1 in rut
                use db::schema::ruts::dsl::{ruts, id as rid, star_count};
                diesel::update(ruts.filter(&rid.eq(&act.rut_id)))
                    .set(star_count.eq(star_count - 1)).execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "unstar".to_string(),})
            },
            _ =>  { Ok( Msg { status: 400, message: "unstar".to_string(),}) },
        }
    }
}

// handle msg from api::rut.star_rut_status
impl Handler<StarRutStatus> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, status: StarRutStatus, _: &mut Self::Context) -> Self::Result {
        use db::schema::starruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_status = starruts
            .filter(&rut_id.eq(&status.rut_id))
            .filter(&uname.eq(&status.uname))
            .load::<StarRut>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        match check_status {
            Some(_) => { Ok( Msg {status: 200, message: "star".to_string() }) },
            None => { Ok( Msg { status: 200, message: "unstar".to_string() }) },
        }
    }
}
