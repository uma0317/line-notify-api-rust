pub enum Parameter {
	Message(String),
	ImageThumbnail(String),
	ImageFullsize(String),
	ImageFile(String),
	StickerPackageId(u32),
	NotificationDisabled(bool),
}

pub struct NotifyParam {
	pub message: String,
	pub image_thumbnail: Option<String>,
	pub image_fullsize: Option<String>,
	pub image_file: Option<String>,
	pub sticker_package_id: Option<u32>,
	pub notification_disabled: Option<bool>,
}

impl NotifyParam {
	pub fn create(message: &str) -> NotifyParam {
		NotifyParam {
			message: message.to_owned(),
			image_thumbnail: None,
			image_fullsize: None,
			image_file: None,
			sticker_package_id: None,
			notification_disabled: None,
		}
	}

	pub fn image_thumbnail(mut self, url: &str) -> NotifyParam {
		self.image_thumbnail = Some(url.to_owned());
		self
	}
	
	pub fn image_fullsize(mut self, url: &str) -> NotifyParam {
		self.image_fullsize = Some(url.to_owned());
		self
	}

	pub fn image_file(mut self, url: &str) -> NotifyParam {
		self.image_file = Some(url.to_owned());
		self
	}

	pub fn sticker_package_id(mut self, id: u32) -> NotifyParam {
		self.sticker_package_id = Some(id);
		self
	}

	pub fn notification_disabled(mut self, b: bool) -> NotifyParam {
		self.notification_disabled = Some(b);
		self
	}
}
