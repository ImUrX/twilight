use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

/// Get a user's information by id.
#[must_use = "requests must be configured and executed"]
pub struct GetUser<'a> {
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self { http, user_id }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<User> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetUser {
            user_id: self.user_id.get(),
        }))
    }
}
