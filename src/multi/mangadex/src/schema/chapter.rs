use miniserde::Deserialize;
use speedate::DateTime;

use crate::bindings::exports::midoku::types::chapter::Chapter;
use crate::HOME_URL;

#[derive(Debug, Deserialize)]
pub struct ChapterResponseSchema {
    pub data: Vec<ChapterDataSchema>,
    pub limit: isize,
    pub offset: isize,
    pub total: isize,
}

#[derive(Debug, Deserialize)]
pub struct ChapterDataSchema {
    pub id: String,
    pub attributes: ChapterAttributesSchema,
    pub relationships: Vec<ChapterRelationshipSchema>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterAttributesSchema {
    pub title: Option<String>,
    pub chapter: Option<String>,
    pub volume: Option<String>,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: String,
    #[serde(rename = "publishAt")]
    pub publish_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ChapterRelationshipSchema {
    pub id: String,
    #[serde(rename = "type")]
    pub relationship_type: String,
    pub attributes: Option<ChapterRelationshipAttributesSchema>,
}

#[derive(Debug, Deserialize)]
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
