use miniserde::json as miniserde_json;
use speedate::DateTime;

use crate::bindings::exports::midoku::types::chapter::Chapter;
use crate::utils::miniserde_trait::{BorrowType, GetType};
use crate::{HOME_URL, LOCALE};

use super::parse_object_attribute;

pub trait ParseChapter {
    fn parse_chapter(&self) -> Result<Chapter, ()>;
}

impl ParseChapter for miniserde_json::Object {
    fn parse_chapter(&self) -> Result<Chapter, ()> {
        parse_chapter(self)
    }
}

fn parse_chapter(chapter_data: &miniserde_json::Object) -> Result<Chapter, ()> {
    let locale: &str = &LOCALE;

    let attributes = chapter_data.get_object("attributes")?;

    let id = chapter_data.get_string("id")?.clone();
    let mut title = parse_object_attribute(attributes, "title").unwrap_or_default();
    let volume = attributes
        .get_string("volume")
        .map(|volume| volume.clone())
        .unwrap_or_default();
    let chapter = attributes
        .get_string("chapter")
        .map(|chapter| chapter.clone())
        .unwrap_or_default();
    let date_updated = attributes.get_string("publishAt")?.clone();
    let url = format!("{}/chapter/{}", HOME_URL, id);
    let language = attributes
        .get_string("translatedLanguage")
        .map(|language| language.clone())
        .unwrap_or(locale.to_string());

    // When the volume, chapter, and title are empty, it's a oneshot
    if volume.is_empty() && chapter.is_empty() && title.is_empty() {
        title = "Oneshot".to_string();
    }

    // Convert the volume and chapter to f32
    // If the conversion fails (e.g. the manga is a oneshot), set the value to -1.0
    let volume: f32 = volume.parse().unwrap_or(-1.0);
    let chapter: f32 = chapter.parse().unwrap_or(-1.0);

    let date_updated = DateTime::parse_str_rfc3339(&date_updated)
        .map_err(|_| ())?
        .timestamp() as u32;

    let mut scanlation_groups = Vec::new();
    let mut uploader = String::new();

    if let Ok(relationships) = chapter_data.get_array("relationships") {
        for relationship in relationships {
            let relationship = relationship.borrow_object()?;
            let relationship_type = relationship.get_string("type");
            let relationship_attributes = relationship.get_object("attributes");

            if relationship_type.is_err() || relationship_attributes.is_err() {
                continue;
            }

            let relationship_type = relationship_type.unwrap();
            let relationship_attributes = relationship_attributes.unwrap();

            match relationship_type.as_str() {
                "scanlation_group" => {
                    scanlation_groups.push(relationship_attributes.get_string("name")?.clone())
                }
                "user" => uploader = relationship_attributes.get_string("username")?.clone(),
                _ => continue,
            }
        }
    }

    let scanlator = if scanlation_groups.is_empty() {
        uploader
    } else {
        scanlation_groups.join(", ")
    };

    Ok(Chapter {
        id,
        title,
        volume,
        chapter,
        date_updated,
        scanlator,
        url,
        language,
    })
}
