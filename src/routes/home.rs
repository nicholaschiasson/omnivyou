use yew::{html, web_sys::File, ChangeData, Component, ComponentLink, Html, ShouldRender};

use crate::components::media::Media;

pub enum Msg {
	IndexDirectory(Vec<File>),
	NextFile,
	PreviousFile,
}

pub struct Home {
	files: Option<Vec<File>>,
	index: isize,
	link: ComponentLink<Self>,
}

impl Component for Home {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Home {
			files: None,
			index: 0,
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
		}
	}

	fn view(&self) -> Html {
		match &self.files {
			Some(files) => {
				let file = &files[(self.index as usize).rem_euclid(files.len())];
				let nav_buttons_class = "text-white bg-gray-700 text-opacity-0 bg-opacity-0 hover:text-opacity-100 hover:bg-opacity-70 transition duration-500 absolute inset-y-0 w-1/12 text-9xl flex place-content-center place-items-center cursor-pointer select-none";
				html! {
					<div class="bg-black absolute inset-0 flex place-content-center place-items-center">
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
				<div class="absolute inset-0 bg-purple-900">
					<label for="directory" class="cursor-pointer border">{ "Choose a directory..." }</label>
					<input id="directory" type="file" webkitdirectory="" class="hidden" onchange=self.link.callback(move |value| {
						let mut result = Vec::new();
						if let ChangeData::Files(files) = value {
								let files = js_sys::try_iter(&files)
										.unwrap()
										.unwrap()
										.map(|v| File::from(v.unwrap()))
										.filter(|f| !f.name().starts_with('.'))
										.filter(|f| f.type_().starts_with("audio/") || f.type_().starts_with("image/") || f.type_().starts_with("video/"));
								result.extend(files);
						}
						Msg::IndexDirectory(result)
					}) />
				</div>
			},
		}
	}
}
