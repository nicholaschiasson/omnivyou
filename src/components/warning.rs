use std::time::Duration;

use yew::{
	html,
	services::{timeout::TimeoutTask, TimeoutService},
	Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};
use yew_octicons::{Icon, IconKind};

pub enum Msg {
	Init,
	Wake,
	Live,
	Kill,
	Dead,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub class: String,
	#[prop_or_default]
	pub message: String,
	pub ondead: Callback<()>,
}

pub struct Warning {
	class: String,
	link: ComponentLink<Self>,
	message: String,
	on_dead: Callback<()>,
	state: Msg,
	timeout: Option<TimeoutTask>,
}

impl Component for Warning {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let callback = link.callback(|_| Msg::Init);
		Self {
			class: props.class,
			link,
			message: props.message,
			on_dead: props.ondead,
			state: Msg::Init,
			timeout: Some(TimeoutService::spawn(Duration::from_millis(100), callback)),
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let mut should_render = false;
		if self.class != props.class {
			should_render = !matches!(self.state, Msg::Dead);
			self.class = props.class;
		}
		if self.message != props.message {
			should_render = true;
			self.message = props.message;
			self.timeout = Some(TimeoutService::spawn(
				Duration::ZERO,
				self.link.callback(|_| Msg::Wake),
			));
		}
		should_render
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Init => {
				self.state = Msg::Wake;
				self.timeout = Some(TimeoutService::spawn(
					Duration::ZERO,
					self.link.callback(|_| Msg::Wake),
				));
				true
			}
			Msg::Wake => {
				self.state = Msg::Wake;
				self.timeout = Some(TimeoutService::spawn(
					Duration::from_millis(500),
					self.link.callback(|_| Msg::Live),
				));
				true
			}
			Msg::Live => {
				self.state = Msg::Live;
				self.timeout = Some(TimeoutService::spawn(
					Duration::from_secs(5),
					self.link.callback(|_| Msg::Kill),
				));
				true
			}
			Msg::Kill => {
				self.state = Msg::Kill;
				self.timeout = Some(TimeoutService::spawn(
					Duration::from_secs(2),
					self.link.callback(|_| Msg::Dead),
				));
				true
			}
			Msg::Dead => {
				self.state = Msg::Dead;
				self.message = String::new();
				self.on_dead.emit(());
				self.timeout = None;
				true
			}
		}
	}

	fn view(&self) -> Html {
		let class = format!("bg-yellow-200 border-yellow-400 border-2 rounded-lg text-3xl text-yellow-600 flex justify-content-left justify-items-left content-center items-center transition {}",
			match self.state {
				Msg::Init => {
					"duration-0 bg-opacity-0 border-opacity-0 text-opacity-0"
				},
				Msg::Wake|Msg::Live => "duration-500 bg-ocpacity-100 border-opacity-100 text-opacity-100",
				Msg::Kill|Msg::Dead => "duration-2000 bg-opacity-0 border-opacity-0 text-opacity-0",
			}
		);
		html! {
			<div class=format!("{} {}", class, &self.class)>
				<div class="inline-block m-1">
					{ Icon::new_sized(IconKind::Alert, 64) }
				</div>
				<div class="inline-block m-1">
					{ &self.message }
				</div>
			</div>
		}
	}
}
