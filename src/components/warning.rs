use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_octicons::{Icon, IconKind};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub class: String,
	pub message: String,
}

pub struct Warning {
	class: String,
	message: String,
}

impl Component for Warning {
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {
			class: props.class,
			message: props.message,
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let mut should_render = false;
		if self.class != props.class {
			should_render = true;
			self.class = props.class;
		}
		if self.message != props.message {
			should_render = true;
			self.message = props.message;
		}
		should_render
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class=&self.class>
				<div class="inline-block">
					{ Icon::new_sized(IconKind::Alert, 64) }
				</div>
				<div class="inline-block">
					{ &self.message }
				</div>
			</div>
		}
	}
}
