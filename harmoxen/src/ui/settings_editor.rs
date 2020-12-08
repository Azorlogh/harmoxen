use crate::state::{settings_editor::State, Message};
use iced::{Column, Element};

pub fn build(_state: &mut State) -> Element<Message> {
	Column::new().into()
}
