use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod home;
pub mod media;

/// App routes
#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
	#[to = "/!"]
	Home,
	#[to = "{/*:path}"]
	Media(String),
	#[to = "/page-not-found!"]
	PageNotFound(Permissive<String>),
}
