use uuid::Uuid;

pub trait CanCreate<TModel, TRequest> {
    async fn create(&self, item: &TRequest) -> sqlx::Result<TModel>;
}

pub trait CanDelete<TModel, TRequest> {
    async fn delete(&self, item: &TRequest) -> sqlx::Result<TModel>;
}

pub trait CanGet<TModel, TRequest> {
    async fn get(&self, item: &TRequest) -> sqlx::Result<TModel>;
}

pub trait CanGetOrCreate<TModel, TRequest> {
    async fn get_or_create(&self, item: &TRequest) -> sqlx::Result<TModel>;
}

pub trait CanPaginate<TModel, TPage> {
    async fn paginate(&self, page: &TPage) -> sqlx::Result<Vec<TModel>>;
}

pub trait CanUpdate<TModel, TRequest> {
    async fn update(&self, id: &Uuid, item: &TRequest) -> sqlx::Result<TModel>;
}
