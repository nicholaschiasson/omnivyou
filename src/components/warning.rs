use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_octicons::{Icon, IconKind};

pub struct Warning;

impl Component for Warning {
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
		html!()
	}
}
