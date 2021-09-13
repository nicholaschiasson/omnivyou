use js_sys::JsString;
use log::warn;
use wasm_bindgen::JsCast;
use yew::{
	html,
	web_sys::{File, Url},
	Component, ComponentLink, Html, Properties, ShouldRender,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub class: String,
	pub file: File,
}

pub struct Menu {
	class: String,
	media_type: Type,
	src: String,
}

impl Component for Menu {
	type Message = ();
	type Properties = Props;

	fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
		Self {
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
