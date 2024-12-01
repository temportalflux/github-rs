use crate::{
	queries::{self, find_repository::find_repository::FindRepositoryRepositoryObject},
	GithubClient, GraphQLQueryExt, RepositoryMetadata,
};
use futures_util::future::LocalBoxFuture;

impl GithubClient {
	pub fn find_repository(
		&self,
		owner: &String,
		repository: &String,
	) -> LocalBoxFuture<'static, Result<Option<RepositoryMetadata>, crate::Error>> {
		let query = queries::FindRepository::post(
			self.client.clone(),
			queries::find_repository::Variables {
				owner: owner.clone(),
				name: repository.clone(),
			},
		);
		Box::pin(async move {
			let response: queries::find_repository::ResponseData = query.await?;
			log::debug!(target: "github", "{response:?}");

			let Some(repo) = response.repository else {
				return Ok(None);
			};

			// All repositories must be initialized (default branch has contents), otherwise they are ignored
			let Some(FindRepositoryRepositoryObject::Tree(default_branch_tree)) = repo.object else {
				return Ok(None);
			};
			let tree_id = default_branch_tree.oid;
			let Some(root_tree_entries) = default_branch_tree.entries else {
				return Ok(None);
			};
			let mut root_trees = Vec::new();
			for entry in root_tree_entries {
				// if this entry is a directory, then it is likely the root for a system in the module.
				// if its not a tree (directory), then we dont care right now.
				if entry.type_ != "tree" {
					continue;
				}
				root_trees.push(entry.name);
			}

			return Ok(Some(RepositoryMetadata {
				owner: repo.owner.login,
				name: repo.name,
				is_private: repo.is_private,
				version: repo.default_branch_ref.unwrap().target.oid.to_string(),
				root_trees,
				tree_id,
			}));
		})
	}
}
