use surrealdb::{
    Error, Surreal,
    engine::local::{Db, SurrealKv},
};

use crate::{constants::FILE_TABLE_NAME, utils::get_os_home::get_home};

pub async fn init() -> Result<Surreal<Db>, Error> {
    let home = get_home().unwrap();

    let db = Surreal::new::<SurrealKv>(format!("{}/kongg-db", home))
        .versioned()
        .await?;
    db.use_ns("test").use_db("test").await?;

    // let a = db
    //     .query(format!("REMOVE TABLE {};", FILE_TABLE_NAME))
    //     .await
    //     .unwrap();

    Ok(db)
}
