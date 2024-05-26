use miniserde::Deserialize;

use crate::bindings::exports::midoku::types::page::Page;
use crate::host_settings::HostSettings;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PageResponseSchema {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: PageChapterSchema,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PageChapterSchema {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}

impl Into<Vec<Page>> for PageResponseSchema {
    fn into(self) -> Vec<Page> {
        let data_saver = HostSettings::get_data_saver();

        let base_url = format!(
            "{}/{}/{}",
            self.base_url,
            if data_saver { "data-saver" } else { "data" },
            self.chapter.hash
        );

        let data = if data_saver {
            self.chapter.data_saver
        } else {
            self.chapter.data
        };

        let mut page_list = Vec::with_capacity(data.len());
        for (index, file_name) in data.into_iter().enumerate() {
            page_list.push(Page {
                index: index as u32,
                url: format!("{}/{}", base_url, file_name),
                base64: Default::default(),
            });
        }

        page_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_response_schema_deserialize() {
        let page_response_schema = r#"{
            "baseUrl": "https://api.mangadex.org",
            "chapter": {
                "hash": "hash",
                "data": ["1.jpg", "2.jpg"],
                "dataSaver": ["1s.jpg", "2s.jpg"]
            }
        }"#;

        let page_response_schema: PageResponseSchema =
            miniserde::json::from_str(page_response_schema).unwrap();

        let expected = PageResponseSchema {
            base_url: "https://api.mangadex.org".to_string(),
            chapter: PageChapterSchema {
                hash: "hash".to_string(),
                data: vec!["1.jpg".to_string(), "2.jpg".to_string()],
                data_saver: vec!["1s.jpg".to_string(), "2s.jpg".to_string()],
            },
        };

        assert_eq!(page_response_schema, expected);
    }

    #[test]
    fn test_page_response_schema_into() {
        let page_response_schema = PageResponseSchema {
            base_url: "https://api.mangadex.org".to_string(),
            chapter: PageChapterSchema {
                hash: "hash".to_string(),
                data: vec!["1.jpg".to_string(), "2.jpg".to_string()],
                data_saver: vec!["1s.jpg".to_string(), "2s.jpg".to_string()],
            },
        };

        let page_list: Vec<Page> = page_response_schema.into();

        assert_eq!(page_list.len(), 2);
        assert_eq!(page_list[0].index, 0);
        assert_eq!(page_list[0].url, "https://api.mangadex.org/data/hash/1.jpg");
        assert_eq!(page_list[1].index, 1);
        assert_eq!(page_list[1].url, "https://api.mangadex.org/data/hash/2.jpg");
    }
}
