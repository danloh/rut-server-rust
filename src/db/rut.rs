// rut typed model and msg handler

use actix::{ Handler };
use diesel::prelude::*;
use diesel::{ 
    self, QueryDsl, ExpressionMethods, 
    dsl::any, PgTextExpressionMethods, RunQueryDsl 
};
use chrono::{ Utc };
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::util::share::{ gen_slug };
use crate::model::msg::{ Msg, RutMsg, RutListMsg, StarStatusMsg };
use crate::model::rut::{ 
    Rut, CreateRut, QueryRut, QueryRuts, UpdateRut, StarRut, StarOrRut, StarRutStatus 
};
use crate::model::PER_PAGE;

// handle msg from api::rut.new_rut
impl Handler<CreateRut> for Dba {
    type Result = Result<RutMsg, ServiceError>;

    fn handle(&mut self, new_rut: CreateRut, _: &mut Self::Context) -> Self::Result {
        // import table, column
        use crate::schema::ruts::dsl::*;
        // retrieve a connecion from pool
        let conn = &self.0.get()?;
        
        // check if existing per url
        let new_url = &new_rut.url;
        if new_url.trim() != "" {
            let check_rut = ruts.filter(&url.eq(new_url)).load::<Rut>(conn)?.pop();
            if let Some(r) = check_rut {
                return Ok( RutMsg { 
                    status: 422, 
                    message: "Existing".to_string(),
                    rut: r,
                })
            }
        }

        // new rut
        let uuid_v4 = uuid::Uuid::new_v4();
        let uid = format!("{}", uuid_v4);
        let r_slug = gen_slug("r", &new_rut.title, &uuid_v4);
        let newrut = Rut::new(uid, r_slug, new_rut);
        let rut_new = diesel::insert_into(ruts)
            .values(&newrut).get_result::<Rut>(conn)?;

        Ok( RutMsg { 
            status: 201, 
            message: "Created".to_string(),
            rut: rut_new,
        })
    }
}

// handle msg from api::rut.get_rut
impl Handler<QueryRut> for Dba {
    type Result = Result<RutMsg, ServiceError>;

    fn handle(&mut self, rslug: QueryRut, _: &mut Self::Context) -> Self::Result {
        use crate::schema::ruts::dsl::*;
        let conn = &self.0.get()?;

        let rut_query = ruts.filter(&slug.eq(&rslug.rut_slug))  // slug here only
            .get_result::<Rut>(conn)?;
    
        Ok( RutMsg { 
            status: 200, 
            message: "Success".to_string(),
            rut: rut_query,
        })
    }
}

// handle msg from api::rut.get_rut_list
impl Handler<QueryRuts> for Dba {
    type Result = Result<RutListMsg, ServiceError>;

    fn handle(&mut self, per: QueryRuts, _: &mut Self::Context) -> Self::Result {
        use crate::schema::ruts::dsl::*;
        let conn = &self.0.get()?;
        
        let mut id_list: Vec<String> = Vec::new();
        let mut rut_list: Vec<Rut> = Vec::new();
        let mut rut_num = 0;  // total
        
        // build id_list per query type
        match per {
            QueryRuts::Index(_) => {
                rut_list = ruts.order(renew_at.desc())
                    //.order(vote.desc())
                    .limit(20).load::<Rut>(conn)?;
            },
            QueryRuts::UserID(u,f,p) => {
                if &f == "create" {
                    let query = ruts.filter(uname.eq(u));
                    rut_num = query.clone().count().get_result(conn)?;
                    rut_list = if p < 1 {  // no limit, hope never use
                        query.order(create_at.desc()).load::<Rut>(conn)?
                    } else {
                        query.order(create_at.desc())
                        .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                        .load::<Rut>(conn)?
                    };
                } else {
                    use crate::schema::starruts::dsl::*;
                    let query = starruts.filter(uname.eq(u));
                    rut_num = query.clone().count().get_result(conn)?;
                    id_list = if p < 1 { // no limit, hope never use
                        query.order(star_at.desc())
                        .select(rut_id).load::<String>(conn)?
                    } else {
                        query.order(star_at.desc())
                        .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                        .select(rut_id).load::<String>(conn)?
                    };
                }
            },
            QueryRuts::ItemID(i,p) => {
                use crate::schema::collects::dsl::*;
                let query = collects.filter(item_id.eq(i));
                rut_num = query.clone().count().get_result(conn)?;
                id_list = if p < 1 { // no limit, hope never use
                    query.order(collect_at.desc())
                    .select(rut_id).load::<String>(conn)?
                } else {
                    query.order(collect_at.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(rut_id).load::<String>(conn)?
                };
            },
            QueryRuts::TagID(t,p) => {
                use crate::schema::tagruts::dsl::*;
                let query = tagruts.filter(tname.eq(t));
                rut_num = query.clone().count().get_result(conn)?;
                id_list = if p < 1 { // no limit, hope never use
                    query.order(count.desc())
                    .select(rut_id).load::<String>(conn)?
                } else {
                    query.order(count.desc())
                    .limit(PER_PAGE.into()).offset((PER_PAGE * (p-1)).into())
                    .select(rut_id).load::<String>(conn)?
                };
            },
            QueryRuts::KeyID(k,f,i,p) => { // per keyword from taged, created, collected
                let fr = f.trim();
                match fr {
                    "user" => {  // just use this arm
                        rut_list = ruts.filter(&uname.eq(&i)).filter(&title.ilike(&k))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn)?;
                    },
                    "tag" => {  // hope never use, to optimaze
                        use crate::schema::tagruts::dsl::{tagruts, tname, rut_id};
                        let ids = tagruts.filter(&tname.eq(&i)).select(rut_id)
                            .load::<String>(conn)?;
                        rut_list = ruts.filter(&title.ilike(&k)).filter(&id.eq(any(&ids)))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn)?;
                    },
                    "item" => { // hope never use, to optimaze
                        use crate::schema::collects::dsl::{collects, item_id, rut_id};
                        let ids = collects.filter(&item_id.eq(&i)).select(rut_id)
                            .load::<String>(conn)?;
                        rut_list = ruts.filter(&title.ilike(&k)).filter(&id.eq(any(&ids)))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn)?;
                    },
                    _ => { // just query per keyword, hope never use 
                        rut_list = ruts.filter(&title.ilike(&k))
                            .order(create_at.desc()).limit(PER_PAGE.into())
                            .load::<Rut>(conn)?;
                    },
                }
            },
        }
        // build rut_list
        if id_list.len() > 0  {
            let mut rut_query = ruts.filter(&id.eq(any(&id_list))).load::<Rut>(conn)?;
            rut_list.append(&mut rut_query);
        }

        Ok( RutListMsg { 
            status: 201, 
            message: "Success".to_string(),
            ruts: rut_list,
            count: rut_num as usize,
        })
    }
}

// handle msg from api::rut.update_rut
impl Handler<UpdateRut> for Dba {
    type Result = Result<RutMsg, ServiceError>;

    fn handle(&mut self, rut: UpdateRut, _: &mut Self::Context) -> Self::Result {
        use crate::schema::ruts::dsl::*;
        let conn = &self.0.get()?;
        
        let rut_update = diesel::update(ruts.filter(&id.eq(&rut.id)))
            .set((
                title.eq(rut.title),
                url.eq(rut.url),
                content.eq(rut.content),
                author.eq(rut.author),
                credential.eq(rut.credential),
                renew_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<Rut>(conn)?;

        Ok( RutMsg { 
            status: 201, 
            message: "Updated".to_string(),
            rut: rut_update,
        })
    }
}

// handle msg from api::rut.star_unstar_rut
impl Handler<StarOrRut> for Dba {
    type Result = Result<StarStatusMsg, ServiceError>;

    fn handle(&mut self, rstar: StarOrRut, _: &mut Self::Context) -> Self::Result {
        use crate::schema::starruts::dsl::*;
        let conn = &self.0.get()?;

        use crate::schema::ruts::dsl::{
            ruts, id as rid, star_count, item_count, vote, comment_count
        };
        let rut_query = ruts.filter(&rid.eq(&rstar.rut_id))
            .get_result::<Rut>(conn)?;
        let s_count = rut_query.star_count;

        match rstar.action {
            1  => {
                let uid = format!("{}", uuid::Uuid::new_v4());
                let new_star = StarRut {
                    id: uid,
                    uname: rstar.clone().uname,
                    rut_id: rstar.clone().rut_id,
                    star_at: Utc::now().naive_utc(),
                    note: rstar.clone().note,
                };
                diesel::insert_into(starruts).values(&new_star).execute(conn)?;
                // to update star_count + 1 in rut
                diesel::update(ruts.filter(&rid.eq(&rstar.rut_id)))
                    .set((
                        star_count.eq(star_count + 1),
                        // cal vote, to be task
                        vote.eq(item_count * 2 + comment_count + star_count)
                    ))
                    .execute(conn)?;

                Ok( StarStatusMsg{ status: 200, message: "star".to_string(), count: s_count+1 })
            },
            0 => {
                diesel::delete(
                    starruts.filter(&rut_id.eq(&rstar.rut_id))
                            .filter(&uname.eq(&rstar.uname))
                )
                .execute(conn)?;
                // to update the star_count - 1 in rut
                diesel::update(ruts.filter(&rid.eq(&rstar.rut_id)))
                    .set(star_count.eq(star_count - 1)).execute(conn)?;

                Ok( StarStatusMsg{ status: 200, message: "unstar".to_string(),count: s_count-1 })
            },
            _ =>  { Ok( StarStatusMsg{ status: 400, message: "unstar".to_string(), count: s_count }) },
        }
    }
}

// handle msg from api::rut.star_rut_status
impl Handler<StarRutStatus> for Dba {
    type Result = Result<StarStatusMsg, ServiceError>;

    fn handle(&mut self, status: StarRutStatus, _: &mut Self::Context) -> Self::Result {
        use crate::schema::starruts::dsl::*;
        let conn = &self.0.get()?;

        use crate::schema::ruts::dsl::{ruts, id as rid, star_count};
        let s_count = ruts.filter(&rid.eq(&status.rut_id))
            .select(star_count).get_result::<i32>(conn)?;

        let check_status = starruts
            .filter(&rut_id.eq(&status.rut_id))
            .filter(&uname.eq(&status.uname))
            .load::<StarRut>(conn)?.pop();
        let msg = match check_status {
            Some(_) => { "star" },
            None => { "unstar" },
        };
        
        Ok(StarStatusMsg{status: 200, message: msg.to_string(), count: s_count})
    }
}
