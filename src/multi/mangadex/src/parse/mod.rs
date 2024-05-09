pub mod parse_chapter;
pub mod parse_manga;

use miniserde::json as miniserde_json;

use crate::utils::miniserde_trait::{BorrowType, GetType};
use crate::LOCALE;

fn parse_object_attribute(
    object_attributes: &miniserde_json::Object,
    key: &str,
) -> Result<String, ()> {
    let locale: &str = &LOCALE;

    let attributes_object = object_attributes.get_object(key)?;
    let attribute = if attributes_object.contains_key(locale) {
        attributes_object.get_string(locale)?
    } else if !attributes_object.is_empty() {
        let (_, attribute_value) = attributes_object.first_key_value().ok_or(())?;
        attribute_value.borrow_string()?
    } else {
        return Ok(String::new());
    };
    Ok(attribute.to_string())
}
