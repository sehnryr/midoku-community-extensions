use miniserde::Deserialize;
use speedate::DateTime;

use crate::bindings::exports::midoku::types::chapter::Chapter;
use crate::HOME_URL;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ChapterResponseSchema {
    pub data: Vec<ChapterDataSchema>,
    pub limit: isize,
    pub offset: isize,
    pub total: isize,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ChapterDataSchema {
    pub id: String,
    pub attributes: ChapterAttributesSchema,
    pub relationships: Vec<ChapterRelationshipSchema>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ChapterAttributesSchema {
    pub title: Option<String>,
    pub chapter: Option<String>,
    pub volume: Option<String>,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: String,
    #[serde(rename = "publishAt")]
    pub publish_at: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ChapterRelationshipSchema {
    pub id: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub attributes: Option<ChapterRelationshipAttributesSchema>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ChapterRelationshipAttributesSchema {
    // For scanlation groups, type is "scanlation_group"
    pub name: Option<String>,

    // For uploaders, type is "user"
    pub username: Option<String>,
}

impl TryInto<Chapter> for ChapterDataSchema {
    type Error = ();

    fn try_into(self) -> Result<Chapter, Self::Error> {
        let id = self.id;
        let url = format!("{}/chapter/{}", HOME_URL, &id);
        let title = self.attributes.title.unwrap_or_default();
        let volume = self.attributes.volume.unwrap_or_default();
        let chapter = self.attributes.chapter.unwrap_or_default();
        let date_updated = self.attributes.publish_at;
        let language = self.attributes.translated_language;

        // When the volume, chapter, and title are empty, it's a oneshot
        let title = if volume.is_empty() && chapter.is_empty() && title.is_empty() {
            "Oneshot".to_string()
        } else {
            title
        };

        // Convert the volume and chapter to f32
        // If the conversion fails (e.g. the manga is a oneshot), set the value to -1.0
        let volume: f32 = volume.parse().unwrap_or(-1.0);
        let chapter: f32 = chapter.parse().unwrap_or(-1.0);

        let date_updated = DateTime::parse_str_rfc3339(&date_updated)
            .map_err(|_| ())?
            .timestamp() as u32;

        let mut scanlation_groups = Vec::new();
        let mut uploader = String::new();

        for relationship in self.relationships {
            if let Some(relationship_attributes) = relationship.attributes {
                match relationship.relationship_type.as_str() {
                    "scanlation_group" => {
                        scanlation_groups.push(relationship_attributes.name.unwrap());
                    }
                    "user" => {
                        uploader = relationship_attributes.username.unwrap();
                    }
                    _ => {}
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_response_schema_deserialize() {
        let chapter_response_schema = r#"{
            "data": [
                {
                    "id": "id",
                    "attributes": {
                        "title": "title",
                        "chapter": "chapter",
                        "volume": "volume",
                        "translatedLanguage": "en",
                        "publishAt": "2024-01-01T00:00:00+00:00"
                    },
                    "relationships": [
                        {
                            "id": "id",
                            "type": "scanlation_group",
                            "attributes": {
                                "name": "scanlator"
                            }
                        }
                    ]
                }
            ],
            "limit": 1,
            "offset": 0,
            "total": 1
        }"#;

        let chapter_response_schema: ChapterResponseSchema =
            miniserde::json::from_str(chapter_response_schema).unwrap();

        let expected = ChapterResponseSchema {
            data: vec![ChapterDataSchema {
                id: "id".to_string(),
                attributes: ChapterAttributesSchema {
                    title: Some("title".to_string()),
                    chapter: Some("chapter".to_string()),
                    volume: Some("volume".to_string()),
                    translated_language: "en".to_string(),
                    publish_at: "2024-01-01T00:00:00+00:00".to_string(),
                },
                relationships: vec![ChapterRelationshipSchema {
                    id: "id".to_string(),
                    relationship_type: "scanlation_group".to_string(),
                    attributes: Some(ChapterRelationshipAttributesSchema {
                        name: Some("scanlator".to_string()),
                        username: None,
                    }),
                }],
            }],
            limit: 1,
            offset: 0,
            total: 1,
        };

        assert_eq!(chapter_response_schema, expected);
    }

    #[test]
    fn test_chapter_response_schema_into() {
        let chapter_response_schema = ChapterResponseSchema {
            data: vec![ChapterDataSchema {
                id: "id".to_string(),
                attributes: ChapterAttributesSchema {
                    title: Some("title".to_string()),
                    chapter: Some("chapter".to_string()),
                    volume: Some("volume".to_string()),
                    translated_language: "en".to_string(),
                    publish_at: "2024-01-01T00:00:00+00:00".to_string(),
                },
                relationships: vec![ChapterRelationshipSchema {
                    id: "id".to_string(),
                    relationship_type: "scanlation_group".to_string(),
                    attributes: Some(ChapterRelationshipAttributesSchema {
                        name: Some("scanlator".to_string()),
                        username: None,
                    }),
                }],
            }],
            limit: 1,
            offset: 0,
            total: 1,
        };

        for chapter_data_schema in chapter_response_schema.data {
            let chapter: Chapter = chapter_data_schema.try_into().unwrap();

            assert_eq!(chapter.id, "id");
            assert_eq!(chapter.title, "title");
            assert_eq!(chapter.volume, -1.0);
            assert_eq!(chapter.chapter, -1.0);
            assert_eq!(chapter.date_updated, 1704067200);
            assert_eq!(chapter.scanlator, "scanlator");
            assert_eq!(chapter.url, format!("{}/chapter/id", HOME_URL));
            assert_eq!(chapter.language, "en");
        }
    }
}
