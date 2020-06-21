use crate::widget::common::{ParseLazy, TextBox};
use druid::{
	widget::{Flex, Label},
	Data, Lens, Widget, WidgetExt,
};

macro_rules! enum_lens (
	($variant:path,$field:ident) => {
		Map::new(
			|data| match data {
				$variant{ $field, ..} => $field.clone(),
				_ => unreachable!(),
			},
			|data, new_data|
			if let $variant{ $field, .. } = data {
					*$field = new_data
			},
		)
	};
);

pub fn make_field<S: Data, F: Data + std::str::FromStr + std::fmt::Display>(
	name: &str,
	lens: impl Lens<S, F>,
) -> impl Widget<S> {
	Flex::row()
		.with_child(Label::new(format!("{}: ", name)))
		.with_child(ParseLazy::new(TextBox::new()))
		.padding(10.0)
		.lens(lens)
}
