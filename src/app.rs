use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{route::Route, router::Router, switch::Permissive};

use crate::routes::{home::Home, AppRoute};

pub struct Settings {
	autoplay: bool,
	autoplay_delay_audio: u32,
	autoplay_delay_image: u32,
	autoplay_delay_video: u32,
	toggle_audio: bool,
	toggle_image: bool,
	toggle_video: bool,
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
