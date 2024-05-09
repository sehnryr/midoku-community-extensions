use miniserde::json as miniserde_json;

use crate::bindings::exports::midoku::types::manga::{ContentRating, Manga, ReadingMode, Status};
use crate::utils::miniserde_trait::{BorrowType, GetType};
use crate::{HOME_URL, LOCALE};

use super::parse_object_attribute;

pub trait ParseManga {
    fn parse_manga(&self) -> Result<Manga, ()>;
    fn parse_partial_manga(&self) -> Result<Manga, ()>;
}

impl ParseManga for miniserde_json::Object {
    fn parse_manga(&self) -> Result<Manga, ()> {
        parse_manga(self)
    }

    fn parse_partial_manga(&self) -> Result<Manga, ()> {
        parse_partial_manga(self)
    }
}

fn parse_manga_status(status: &str) -> Status {
    match status {
        "ongoing" => Status::Ongoing,
        "completed" => Status::Completed,
        "hiatus" => Status::Hiatus,
        "cancelled" => Status::Cancelled,
        _ => Default::default(),
    }
}

fn parse_manga_content_rating(content_rating: &str) -> ContentRating {
    match content_rating {
        "safe" => ContentRating::Safe,
        "suggestive" => ContentRating::Suggestive,
        "erotica" => ContentRating::Nsfw,
        "pornographic" => ContentRating::Nsfw,
        _ => Default::default(),
    }
}

fn parse_manga_reading_mode(reading_mode: &str) -> ReadingMode {
    match reading_mode {
        "ja" => ReadingMode::RightToLeft,
        "zh" => ReadingMode::Scroll,
        "ko" => ReadingMode::Scroll,
        _ => Default::default(),
    }
}

fn parse_manga(manga_data: &miniserde_json::Object) -> Result<Manga, ()> {
    let locale: &str = &LOCALE;

    let attributes = manga_data.get_object("attributes")?;

    let id = manga_data.get_string("id")?.clone();
    let title = parse_object_attribute(attributes, "title")?;
    let url = format!("{}/title/{}", HOME_URL, id);
    let description = parse_object_attribute(attributes, "description")?;

    let mut cover_file = String::new();
    let mut author_name = String::new();
    let mut artist_name = String::new();

    if let Ok(relationships) = manga_data.get_array("relationships") {
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
                "cover_art" => {
                    cover_file = relationship_attributes.get_string("fileName")?.clone();
                }
                "author" => {
                    author_name = relationship_attributes.get_string("name")?.clone();
                }
                "artist" => {
                    artist_name = relationship_attributes.get_string("name")?.clone();
                }
                _ => continue,
            }
        }
    }

    let cover_url = if cover_file.is_empty() {
        Default::default()
    } else {
        format!("{}/covers/{}/{}", HOME_URL, id, cover_file)
    };

    let mut categories: Vec<String> = Vec::new();

    if let Ok(tags) = attributes.get_array("tags") {
        for tag in tags {
            let tag = tag.borrow_object()?;
            let tag_attributes = tag.get_object("attributes")?;
            let tag_name_object = tag_attributes.get_object("name")?;
            let tag_name = if tag_name_object.contains_key(locale) {
                tag_name_object.get_string(locale)?
            } else {
                let (_, tag_name_value) = tag_name_object.first_key_value().ok_or(())?;
                tag_name_value.borrow_string()?
            };
            categories.push(tag_name.clone());
        }
    }

    let status = parse_manga_status(
        attributes
            .get_string("status")
            .unwrap_or(&String::new())
            .as_str(),
    );

    let content_rating = parse_manga_content_rating(
        attributes
            .get_string("contentRating")
            .unwrap_or(&String::new())
            .as_str(),
    );

    let reading_mode = parse_manga_reading_mode(
        attributes
            .get_string("originalLanguage")
            .unwrap_or(&String::new())
            .as_str(),
    );

    Ok(Manga {
        id,
        title,
        url,
        description,
        cover_url,
        author_name,
        artist_name,
        categories,
        status,
        content_rating,
        reading_mode,
    })
}

fn parse_partial_manga(manga_data: &miniserde_json::Object) -> Result<Manga, ()> {
    let attributes = manga_data.get_object("attributes")?;

    let id = manga_data.get_string("id")?.clone();
    let title = parse_object_attribute(attributes, "title")?;

    let mut cover_file = String::new();

    if let Ok(relationships) = manga_data.get_array("relationships") {
        for relationship in relationships {
            let relationship = relationship.borrow_object()?;
            let relationship_type = relationship.get_string("type");
            let relationship_attributes = relationship.get_object("attributes");

            if relationship_type.is_err() || relationship_attributes.is_err() {
                continue;
            }

            let relationship_type = relationship_type.unwrap();
            let relationship_attributes = relationship_attributes.unwrap();

            if relationship_type.as_str() == "cover_art" {
                cover_file = relationship_attributes.get_string("fileName")?.clone();
                break;
            }
        }
    }

    let cover_url = if cover_file.is_empty() {
        Default::default()
    } else {
        format!("{}/covers/{}/{}", HOME_URL, id, cover_file)
    };

    Ok(Manga {
        id,
        title,
        cover_url,
        ..Default::default()
    })
}
