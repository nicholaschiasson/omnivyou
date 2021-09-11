use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::{
	html,
	web_sys::{window, File, KeyboardEvent},
	ChangeData, Component, ComponentLink, Html, ShouldRender,
};

use crate::components::media::{Media, Type};

pub enum Msg {
	IndexDirectory(Vec<File>),
	NextFile,
	PreviousFile,
	Quit,
	None,
}

pub struct Home {
	files: Option<Vec<File>>,
	index: isize,
	keydown_listener: Option<EventListener>,
	link: ComponentLink<Self>,
}

impl Component for Home {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			files: None,
			index: 0,
			keydown_listener: None,
			link,
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::IndexDirectory(files) => {
				if files.len() > 0 {
					self.files = Some(files);
					self.index = 0;
					if let Some(w) = window() {
						let onkeydown = self
							.link
							.callback(|e: KeyboardEvent| match e.key().as_str() {
								"ArrowLeft" => Msg::PreviousFile,
								"ArrowRight" => Msg::NextFile,
								"Escape" => Msg::Quit,
								_ => Msg::None,
							});
						self.keydown_listener = Some(EventListener::new(&w, "keydown", move |e| {
							onkeydown.emit(e.dyn_ref::<KeyboardEvent>().unwrap().clone())
						}));
					}
					return true;
				}
				false
			}
			Msg::NextFile => {
				if let Some(files) = &self.files {
					self.index = (self.index + 1).rem_euclid(files.len() as isize);
					return true;
				}
				false
			}
			Msg::PreviousFile => {
				if let Some(files) = &self.files {
					self.index = (self.index - 1).rem_euclid(files.len() as isize);
					return true;
				}
				false
			}
			Msg::Quit => {
				self.files = None;
				self.index = 0;
				self.keydown_listener = None;
				true
			}
			Msg::None => false,
		}
	}

	fn view(&self) -> Html {
		match &self.files {
			Some(files) => {
				let file = &files[(self.index as usize).rem_euclid(files.len())];
				let nav_buttons_class = "text-white bg-gray-700 text-opacity-0 bg-opacity-0 hover:text-opacity-100 hover:bg-opacity-70 transition duration-500 absolute inset-y-0 w-1/12 text-9xl flex place-content-center place-items-center cursor-pointer select-none";
				html! {
					<div class="bg-black text-white absolute inset-0 flex place-content-center place-items-center">
						<Media class="max-h-screen max-w-screen" file=file.clone() />
						<div class=format!("{} {}", nav_buttons_class, "left-0") onclick=self.link.callback(|_| Msg::PreviousFile)>
							<p>{ "←" }</p>
						</div>
						<div class=format!("{} {}", nav_buttons_class, "right-0") onclick=self.link.callback(|_| Msg::NextFile)>
							<p>{ "→" }</p>
						</div>
					</div>
				}
			}
			None => html! {
				<div class="bg-gray-700 text-white absolute inset-0 flex flex-col place-content-center place-items-center">
					<h1 class="animate-bounce text-5xl m-2">{ "OmnivYou" }</h1>
					<label for="directory" class="cursor-pointer border rounded text-2xl p-1 bg-white bg-opacity-0 hover:bg-opacity-100 hover:text-black transition duration-500">{ "Choose a directory..." }</label>
					<input id="directory" type="file" webkitdirectory="" class="hidden" onchange=self.link.callback(move |value| {
						let mut result = Vec::new();
						if let ChangeData::Files(files) = value {
								let files = js_sys::try_iter(&files)
										.unwrap()
										.unwrap()
										.map(|v| File::from(v.unwrap()))
										.filter(|f| !f.name().starts_with('.'))
										.filter(|f| !matches!(Type::from(f.type_()), Type::Invalid(_)));
								result.extend(files);
						}
						Msg::IndexDirectory(result)
					}) />
				</div>
			},
		}
	}
}
