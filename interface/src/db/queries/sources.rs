use crate::db::{models::SourceWithMeta, Pool};
use crate::result::Result;

pub async fn unsubscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<()> {
    sqlx::query!(
        r#"
    DELETE FROM records_user_settings 
    WHERE 
        record_id IN (SELECT id FROM records WHERE source_id = $1) 
        AND user_id = $2
    "#,
        source_id,
        user_id
    )
    .execute(db_pool)
    .await?;
    sqlx::query!(
        r#"
    DELETE FROM sources_user_settings 
    WHERE 
        source_id = $1 AND user_id = $2
    "#,
        source_id,
        user_id
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn subscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<()> {
    let sus = sqlx::query!(
        "INSERT INTO sources_user_settings (source_id, user_id) VALUES ($1, $2) RETURNING id",
        source_id,
        user_id
    )
    .fetch_one(db_pool)
    .await?;

    sqlx::query!(
        "INSERT INTO user_source_to_folder (user_source_id, folder_id) VALUES ($1, (SELECT min(id) FROM user_folders WHERE user_id = $2))",
        sus.id, user_id
    ).execute(db_pool).await?;
    Ok(())
}

pub async fn get_for_user(db_pool: &Pool, user_id: i32) -> Result<Vec<SourceWithMeta>> {
    let sources = sqlx::query_as!(
        SourceWithMeta,
        r#"SELECT 
        s.id, s.name, s.origin, s.kind, s.image, s.last_scrape_time, s.external_link, usf.folder_id 
        FROM sources s
        INNER JOIN sources_user_settings sus ON sus.id = s.id
        LEFT JOIN user_source_to_folder usf ON usf.id = sus.id
        WHERE sus.user_id = $1 
        "#,
        user_id
    )
    .fetch_all(db_pool)
    .await?;
    Ok(sources)
}

pub async fn move_to_folder(
    db_pool: &Pool,
    user_id: i32,
    source_id: i32,
    folder_id: i32,
) -> Result<()> {
    sqlx::query!(r#"
    UPDATE user_source_to_folder
    SET folder_id = $1
    WHERE user_source_id = (SELECT id FROM sources_user_settings WHERE source_id = $2 AND user_id = $3 LIMIT 1)"#,
    folder_id, source_id, user_id).execute(db_pool)
        .await?;
    Ok(())
}
