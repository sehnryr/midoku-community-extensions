use miniserde::{json, Deserialize};

use crate::bindings::exports::midoku::types::manga::{ContentRating, Manga, ReadingMode, Status};
use crate::host_settings::HostSettings;
use crate::HOME_URL;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MangaResponseSchema {
    pub data: Vec<MangaDataSchema>,
    pub limit: isize,
    pub offset: isize,
    pub total: isize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MangaResponseSingleSchema {
    pub data: MangaDataSchema,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

impl PartialEq for MangaAttributesSchema {
    fn eq(&self, other: &Self) -> bool {
        if self.title.len() != other.title.len()
            || self.description.len() != other.description.len()
        {
            return false;
        }
        if self.title.keys().ne(other.title.keys())
            || self.description.keys().ne(other.description.keys())
        {
            return false;
        }
        for (key, value) in &self.title {
            let other_value = other.title.get(key).unwrap();
            match (value, other_value) {
                (json::Value::String(value), json::Value::String(other_value)) => {
                    if value != other_value {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        for (key, value) in &self.description {
            let other_value = other.description.get(key).unwrap();
            match (value, other_value) {
                (json::Value::String(value), json::Value::String(other_value)) => {
                    if value != other_value {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        self.original_language == other.original_language
            && self.status == other.status
            && self.content_rating == other.content_rating
            && self.tags == other.tags
    }
}

impl Eq for MangaAttributesSchema {}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MangaTagSchema {
    pub id: String,
    pub attributes: MangaTagAttributesSchema,
}

#[derive(Debug, Deserialize)]
pub struct MangaTagAttributesSchema {
    pub name: json::Object,
}

impl PartialEq for MangaTagAttributesSchema {
    fn eq(&self, other: &Self) -> bool {
        if self.name.len() != other.name.len() {
            return false;
        }
        if self.name.keys().ne(other.name.keys()) {
            return false;
        }
        for (key, value) in &self.name {
            let other_value = other.name.get(key).unwrap();
            match (value, other_value) {
                (json::Value::String(value), json::Value::String(other_value)) => {
                    if value != other_value {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

impl Eq for MangaTagAttributesSchema {}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MangaRelationshipSchema {
    pub id: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub attributes: Option<MangaRelationshipAttributesSchema>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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
        let locale = HostSettings::get_locale();

        let id = self.id;
        let url = format!("{}/title/{}", HOME_URL, &id);

        let title = object_get_string!(self.attributes.title, &locale)?;
        let description =
            object_get_string!(self.attributes.description, &locale).unwrap_or(String::new());

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

        let cover_quality = match HostSettings::get_cover_quality() {
            0 => "",
            1 => ".512.jpg",
            2 => ".256.jpg",
            _ => "",
        };

        let cover_url = if cover_file.is_empty() {
            Default::default()
        } else {
            format!("{}/covers/{}/{}{}", HOME_URL, id, cover_file, cover_quality)
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
            let name = object_get_string!(tag.attributes.name, &locale)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manga_response_schema_deserialize() {
        let manga_response_schema = r#"{
            "data": [
                {
                    "id": "id",
                    "attributes": {
                        "title": {
                            "en": "title"
                        },
                        "description": {
                            "en": "description"
                        },
                        "originalLanguage": "ja",
                        "status": "ongoing",
                        "contentRating": "safe",
                        "tags": []
                    },
                    "relationships": []
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        }"#;

        let manga_response_schema: MangaResponseSchema =
            miniserde::json::from_str(manga_response_schema).unwrap();

        let expected = MangaResponseSchema {
            data: vec![MangaDataSchema {
                id: "id".to_string(),
                attributes: MangaAttributesSchema {
                    title: miniserde::json::from_str(r#"{ "en": "title" }"#).unwrap(),
                    description: miniserde::json::from_str(r#"{ "en": "description" }"#).unwrap(),
                    original_language: "ja".to_string(),
                    status: "ongoing".to_string(),
                    content_rating: "safe".to_string(),
                    tags: vec![],
                },
                relationships: vec![],
            }],
            limit: 1,
            offset: 0,
            total: 1,
        };

        assert_eq!(manga_response_schema, expected);
    }

    #[test]
    fn test_manga_data_schema_try_into() {
        let manga_data_schema = MangaResponseSchema {
            data: vec![MangaDataSchema {
                id: "id".to_string(),
                attributes: MangaAttributesSchema {
                    title: miniserde::json::from_str(r#"{ "en": "title" }"#).unwrap(),
                    description: miniserde::json::from_str(r#"{ "en": "description" }"#).unwrap(),
                    original_language: "ja".to_string(),
                    status: "ongoing".to_string(),
                    content_rating: "safe".to_string(),
                    tags: vec![],
                },
                relationships: vec![],
            }],
            limit: 1,
            offset: 0,
            total: 1,
        };

        for manga in manga_data_schema.data {
            let manga: Manga = manga.try_into().unwrap();

            assert_eq!(manga.id, "id");
            assert_eq!(manga.title, "title");
            assert_eq!(manga.url, format!("{}/title/id", HOME_URL));
            assert_eq!(manga.description, "description");
            assert_eq!(manga.cover_url, "");
            assert_eq!(manga.author_name, "");
            assert_eq!(manga.artist_name, "");
            assert_eq!(manga.categories.len(), 0);
            assert_eq!(manga.status, Status::Ongoing);
            assert_eq!(manga.content_rating, ContentRating::Safe);
            assert_eq!(manga.reading_mode, ReadingMode::RightToLeft);
        }
    }
}
