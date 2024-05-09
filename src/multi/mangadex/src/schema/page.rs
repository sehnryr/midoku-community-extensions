use miniserde::Deserialize;

use crate::bindings::exports::midoku::types::page::Page;
use crate::DATA_SAVER;

#[derive(Debug, Deserialize)]
pub struct PageResponseSchema {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: PageChapterSchema,
}

#[derive(Debug, Deserialize)]
pub struct PageChapterSchema {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}

impl Into<Vec<Page>> for PageResponseSchema {
    fn into(self) -> Vec<Page> {
        let base_url = format!(
            "{}/{}/{}",
            self.base_url,
            if DATA_SAVER { "data-saver" } else { "data" },
            self.chapter.hash
        );

        let data = if DATA_SAVER {
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
