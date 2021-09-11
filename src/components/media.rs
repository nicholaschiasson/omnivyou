use js_sys::JsString;
use log::warn;
use wasm_bindgen::JsCast;
use yew::{
	html,
	web_sys::{File, Url},
	Component, ComponentLink, Html, Properties, ShouldRender,
};

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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	pub class: String,
	pub file: File,
}

pub struct Media {
	class: String,
	media_type: Type,
	src: String,
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
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Media {
			class: props.class,
			media_type: Type::from(props.file.type_()),
			src: Url::create_object_url_with_blob(&props.file.slice().ok().unwrap()).unwrap(),
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.revoke_src();
		self.class = props.class;
		self.media_type = Type::from(props.file.type_());
		self.src = Url::create_object_url_with_blob(&props.file.slice().ok().unwrap()).unwrap();
		true
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		match &self.media_type {
			Type::Audio(_) => {
				html!(<audio autoplay="" class=&self.class controls=true src=self.src.clone() />)
			}
			Type::Image(_) => html!(<img class=&self.class src=self.src.clone() />),
			Type::Video(_) => {
				html!(<video autoplay="" class=&self.class controls=true src=self.src.clone() />)
			}
			Type::Invalid(t) => html!(format!("Invalid media type '{}'", t)),
		}
	}

	fn destroy(&mut self) {
		self.revoke_src();
	}
}
