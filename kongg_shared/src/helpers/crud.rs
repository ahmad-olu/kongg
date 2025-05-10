use surrealdb::{RecordId, Surreal, engine::local::Db};

use crate::{
    constants::FILE_TABLE_NAME,
    models::file::{File, FileResponse},
};

pub async fn get_file(db: &Surreal<Db>) -> Vec<FileResponse> {
    db.select(FILE_TABLE_NAME).await.unwrap()
}

pub async fn create_file(db: &Surreal<Db>, file: File) -> () {
    let _res: Option<FileResponse> = db.create(FILE_TABLE_NAME).content(file).await.unwrap();
    ()
}

#[warn(dead_code)]
async fn _update_file(db: &Surreal<Db>, file: File, id: RecordId) -> () {
    let _res: Option<FileResponse> = db.update(id).merge(file).await.unwrap();
    ()
}

#[warn(dead_code)]
async fn _delete_file(db: &Surreal<Db>, id: RecordId) -> () {
    let _res: Option<FileResponse> = db.delete(id).await.unwrap();
    ()
}
