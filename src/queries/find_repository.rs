use graphql_client::GraphQLQuery;

type GitObjectID = String;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/graphql/schema.graphql",
	query_path = "src/queries/graphql/query_find_repository.graphql",
	response_derives = "Debug"
)]
pub struct FindRepository;
pub use find_repository::ResponseData;
pub use find_repository::Variables;
