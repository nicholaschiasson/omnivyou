use std::time::Duration;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{route::Route, router::Router, switch::Permissive};

use crate::routes::{home::Home, AppRoute};

pub const MAX_DELAY_SECONDS: Duration = Duration::from_millis(u32::MAX as u64);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Settings {
	pub config_audio_autoplay_delay: Duration,
	pub config_image_autoplay_delay: Duration,
	pub config_video_autoplay_delay: Duration,
	pub toggle_audio: bool,
	pub toggle_audio_autoplay: bool,
	pub toggle_image: bool,
	pub toggle_image_autoplay: bool,
	pub toggle_video: bool,
	pub toggle_video_autoplay: bool,
}

impl Settings {
	pub fn new() -> Self {
		Self {
			config_audio_autoplay_delay: Duration::ZERO,
			config_image_autoplay_delay: Duration::from_secs(2),
			config_video_autoplay_delay: Duration::ZERO,
			toggle_audio: true,
			toggle_audio_autoplay: true,
			toggle_image: true,
			toggle_image_autoplay: true,
			toggle_video: true,
			toggle_video_autoplay: true,
		}
	}
}

pub struct App;

impl Component for App {
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<>
				<Router<AppRoute, ()>
					render = Router::render(|switch: AppRoute| {
						match switch {
							AppRoute::Home => html!{ <Home/> },
							AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found!"},
							AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found!", missed_route)},
						}
					} )
					redirect = Router::redirect(|route: Route<()>| {
						AppRoute::PageNotFound(Permissive(Some(route.route)))
					})
				/>
			</>
		}
	}
}
