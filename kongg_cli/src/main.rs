use kongg_shared::helpers::{crud::get_file, surreal_init::init};

#[tokio::main]
async fn main() {
    let db = init().await.unwrap();
    let files = get_file(&db).await;
    println!("----------> START");
    println!("|event type | location| time & date |");
    println!("-----------------------------------");
    for file in files {
        println!(
            "|{:?} | {} | {} |",
            file.event_type, file.path, file.timestamp
        )
    }
    println!("----------> END");
}
