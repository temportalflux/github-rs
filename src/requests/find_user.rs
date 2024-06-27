use crate::{queries, GithubClient, GraphQLQueryExt};
use futures_util::future::LocalBoxFuture;

pub enum FindUserResponse {
	NotFound,
	Viewer,
	Valid,
}

impl GithubClient {
	pub fn find_user(&self, username: String) -> LocalBoxFuture<'static, Result<FindUserResponse, crate::Error>> {
		let query = queries::FindUser::post(self.client.clone(), queries::find_user::Variables { login: username });
		Box::pin(async move {
			let response = query.await?;
			log::debug!(target: "github", "{response:?}");
			let Some(user) = response.user else {
				return Ok(FindUserResponse::NotFound);
			};
			if user.is_viewer {
				return Ok(FindUserResponse::Viewer);
			}
			Ok(FindUserResponse::Valid)
		})
	}
}
