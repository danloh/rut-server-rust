// rut msg handler

use db::dba::Dba;
use actix_web::{ actix::Handler, error, Error };
use diesel::{ self, QueryDsl, ExpressionMethods, RunQueryDsl };
use chrono::Utc;
use uuid;

use model::rut::{
    Rut, NewRut, CreateRut, RutID, RutsPerID, UpdateRut, 
    StarRut, RutStar, StarOrRut, StarRutStatus
};
use model::msg::{ Msg, RutMsg, RutListMsg };

// handle msg from api::rut.new_rut
impl Handler<CreateRut> for Dba {
    type Result = Result<RutMsg, Error>;

    fn handle(&mut self, new_rut: CreateRut, _: &mut Self::Context) -> Self::Result {
        // import table, column
        use db::schema::ruts::dsl::*;
        // retrieve a connecion from pool
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let newrut = NewRut {
            id: &uuid,
            title: &new_rut.title,
            url: &new_rut.url,
            content: &new_rut.content,
            user_id: &new_rut.user_id,
            user_name: &new_rut.user_name,
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
            status: 200, 
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
        
        // build id_list per query type
        match per {
            RutsPerID::Index(_) => {
                id_list = ruts.select(id).order(id.desc()).limit(20)
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            RutsPerID::UserID(u,f) => {
                if &f == "create" {
                    rut_list = ruts.filter(&user_id.eq(&u)).load::<Rut>(conn)
                        .map_err(error::ErrorInternalServerError)?;
                } else {
                    use db::schema::starruts::dsl::*;
                    id_list = starruts.filter(&user_id.eq(&u)).select(rut_id)
                        .load::<String>(conn)
                        .map_err(error::ErrorInternalServerError)?;
                }
            },
            RutsPerID::ItemID(i) => {
                use db::schema::collects::dsl::*;
                id_list = collects.filter(&item_id.eq(&i)).select(rut_id)
                    .load::<String>(conn)
                    .map_err(error::ErrorInternalServerError)?;
            },
            RutsPerID::TagID(t) => {
                use db::schema::tagruts::dsl::*;
                id_list = tagruts.filter(&tname.eq(&t)).select(rut_id)  
                    .load::<String>(conn)    // to do order per count
                    .map_err(error::ErrorInternalServerError)?;
            },
        }
        // build rut_list
        for rid in id_list {
            let rut_query = ruts.filter(&id.eq(&rid)).load::<Rut>(conn)
                .map_err(error::ErrorInternalServerError)?.pop();
            let mut rut = Rut::new(); 
            match rut_query {
                Some(r_q) => {
                    rut = r_q.clone();
                    rut_list.push(rut);
                },
                None => (),
            }
        }

        Ok( RutListMsg { 
            status: 200, 
            message: "Success".to_string(),
            ruts: rut_list.clone(),
            count: rut_list.len(),
        })
    }
}

// handle msg from api::rut.update_rut
impl Handler<UpdateRut> for Dba {
    type Result = Result<RutMsg, Error>;

    fn handle(&mut self, rut: UpdateRut, _: &mut Self::Context) -> Self::Result {
        use db::schema::ruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        let rut_update = diesel::update(ruts)
            .filter(&id.eq(&rut.id))
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
            status: 200, 
            message: "Updated".to_string(),
            rut: rut_update.clone(),
        })
    }
}

// handle msg from api::rut.star_unstar_rut
impl Handler<StarOrRut> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, act: StarOrRut, _: &mut Self::Context) -> Self::Result {
        // use db::schema::ruts::dsl::*;
        use db::schema::starruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        
        match act.action {
            1  => {
                let uuid = format!("{}", uuid::Uuid::new_v4());
                let new_star = RutStar {
                    id: &uuid,
                    user_id: &act.user_id,
                    rut_id: &act.rut_id,
                    star_at: Utc::now().naive_utc(),
                    note: &act.note,
                };
                diesel::insert_into(starruts).values(&new_star)
                        .execute(conn).map_err(error::ErrorInternalServerError)?;
                // to update star_count + 1 in rut
                use db::schema::ruts::dsl::{ruts, id as rid, star_count};
                diesel::update(ruts).filter(&rid.eq(&act.rut_id))
                    .set(star_count.eq(star_count + 1)).execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "star".to_string(),})
            },
            0 => {
                diesel::delete(
                    starruts.filter(&rut_id.eq(&act.rut_id))
                            .filter(&user_id.eq(&act.user_id))
                )
                .execute(conn).map_err(error::ErrorInternalServerError)?;
                // to update the star_count - 1 in rut
                use db::schema::ruts::dsl::{ruts, id as rid, star_count};
                diesel::update(ruts).filter(&rid.eq(&act.rut_id))
                    .set(star_count.eq(star_count - 1)).execute(conn)
                    .map_err(error::ErrorInternalServerError)?;

                Ok( Msg { status: 200, message: "unstar".to_string(),})
            },
            _ =>  { Ok( Msg { status: 400, message: "unstar".to_string(),}) },
        }
    }
}

// handle msg from api::rut.star_unstar_rut
impl Handler<StarRutStatus> for Dba {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, status: StarRutStatus, _: &mut Self::Context) -> Self::Result {
        use db::schema::starruts::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

        let check_status = starruts
            .filter(&rut_id.eq(&status.rut_id))
            .filter(&user_id.eq(&status.user_id))
            .load::<StarRut>(conn)
            .map_err(error::ErrorInternalServerError)?.pop();
        
        match check_status {
            Some(_) => { Ok( Msg {status: 200, message: "star".to_string() }) },
            None => { Ok( Msg { status: 200, message: "unstar".to_string() }) },
        }
    }
}
