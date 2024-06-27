use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/graphql/schema.graphql",
	query_path = "src/queries/graphql/query_find_user.graphql",
	response_derives = "Debug"
)]
pub struct FindUser;
pub use find_user::Variables;
