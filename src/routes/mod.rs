use yew_router::{switch::Permissive, Switch};

pub mod home;

/// App routes
#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
	#[to = "/!"]
	Home,
	#[to = "/page-not-found!"]
	PageNotFound(Permissive<String>),
}
