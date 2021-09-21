/**
 * This was the first version of the home component using the experimental FileSystemAccess API proposal in order to index directories.
 */

use std::{
	error::Error,
	fmt::{self, Debug, Display, Formatter}
};
use log::debug;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use yew::{Component, ComponentLink, Html, ShouldRender, html, web_sys::{FileSystemHandle, FileSystemDirectoryHandle, window}};
use yewtil::future::LinkFuture;

#[derive(Clone, Debug, PartialEq)]
pub struct JsFutureError {
	err: JsValue,
}

impl Display for JsFutureError {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
			Debug::fmt(&self.err, f)
	}
}

impl Error for JsFutureError {}

impl From<JsValue> for JsFutureError {
	fn from(value: JsValue) -> Self {
			Self { err: value }
	}
}

async fn pick_directory() -> Result<Option<FileSystemDirectoryHandle>, JsFutureError> {
	if let Some(w) = window() {
		let d = JsFuture::from(w.show_directory_picker()).await?;
		let directory: FileSystemDirectoryHandle = d.dyn_into().unwrap();
		return Ok(Some(directory));
	}
	Ok(None)
}

pub enum HomeMsg {
	ShowDirectoryPicker,
	IndexDirectory,
}

pub struct Home {
	link: ComponentLink<Self>,
}

impl Component for Home {
	type Message = HomeMsg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Home {
			link
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			HomeMsg::ShowDirectoryPicker => {
				self.link.send_future(async {
					if let Ok(Some(d)) = pick_directory().await {
						debug!("{:?}", d);
					}
					HomeMsg::IndexDirectory
				});
			}
			HomeMsg::IndexDirectory => {}
		}
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
				<button onclick=self.link.callback(|_| HomeMsg::ShowDirectoryPicker)>{ "Choose a directory..." }</button>
			</div>
		}
	}
}
