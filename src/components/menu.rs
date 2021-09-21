use yew::{html, Callback, ChangeData, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_octicons::{Icon, IconKind};

use crate::app::{self, Settings};

pub enum Msg {
	Close,
	ToggleVisible,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
	#[prop_or_default]
	pub button_class: String,
	#[prop_or_default]
	pub class: String,
	pub settings: Settings,
	pub toggle_audio_callback: Callback<ChangeData>,
	pub toggle_audio_autoplay_callback: Callback<ChangeData>,
	pub config_audio_autoplay_delay_callback: Callback<ChangeData>,
	pub toggle_image_callback: Callback<ChangeData>,
	pub toggle_image_autoplay_callback: Callback<ChangeData>,
	pub config_image_autoplay_delay_callback: Callback<ChangeData>,
	pub toggle_video_callback: Callback<ChangeData>,
	pub toggle_video_autoplay_callback: Callback<ChangeData>,
	pub config_video_autoplay_delay_callback: Callback<ChangeData>,
}

pub struct Menu {
	button_class: String,
	class: String,
	link: ComponentLink<Self>,
	settings: Settings,
	toggle_audio_callback: Callback<ChangeData>,
	toggle_audio_autoplay_callback: Callback<ChangeData>,
	config_audio_autoplay_delay_callback: Callback<ChangeData>,
	toggle_image_callback: Callback<ChangeData>,
	toggle_image_autoplay_callback: Callback<ChangeData>,
	config_image_autoplay_delay_callback: Callback<ChangeData>,
	toggle_video_callback: Callback<ChangeData>,
	toggle_video_autoplay_callback: Callback<ChangeData>,
	config_video_autoplay_delay_callback: Callback<ChangeData>,
	visible: bool,
}

impl Component for Menu {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			button_class: props.button_class,
			class: props.class,
			link,
			settings: props.settings,
			toggle_audio_callback: props.toggle_audio_callback,
			toggle_audio_autoplay_callback: props.toggle_audio_autoplay_callback,
			config_audio_autoplay_delay_callback: props.config_audio_autoplay_delay_callback,
			toggle_image_callback: props.toggle_image_callback,
			toggle_image_autoplay_callback: props.toggle_image_autoplay_callback,
			config_image_autoplay_delay_callback: props.config_image_autoplay_delay_callback,
			toggle_video_callback: props.toggle_video_callback,
			toggle_video_autoplay_callback: props.toggle_video_autoplay_callback,
			config_video_autoplay_delay_callback: props.config_video_autoplay_delay_callback,
			visible: false,
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.button_class = props.button_class;
		self.class = props.class;
		self.settings = props.settings;
		self.toggle_audio_callback = props.toggle_audio_callback;
		self.toggle_audio_autoplay_callback = props.toggle_audio_autoplay_callback;
		self.config_audio_autoplay_delay_callback = props.config_audio_autoplay_delay_callback;
		self.toggle_image_callback = props.toggle_image_callback;
		self.toggle_image_autoplay_callback = props.toggle_image_autoplay_callback;
		self.config_image_autoplay_delay_callback = props.config_image_autoplay_delay_callback;
		self.toggle_video_callback = props.toggle_video_callback;
		self.toggle_video_autoplay_callback = props.toggle_video_autoplay_callback;
		self.config_video_autoplay_delay_callback = props.config_video_autoplay_delay_callback;
		true
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Close => {
				self.visible = false;
				true
			}
			Msg::ToggleVisible => {
				self.visible = !self.visible;
				true
			}
		}
	}

	fn view(&self) -> Html {
		let panel_class = "bg-gray-800 text-white absolute inset-y-0 w-4/5 md:w-3/5 lg:w-2/6 flex flex-col place-content-center place-items-center select-none transition left-0 divide-y";
		let form_class =
			"px-8 w-full h-full overflow-y-auto flex flex-col divide-y place-content-center";
		let fieldset_class = "flex flex-col border-t";
		let field_class =
			"flex bg-white bg-opacity-0 hover:bg-opacity-30 transition text-6xl lg:text-4xl p-4";
		let input_checkbox = "self-center scale-500 lg:scale-400 mr-4";
		let input_number = "bg-white bg-opacity-0 text-right outline-none disabled:opacity-50 w-1/3";
		html! {
			<div class=&self.class>
				{if self.visible {
					html! {<div class="absolute bg-opacity-0 inset-0" onclick=self.link.callback(|_| Msg::Close)></div>}
				} else {
					html!()
				}}
				<div class=format!("{} {}", panel_class, if self.visible { "translate-x-0" } else { "-translate-x-full" })>
					<form class=form_class>
						<fieldset class=fieldset_class>
							<legend>{ "Video" }</legend>
							<div class=field_class>
								<label class="flex-1">{ "Autoplay" }</label>
								<input type="checkbox" class=input_checkbox
									checked=self.settings.toggle_video_autoplay
									disabled=!self.settings.toggle_video
									onchange=&self.toggle_video_autoplay_callback
								/>
							</div>
							<div class=field_class>
								<label class="flex-1"
									disabled={ !self.settings.toggle_video || !self.settings.toggle_video_autoplay }>
									{ "Delay" }
								</label>
								<input type="number" class=input_number
									disabled={ !self.settings.toggle_video || !self.settings.toggle_video_autoplay }
									min=0
									max=format!("{}", app::MAX_DELAY_SECONDS.as_secs())
									pattern="^\\d{1,4}$"
									value=format!("{}", self.settings.config_video_autoplay_delay.as_secs())
									onchange=&self.config_video_autoplay_delay_callback
								/>
							</div>
						</fieldset>
						<fieldset class=fieldset_class>
							<legend>{ "Audio" }</legend>
							<div class=field_class>
								<label class="flex-1">{ "Autoplay" }</label>
								<input type="checkbox" class=input_checkbox
									checked=self.settings.toggle_audio_autoplay
									disabled=!self.settings.toggle_audio
									onchange=&self.toggle_audio_autoplay_callback
								/>
							</div>
							<div class=field_class>
								<label class="flex-1"
									disabled={ !self.settings.toggle_audio || !self.settings.toggle_audio_autoplay }>
									{ "Delay" }
								</label>
								<input type="number" class=input_number
									disabled={ !self.settings.toggle_audio || !self.settings.toggle_audio_autoplay }
									min=0
									max=format!("{}", app::MAX_DELAY_SECONDS.as_secs())
									pattern="^\\d{1,4}$"
									value=format!("{}", self.settings.config_audio_autoplay_delay.as_secs())
									onchange=&self.config_audio_autoplay_delay_callback
								/>
							</div>
						</fieldset>
						<fieldset class=fieldset_class>
							<legend>{ "Photos" }</legend>
							<div class=field_class>
								<label class="flex-1">{ "Autoplay" }</label>
								<input type="checkbox" class=input_checkbox
									checked=self.settings.toggle_image_autoplay
									disabled=!self.settings.toggle_image
									onchange=&self.toggle_image_autoplay_callback
								/>
							</div>
							<div class=field_class>
								<label class="flex-1"
									disabled={ !self.settings.toggle_image || !self.settings.toggle_image_autoplay }>
									{ "Delay" }
								</label>
								<input type="number" class=input_number
									disabled={ !self.settings.toggle_image || !self.settings.toggle_image_autoplay }
									min=1
									max=format!("{}", app::MAX_DELAY_SECONDS.as_secs())
									pattern="^\\d{1,4}$"
									value=format!("{}", self.settings.config_image_autoplay_delay.as_secs())
									onchange=&self.config_image_autoplay_delay_callback
								/>
							</div>
						</fieldset>
					</form>
				</div>
				<div class=&self.button_class onclick=self.link.callback(|_| Msg::ToggleVisible)>
					{ Icon::new_sized(IconKind::Gear, 64) }
				</div>
			</div>
		}
	}
}
