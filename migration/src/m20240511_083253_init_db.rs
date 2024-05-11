use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use sea_orm_migration::prelude::*;
use crate::get_model_from_json;
use crate::entity::prelude::*;
use crate::entity::{users,books,book_managers,borrow_records};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db=manager.get_connection();

        let users=get_model_from_json::<users::Model>(include_str!("json/Users.json"));
        let books=get_model_from_json::<books::Model>(include_str!("json/Books.json"));
        let book_managers=get_model_from_json::<book_managers::Model>(include_str!("json/BookManagers.json"));
        let borrow_records=get_model_from_json::<borrow_records::Model>(include_str!("json/BorrowRecords.json"));

        let users=users.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let books=books.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let book_managers=book_managers.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let borrow_records=borrow_records.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        Users::insert_many(users).exec(db).await?;
        Books::insert_many(books).exec(db).await?;
        BookManagers::insert_many(book_managers).exec(db).await?;
        BorrowRecords::insert_many(borrow_records).exec(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db=manager.get_connection();

        let users=get_model_from_json::<users::Model>(include_str!("json/Users.json"));
        let books=get_model_from_json::<books::Model>(include_str!("json/Books.json"));
        let book_managers=get_model_from_json::<book_managers::Model>(include_str!("json/BookManagers.json"));
        let borrow_records=get_model_from_json::<borrow_records::Model>(include_str!("json/BorrowRecords.json"));

        let users=users.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let books=books.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let book_managers=book_managers.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();

        let borrow_records=borrow_records.into_iter()
            .map(|x| x.into_active_model())
            .collect::<Vec<_>>();


        for it in borrow_records{
            it.delete(db).await?;
        }

        for it in book_managers{
            it.delete(db).await?;
        }

        for it in books{
            it.delete(db).await?;
        }

        for it in users{
            it.delete(db).await?;
        }


        Ok(())
    }
}

