use crate::{queries, GithubClient, QueryStream};
use futures_util::future::LocalBoxFuture;

#[derive(Debug)]
pub struct QueryFileHistoryParams {
	// The owner of the repository to fetch
	pub owner: String,
	// The name of the repository
	pub repository: String,
	// The branch (or other github ref) the file can be found in
	pub ref_name: String,
	// The path of the file in the branch/ref
	pub file_path: String,
	// Used to chain multiple queries (e.g. delaying additional queries until after prior ones are finished).
	// If provided, the pages fetched will start at this cursor. If this cursor is indicates there are no more pages,
	// no query will be made.
	pub cursor: crate::Cursor,
	// How many commits should be fetched at a time
	pub page_size: usize,
	// How many pages of commits to fetch at most.
	// The returned number of commits will be no more than `max_pages * page_size`.
	pub max_pages: Option<usize>,
}

impl GithubClient {
	// Queries github for the commit history of a file.
	// Returns a list of commits, ordered from newest to oldest.
	pub fn query_file_history(
		&self,
		params: QueryFileHistoryParams,
	) -> LocalBoxFuture<'static, (Vec<queries::file_commit_history::Commit>, crate::Cursor)> {
		if params.cursor == crate::Cursor::End {
			return Box::pin(async { (Vec::new(), crate::Cursor::End) });
		}

		let query_args = queries::file_commit_history::Variables {
			owner: params.owner,
			repo_name: params.repository,
			ref_name: params.ref_name,
			path: params.file_path,
			amount: params.page_size as i64,
			cursor: params.cursor.value().cloned(),
		};
		let mut stream = QueryStream::<queries::FileCommitHistory>::new_cursor(
			self.client.clone(),
			query_args,
			params.cursor.clone(),
		);

		let max_commits = params.max_pages.map(|pages| pages * params.page_size);
		Box::pin(async move {
			use futures_util::StreamExt;
			let mut commits = Vec::new();
			while let Some(mut page) = stream.next().await {
				commits.append(&mut page.commits);

				if let Some(max_commits) = max_commits {
					if commits.len() >= max_commits {
						break;
					}
				}
			}
			(commits, stream.cursor().clone())
		})
	}
}
