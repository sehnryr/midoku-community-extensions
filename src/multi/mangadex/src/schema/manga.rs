use miniserde::{json, Deserialize};

use crate::bindings::exports::midoku::types::manga::{ContentRating, Manga, ReadingMode, Status};
use crate::{HOME_URL, LOCALE};

#[derive(Debug, Deserialize)]
pub struct MangaResponseSchema {
    pub data: Vec<MangaDataSchema>,
    pub limit: isize,
    pub offset: isize,
    pub total: isize,
}

#[derive(Debug, Deserialize)]
pub struct MangaResponseSingleSchema {
    pub data: MangaDataSchema,
}

#[derive(Debug, Deserialize)]
pub struct MangaDataSchema {
    pub id: String,
    pub attributes: MangaAttributesSchema,
    pub relationships: Vec<MangaRelationshipSchema>,
}

#[derive(Debug, Deserialize)]
pub struct MangaAttributesSchema {
    pub title: json::Object,
    pub description: json::Object,
    #[serde(rename = "originalLanguage")]
    pub original_language: String,
    pub status: String,
    #[serde(rename = "contentRating")]
    pub content_rating: String,
    pub tags: Vec<MangaTagSchema>,
}

#[derive(Debug, Deserialize)]
pub struct MangaTagSchema {
    pub id: String,
    pub attributes: MangaTagAttributesSchema,
}

#[derive(Debug, Deserialize)]
pub struct MangaTagAttributesSchema {
    pub name: json::Object,
}

#[derive(Debug, Deserialize)]
pub struct MangaRelationshipSchema {
    pub id: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub attributes: Option<MangaRelationshipAttributesSchema>,
}

#[derive(Debug, Deserialize)]
pub struct MangaRelationshipAttributesSchema {
    // For cover art, type is "cover_art"
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,

    // For authors and artists, type is "author" or "artist"
    pub name: Option<String>,
}

#[doc(hidden)]
macro_rules! object_get_string {
    ($attributes:expr, $key:expr) => {
        match $attributes.get($key).unwrap_or_else(|| {
            if !$attributes.is_empty() {
                $attributes.iter().next().unwrap().1
            } else {
                &json::Value::Null
            }
        }) {
            json::Value::String(value) => Ok(value.clone()),
            _ => Err(()),
        }
    };
}

impl TryInto<Manga> for MangaDataSchema {
    type Error = ();

    fn try_into(self) -> Result<Manga, Self::Error> {
        let locale: &str = LOCALE;

        let id = self.id;
        let url = format!("{}/title/{}", HOME_URL, &id);

        let title = object_get_string!(self.attributes.title, locale)?;
        let description =
            object_get_string!(self.attributes.description, locale).unwrap_or(String::new());

        let mut cover_file = String::new();
        let mut author_name = String::new();
        let mut artist_name = String::new();

        for relationship in self.relationships {
            let relationship_attributes = relationship.attributes;

            if relationship_attributes.is_none() {
                continue;
            }

            let relationship_attributes = relationship_attributes.unwrap();

            match relationship.relationship_type.as_str() {
                "cover_art" => {
                    cover_file = relationship_attributes.file_name.unwrap();
                }
                "author" => {
                    author_name = relationship_attributes.name.unwrap();
                }
                "artist" => {
                    artist_name = relationship_attributes.name.unwrap();
                }
                _ => continue,
            }
        }

        let cover_url = if cover_file.is_empty() {
            Default::default()
        } else {
            format!("{}/covers/{}/{}", HOME_URL, id, cover_file)
        };

        let status = match self.attributes.status.as_str() {
            "ongoing" => Status::Ongoing,
            "completed" => Status::Completed,
            "hiatus" => Status::Hiatus,
            "cancelled" => Status::Cancelled,
            _ => Default::default(),
        };

        let content_rating = match self.attributes.content_rating.as_str() {
            "safe" => ContentRating::Safe,
            "suggestive" => ContentRating::Suggestive,
            "erotica" => ContentRating::Nsfw,
            "pornographic" => ContentRating::Nsfw,
            _ => Default::default(),
        };

        let reading_mode = match self.attributes.original_language.as_str() {
            "ja" => ReadingMode::RightToLeft,
            "zh" => ReadingMode::Scroll,
            "ko" => ReadingMode::Scroll,
            _ => Default::default(),
        };

        let mut categories = Vec::with_capacity(self.attributes.tags.len());
        for tag in self.attributes.tags {
            let name = object_get_string!(tag.attributes.name, locale)?;
            categories.push(name);
        }

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
}

// Implement the default trait for the manga types

impl Default for Status {
    fn default() -> Self {
        Status::Unknown
    }
}

impl Default for ContentRating {
    fn default() -> Self {
        ContentRating::Safe
    }
}

impl Default for ReadingMode {
    fn default() -> Self {
        ReadingMode::RightToLeft
    }
}

impl Default for Manga {
    fn default() -> Self {
        Manga {
            id: Default::default(),
            title: Default::default(),
            cover_url: Default::default(),
            url: Default::default(),
            description: Default::default(),
            author_name: Default::default(),
            artist_name: Default::default(),
            categories: Default::default(),
            status: Default::default(),
            content_rating: Default::default(),
            reading_mode: Default::default(),
        }
    }
}
