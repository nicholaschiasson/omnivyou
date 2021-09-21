use std::time::Duration;

use js_sys::JsString;
use log::warn;
use wasm_bindgen::JsCast;
use yew::{
	html,
	services::timeout::{TimeoutService, TimeoutTask},
	web_sys::{File, Url},
	Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};

use crate::app::Settings;

#[derive(Clone, PartialEq)]
pub enum Type {
	Audio(String),
	Image(String),
	Video(String),
	Invalid(String),
}

impl From<String> for Type {
	fn from(s: String) -> Self {
		if s.starts_with("audio/") {
			Type::Audio(s)
		} else if s.starts_with("image/") {
			Type::Image(s)
		} else if s.starts_with("video/") {
			Type::Video(s)
		} else {
			Type::Invalid(s)
		}
	}
}

pub enum Msg {
	Ended,
	Seeked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub class: String,
	pub file: File,
	pub onended: Callback<()>,
	pub settings: Settings,
}

pub struct Media {
	class: String,
	ended: bool,
	file: File,
	link: ComponentLink<Self>,
	media_type: Type,
	on_ended: Callback<()>,
	settings: Settings,
	src: String,
	timeout: Option<TimeoutTask>,
}

impl Media {
	fn revoke_src(&self) {
		if let Err(err) = Url::revoke_object_url(&self.src) {
			if let Some(err_str) = err.dyn_ref::<JsString>() {
				warn!("{}", err_str);
			} else if let Some(err_code) = err.as_f64() {
				warn!("{}", err_code);
			} else {
				warn!("Failed to revoke object url '{}'", self.src);
			}
		}
	}
}

impl Component for Media {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let media_type = Type::from(props.file.type_());
		let timeout = match media_type {
			Type::Image(_) => {
				if props.settings.toggle_image_autoplay {
					Some(TimeoutService::spawn(
						props.settings.config_image_autoplay_delay,
						props.onended.clone(),
					))
				} else {
					None
				}
			}
			_ => None,
		};
		Self {
			class: props.class,
			ended: false,
			file: props.file.clone(),
			link,
			media_type,
			on_ended: props.onended.clone(),
			settings: props.settings,
			src: Url::create_object_url_with_blob(&props.file.slice().ok().unwrap()).unwrap(),
			timeout,
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		let mut should_render = false;
		if self.class != props.class {
			should_render = true;
			self.class = props.class;
		}
		let media_type = Type::from(props.file.type_());
		let new_media_type = self.media_type != media_type;
		let new_media = new_media_type || self.file != props.file;
		if new_media {
			should_render = true;
			self.revoke_src();
			self.ended = false;
			self.file = props.file.clone();
			self.media_type = media_type.clone();
			self.src = Url::create_object_url_with_blob(&props.file.slice().ok().unwrap()).unwrap();
		}
		let new_callback = self.on_ended != props.onended;
		if new_callback {
			should_render = true;
			self.on_ended = props.onended;
		}
		let new_settings = self.settings != props.settings;
		let new_audio_setting = self.settings.toggle_audio_autoplay
			!= props.settings.toggle_audio_autoplay
			|| self.settings.config_audio_autoplay_delay != props.settings.config_audio_autoplay_delay;
		let new_image_setting = self.settings.toggle_image_autoplay
			!= props.settings.toggle_image_autoplay
			|| self.settings.config_image_autoplay_delay != props.settings.config_image_autoplay_delay;
		let new_video_setting = self.settings.toggle_video_autoplay
			!= props.settings.toggle_video_autoplay
			|| self.settings.config_video_autoplay_delay != props.settings.config_video_autoplay_delay;
		if new_settings {
			if !new_media_type && !new_callback {
				match media_type {
					Type::Audio(_) => {
						if new_audio_setting {
							self.timeout =
								if props.settings.toggle_audio_autoplay && self.ended {
									Some(TimeoutService::spawn(
										props.settings.config_audio_autoplay_delay,
										self.on_ended.clone(),
									))
								} else {
									None
								};
						}
					}
					Type::Video(_) => {
						if new_video_setting {
							self.timeout =
								if props.settings.toggle_video_autoplay && self.ended {
									Some(TimeoutService::spawn(
										props.settings.config_video_autoplay_delay,
										self.on_ended.clone(),
									))
								} else {
									None
								};
						}
					}
					_ => (),
				}
			}
			self.settings = props.settings;
		}
		if matches!(media_type, Type::Image(_)) {
			if props.settings.toggle_image_autoplay {
				if new_media || new_callback || new_image_setting {
					self.timeout = Some(TimeoutService::spawn(
						props.settings.config_image_autoplay_delay,
						self.on_ended.clone(),
					));
				}
			} else {
				self.timeout = None;
			}
		}
		should_render
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Ended => {
				self.ended = true;
				let (set_timeout, delay) = match self.media_type {
					Type::Audio(_) => (
						self.settings.toggle_audio_autoplay,
						self.settings.config_audio_autoplay_delay,
					),
					Type::Image(_) => (
						self.settings.toggle_image_autoplay,
						self.settings.config_image_autoplay_delay,
					),
					Type::Video(_) => (
						self.settings.toggle_video_autoplay,
						self.settings.config_video_autoplay_delay,
					),
					Type::Invalid(_) => (false, Duration::ZERO),
				};
				if set_timeout {
					self.timeout = Some(TimeoutService::spawn(delay, self.on_ended.clone()));
				}
				false
			}
			Msg::Seeked => {
				self.ended = false;
				self.timeout = None;
				false
			}
		}
	}

	fn view(&self) -> Html {
		match &self.media_type {
			Type::Audio(_) => {
				html!(<audio autoplay="" class=&self.class controls=true src=self.src.clone() onended=self.link.callback(|_| Msg::Ended) onseeked=self.link.callback(|_| Msg::Seeked) />)
			}
			Type::Image(_) => {
				html!(<img class=&self.class src=self.src.clone() />)
			}
			Type::Video(_) => {
				html!(<video autoplay="" class=&self.class controls=true src=self.src.clone() onended=self.link.callback(|_| Msg::Ended) onseeked=self.link.callback(|_| Msg::Seeked) />)
			}
			Type::Invalid(t) => html!(format!("Invalid media type '{}'", t)),
		}
	}

	fn destroy(&mut self) {
		self.revoke_src();
	}
}
