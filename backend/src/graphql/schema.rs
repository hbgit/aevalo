use async_graphql::{Schema, EmptySubscription};
use super::queries::QueryRoot;
use super::mutations::MutationRoot;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish()
}
