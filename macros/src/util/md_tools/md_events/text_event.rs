//! Provider of [`TextEvent`].

use pulldown_cmark::{CowStr, Event};

/// Markdown event with text.
pub(crate) struct TextEvent<'a> {
    pub text: CowStr<'a>,
    pub is_code: bool,
}

impl<'a> TextEvent<'a> {
    /// Creates a text event if it exists.
    pub fn try_new(event: &Event<'a>) -> Option<Self> {
        match event {
            Event::Text(cow_str) => Some(Self {
                text: cow_str.clone(),
                is_code: false,
            }),
            Event::Code(cow_str) => Some(Self {
                text: cow_str.clone(),
                is_code: true,
            }),
            _ => None,
        }
    }

    /// Creates a instance with given text.
    pub fn with_text(self, value: &str) -> Self {
        Self {
            text: CowStr::Boxed(value.to_string().into_boxed_str()),
            ..self
        }
    }

    /// Retruns event.
    pub fn to_event(&self) -> Event<'a> {
        match self.is_code {
            false => Event::Text(self.text.clone()),
            true => Event::Code(self.text.clone()),
        }
    }
}
