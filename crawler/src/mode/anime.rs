use sqlx::{Pool, Postgres};
use tracing_unwrap::{OptionExt, ResultExt};

async fn crawl_batch(offset: i32) {
    
}

pub async fn crawl_anime(db: &Pool<Postgres>) {
    // Check previous crawl task status
    let previous_offset =
        match sqlx::query!(r#"SELECT * FROM "crawl_status" WHERE "crawl_mode" = 'anime';"#)
            .fetch_one(db)
            .await
        {
            Ok(result) => result
                .crawl_offset
                .expect_or_log("Corrupted database, missing crawl_offset for existing crawl_mode"),
            Err(_) => 0,
        };

    
}
