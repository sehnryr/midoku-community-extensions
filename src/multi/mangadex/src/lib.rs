mod utils;

use miniserde::json as miniserde_json;

#[allow(warnings)]
mod bindings;

use bindings::exports::midoku::bindings::api::Guest;
use bindings::exports::midoku::types::chapter::Chapter;
use bindings::exports::midoku::types::filter::Filter;
use bindings::exports::midoku::types::manga::{ContentRating, Manga, ReadingMode, Status};
use bindings::exports::midoku::types::page::Page;
use bindings::midoku::http::outgoing_handler::{handle, Method};
use bindings::midoku::limiter::rate_limiter::{block, set_burst, set_period_ms};

use crate::utils::miniserde_trait::{BorrowType, GetType, TakeType};
use crate::utils::url_encode::url_encode;

const URL: &str = "https://api.mangadex.org";

struct Component;

impl Guest for Component {
    fn initialize() -> Result<(), ()> {
        // Set the rate limiter to 3 requests per second
        set_burst(3)?;
        set_period_ms(1000)?;

        Ok(())
    }

    fn get_manga_list(filters: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool), ()> {
        // Block until the rate limiter allows the request
        block();

        let limit = 20;
        let offset = page * limit;

        let mut url = format!(
            "{}/manga/?includes[]=cover_art&limit={}&offset={}",
            URL, limit, offset
        );

        for filter in filters {
            match filter {
                Filter::Title(title) => {
                    url.push_str(&format!("&title={}", url_encode(&title.query)));
                }
                Filter::Sort(sort) => {
                    let option = match sort.option_index {
                        0 => "latestUploadedChapter",
                        1 => "relevance",
                        2 => "followedCount",
                        3 => "createdAt",
                        4 => "updatedAt",
                        5 => "title",
                        _ => return Err(()),
                    };
                    let order = match sort.option_reversed {
                        true => "asc",
                        false => "desc",
                    };
                    url.push_str(&format!("&order[{}]={}", option, order));
                }
            }
        }

        let headers = vec![("User-Agent".to_string(), "Midoku".to_string())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let json = miniserde_json::from_str::<miniserde_json::Value>(&content)
            .map_err(|_| ())?
            .take_object()?;

        // Get the data field from the JSON response
        let data = json.get_array("data")?;

        // Parse the manga data
        let mut manga_list = Vec::new();
        for manga_data in data {
            let manga_data = manga_data.borrow_object()?;
            let attributes = manga_data.get_object("attributes")?;

            let id = manga_data.get_string("id")?;

            let title = {
                let title_object = attributes.get_object("title")?;
                if title_object.contains_key("en") {
                    title_object.get_string("en")?
                } else {
                    let (_, title_value) = title_object.first_key_value().ok_or(())?;
                    title_value.borrow_string()?
                }
            };

            let cover_url = if let Ok(relationships) = manga_data.get_array("relationships") {
                let mut cover_art_file_name = &String::new();
                for relationship in relationships {
                    let relationship = relationship.borrow_object()?;
                    let relationship_type = relationship.get_string("type");

                    if relationship_type.is_err() {
                        continue;
                    }

                    if relationship_type.unwrap() != "cover_art" {
                        continue;
                    }

                    let relationship_attributes = relationship.get_object("attributes")?;
                    cover_art_file_name = relationship_attributes.get_string("fileName")?;
                    break;
                }

                if cover_art_file_name.is_empty() {
                    Default::default()
                } else {
                    format!(
                        "https://uploads.mangadex.org/cover/{}/{}",
                        id, cover_art_file_name
                    )
                }
            } else {
                Default::default()
            };

            manga_list.push(Manga {
                id: id.clone(),
                title: title.clone(),
                url: Default::default(),
                description: Default::default(),
                cover_url,
                author_name: Default::default(),
                artist_name: Default::default(),
                categories: Default::default(),
                status: Status::Ongoing,
                content_rating: ContentRating::Safe,
                reading_mode: ReadingMode::RightToLeft,
            });
        }

        // Get the total number of manga
        let total = match json.get_number("total")? {
            miniserde_json::Number::U64(n) => n.clone() as u32,
            _ => return Err(()),
        };

        let has_next = (offset + limit) < total;

        Ok((manga_list, has_next))
    }

    fn get_manga_details(manga_id: String) -> Result<Manga, ()> {
        todo!("Get manga details not implemented")
    }

    fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>, ()> {
        todo!("Get chapter list not implemented")
    }

    fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        todo!("Get page list not implemented")
    }
}

bindings::export!(Component with_types_in bindings);
