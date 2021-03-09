use diesel::prelude::*;
use diesel::{delete, insert_into};

use crate::db::models::UserFolder;
use crate::db::Pool;
use crate::result::Result;
use crate::schema::user_folders;
use diesel::pg::upsert::excluded;
use tokio_diesel::*;

pub async fn get_user_folders(db_pool: &Pool, user_id: i32) -> Result<Vec<UserFolder>> {
    Ok(user_folders::table
        .filter(user_folders::user_id.eq(user_id))
        .load_async::<UserFolder>(db_pool)
        .await?)
}

pub async fn add_user_folder(
    db_pool: &Pool,
    user_id: i32,
    name: String,
    parent_folder_id: Option<i32>,
) -> Result<()> {
    insert_into(user_folders::table)
        .values((
            user_folders::user_id.eq(user_id),
            user_folders::name.eq(name),
            user_folders::parent_folder.eq(parent_folder_id),
        ))
        .on_conflict((user_folders::name, user_folders::user_id))
        .do_update()
        .set((user_folders::parent_folder.eq(excluded(user_folders::parent_folder)),))
        .execute_async(db_pool)
        .await?;
    Ok(())
}

pub async fn remove_user_folder(db_pool: &Pool, user_id: i32, folder_id: i32) -> Result<()> {
    delete(
        user_folders::table.filter(
            user_folders::id
                .eq(folder_id)
                .and(user_folders::user_id.eq(user_id)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    Ok(())
}
