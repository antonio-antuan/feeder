use crate::db::models::UserFolder;
use crate::db::Pool;
use crate::result::Result;

pub async fn get_user_folders(db_pool: &Pool, user_id: i32) -> Result<Vec<UserFolder>> {
    Ok(sqlx::query_as!(
        UserFolder,
        "SELECT * FROM user_folders WHERE user_id = $1",
        user_id
    )
    .fetch_all(db_pool)
    .await?)
}

pub async fn add_user_folder(
    db_pool: &Pool,
    user_id: i32,
    name: String,
    parent_folder_id: Option<i32>,
) -> Result<i32> {
    let rec = sqlx::query!(
        r#"INSERT INTO user_folders (name, user_id, parent_folder_id) VALUES ($1, $2, $3) 
        ON CONFLICT (name, user_id) DO UPDATE SET parent_folder_id = EXCLUDED.parent_folder_id
        RETURNING id
        "#,
        name,
        user_id,
        parent_folder_id
    )
    .fetch_one(db_pool)
    .await?;
    Ok(rec.id)
}

pub async fn remove_user_folder(db_pool: &Pool, user_id: i32, folder_id: i32) -> Result<()> {
    sqlx::query!(
        "DELETE FROM user_folders WHERE user_id = $1 AND id = $2",
        user_id,
        folder_id
    )
    .execute(db_pool)
    .await?;
    Ok(())
}
