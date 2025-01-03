use crate::{Cursor, StreamableQuery};
use file_commit_history::FileCommitHistoryRepositoryRefTarget;
use graphql_client::GraphQLQuery;

type GitObjectID = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/queries/graphql/schema.graphql",
	query_path = "src/queries/graphql/query_file_commit_history.graphql",
	response_derives = "Debug",
	variables_derives = "Debug"
)]
pub struct FileCommitHistory;
pub use file_commit_history::Variables;

#[derive(Default, Debug)]
pub struct Page {
	pub commits: Vec<Commit>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Commit {
	// the sha (oid in the github api) of the commit
	pub id: String,
	pub authored_date: String,
	pub commited_date: String,
	pub message_headline: String,
	pub message_body: String,
}

impl StreamableQuery<FileCommitHistory> for FileCommitHistory {
	type Item = Page;

	fn update_variables(
		vars: &file_commit_history::Variables,
		cursor: Option<String>,
	) -> file_commit_history::Variables {
		file_commit_history::Variables {
			owner: vars.owner.clone(),
			repo_name: vars.repo_name.clone(),
			ref_name: vars.ref_name.clone(),
			path: vars.path.clone(),
			amount: vars.amount,
			cursor,
		}
	}

	fn next(data: file_commit_history::ResponseData) -> (Cursor, Self::Item) {
		let mut cursor = Cursor::End;
		let mut page = Page::default();
		let Some(repository) = data.repository else {
			return (cursor, page);
		};
		let Some(branch_ref) = repository.ref_ else {
			return (cursor, page);
		};

		let FileCommitHistoryRepositoryRefTarget::Commit(commits_query) = branch_ref.target else {
			return (cursor, page);
		};
		let page_info = commits_query.history.page_info;
		cursor = Cursor::new(page_info.has_next_page, page_info.end_cursor);

		let Some(history_nodes) = commits_query.history.nodes else {
			return (cursor, page);
		};
		for node in history_nodes {
			let Some(commit) = node else { continue };
			page.commits.push(Commit {
				id: commit.oid,
				authored_date: commit.authored_date,
				commited_date: commit.committed_date,
				message_headline: commit.message_headline,
				message_body: commit.message_body,
			});
		}

		(cursor, page)
	}
}
