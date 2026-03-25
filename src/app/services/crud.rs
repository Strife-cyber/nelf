use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr,
    DeleteResult, EntityTrait, PrimaryKeyTrait, IntoActiveModel
};

#[async_trait]
pub trait CrudService<E>
where
    E: EntityTrait,
// Ensure standard thread safety for async boundaries
    E::Model: IntoActiveModel<E::ActiveModel> + Send + Sync,
    E::ActiveModel: ActiveModelTrait<Entity = E> + Send + Sync,
// Dynamically extract the Primary Key type (i32, Uuid, etc.) and ensure it is thread-safe
    <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: Send + Sync + Clone,
{
    async fn list_all(db: &DatabaseConnection) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(db).await
    }

    async fn find_by_id(
        db: &DatabaseConnection,
        id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<E::Model>, DbErr> {
        E::find_by_id(id).one(db).await
    }

    async fn create(db: &DatabaseConnection, model: E::ActiveModel) -> Result<E::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(db: &DatabaseConnection, model: E::ActiveModel) -> Result<E::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(db: &DatabaseConnection, model: E::ActiveModel) -> Result<DeleteResult, DbErr> {
        model.delete(db).await
    }
}
