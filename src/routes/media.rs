use yew::prelude::*;

pub struct Media;

impl Component for Media {
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Media {}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		true
	}

	fn view(&self) -> Html {
		html! {
			<div>
				{ "todo" }
			</div>
		}
	}
}
