use log::debug;
use yew::{ChangeData, Component, ComponentLink, Html, ShouldRender, html, web_sys::{File, Url}};

pub enum HomeMsg {
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
	type Message = HomeMsg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Home {
			files: None,
			index: 0,
			link
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			HomeMsg::IndexDirectory(files) => {
				self.files = Some(files);
				true
			}
			HomeMsg::NextFile => {
				if let Some(files) = &self.files {
					self.index = (self.index + 1).rem_euclid(files.len() as isize);
					return true;
				}
				false
			}
			HomeMsg::PreviousFile => {
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
				let src_url = Url::create_object_url_with_blob(&file.slice().ok().unwrap()).ok();
				let nav_buttons_class = "text-white bg-gray-700 text-opacity-0 bg-opacity-0 hover:text-opacity-100 hover:bg-opacity-70 transition duration-500 absolute inset-y-0 w-1/12 text-9xl flex place-content-center place-items-center cursor-pointer select-none";
				html! {
					// TODO: Figure out how to get content to stretch to fill if too small.
					<div class="bg-black">
						{
							if file.type_().starts_with("audio/") {
								html! {
									<audio
										autoplay=""
										class="max-h-screen max-w-screen mx-auto object-center"
										controls=true
										src={ src_url }
									/>
								}
							} else if file.type_().starts_with("image/") {
								html! {
									<img class="max-h-screen max-w-screen mx-auto object-center" src={ src_url } />
								}
							} else if file.type_().starts_with("video/") {
								html! {
									<video
										autoplay=""
										class="max-h-screen max-w-screen mx-auto object-center"
										controls=true
										src={ src_url }
									/>
								}
							} else {
								self.link.send_message(HomeMsg::NextFile);
								html!()
							}
						}
						<div class=format!("{} {}", nav_buttons_class, "left-0") onclick=self.link.callback(|_| HomeMsg::PreviousFile)>
							<p>{ "←" }</p>
						</div>
						<div class=format!("{} {}", nav_buttons_class, "right-0") onclick=self.link.callback(|_| HomeMsg::NextFile)>
							<p>{ "→" }</p>
						</div>
					</div>
				}
			},
			None => html! {
				<div>
					<input type="file" webkitdirectory="" onchange=self.link.callback(move |value| {
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
						HomeMsg::IndexDirectory(result)
					}) />
				</div>
			},
		}
	}
}
