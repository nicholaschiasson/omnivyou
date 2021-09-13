use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::{
	html,
	web_sys::{window, File, KeyboardEvent},
	ChangeData, Component, ComponentLink, Html, ShouldRender,
};
use yew_octicons::{Icon, IconKind};

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
				let nav_buttons_class = "text-white bg-gray-700 text-opacity-0 bg-opacity-0 hover:text-opacity-100 hover:bg-opacity-70 transition duration-500 absolute inset-y-0 w-1/6 lg:w-1/12 text-9xl flex place-content-center place-items-center cursor-pointer select-none";
				let circle_buttons_class = "text-center text-white bg-gray-500 text-opacity-25 bg-opacity-25 hover:text-opacity-80 hover:bg-opacity-90 transition duration-500 absolute top-0 rounded-full text-4xl mx-6 my-4 p-2 h-32 w-32 lg:h-16 lg:w-16 flex place-content-center place-items-center cursor-pointer select-none";
				html! {
					<div class="bg-black text-white absolute inset-0 flex place-content-center place-items-center">
						<div class=format!("{} {}", nav_buttons_class, "left-0") onclick=self.link.callback(|_| Msg::PreviousFile)>
							{ Icon::new_sized(IconKind::ArrowLeft, 128) }
						</div>
						<Media class="max-h-screen max-w-screen" file=file.clone() />
						<div class=format!("{} {}", nav_buttons_class, "right-0") onclick=self.link.callback(|_| Msg::NextFile)>
							{ Icon::new_sized(IconKind::ArrowRight, 128) }
						</div>
						<div class=format!("{} {}", circle_buttons_class, "right-0") onclick=self.link.callback(|_| Msg::Quit)>
							{ Icon::new_sized(IconKind::X, 64) }
						</div>
						<div class=format!("{} {}", circle_buttons_class, "left-0")>
							{ Icon::new_sized(IconKind::Gear, 64) }
						</div>
					</div>
				}
			}
			None => html! {
				<div class="bg-gray-700 text-white absolute inset-0 flex flex-col place-content-center place-items-center">
					<h1 class="animate-bounce text-9xl m-2">{ "OmnivYou" }</h1>
					<label for="directory" class="cursor-pointer border-2 rounded text-7xl px-2 py-1 bg-white bg-opacity-0 hover:bg-opacity-100 hover:text-black transition duration-500 flex place-content-center place-items-center">
						<span class="pr-1 text-yellow-300">
							{ Icon::new_sized(IconKind::FileDirectoryFill, 64) }
						</span>
						{ "Select a folder" }
					</label>
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
					<div class="p-2 flex place-content-center place-items-center">
						<div class="w-16 h-16 lg:w-12 lg:h-12 p-2 m-2 border-2 rounded-full flex place-content-center place-items-center cursor-pointer">
							{ Icon::new_sized(IconKind::DeviceCameraVideo, 32) }
						</div>
						<div class="w-16 h-16 lg:w-12 lg:h-12 p-2 m-2 border-2 rounded-full flex place-content-center place-items-center cursor-pointer">
							{ Icon::new_sized(IconKind::Unmute, 32) }
						</div>
						<div class="w-16 h-16 lg:w-12 lg:h-12 p-2 m-2 border-2 rounded-full flex place-content-center place-items-center cursor-pointer">
							{ Icon::new_sized(IconKind::DeviceCamera, 32) }
						</div>
					</div>
				</div>
			},
		}
	}
}
