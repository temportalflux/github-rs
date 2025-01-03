use crate::{Cursor, StreamableQuery};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/graphql/schema.graphql",
	query_path = "src/queries/graphql/query_find_orgs.graphql",
	response_derives = "Debug"
)]
pub struct FindOrgs;
pub use find_orgs::Variables;

impl StreamableQuery<FindOrgs> for FindOrgs {
	type Item = Vec<String>;

	fn update_variables(vars: &find_orgs::Variables, cursor: Option<String>) -> find_orgs::Variables {
		find_orgs::Variables {
			cursor,
			amount: vars.amount,
		}
	}

	fn next(data: find_orgs::ResponseData) -> (Cursor, Self::Item) {
		let page_info = data.viewer.organizations.page_info;
		let cursor = Cursor::new(page_info.has_next_page, page_info.end_cursor);

		let mut output = Vec::new();
		if let Some(org_nodes) = data.viewer.organizations.nodes {
			output.reserve(org_nodes.len());
			for org_node in org_nodes {
				let Some(org) = org_node else {
					continue;
				};
				output.push(org.login);
			}
		}

		(cursor, output)
	}
}
