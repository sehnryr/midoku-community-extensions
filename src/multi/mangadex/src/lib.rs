mod utils;

use miniserde::json as miniserde_json;
use once_cell::sync::Lazy;

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
use crate::utils::parse::{parse_manga, parse_partial_manga};
use crate::utils::url_encode::url_encode;

const API_URL: &str = "https://api.mangadex.org";
const HOME_URL: &str = "https://mangadex.org";

static LOCALE: Lazy<&str> = Lazy::new(|| {
    // TODO: Get the locale from the host

    "en"
});

struct Component;

impl Guest for Component {
    fn initialize() -> Result<(), ()> {
        // Set the rate limiter to 3 requests per second
        set_burst(3)?;
        set_period_ms(1000)?;

        // First access the lazy static variable to initialize it
        let _locale: &str = &LOCALE;

        Ok(())
    }

    fn get_manga_list(filters: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool), ()> {
        // Block until the rate limiter allows the request
        block();

        let limit = 20;
        let offset = page * limit;

        let mut url = format!(
            "{}/manga/?includes[]=cover_art&limit={}&offset={}",
            API_URL, limit, offset
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
            manga_list.push(parse_partial_manga(manga_data)?);
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
        // Block until the rate limiter allows the request
        block();

        let url = format!(
            "{}/manga/{}?includes[]=cover_art&includes[]=author&includes[]=artist",
            API_URL, manga_id,
        );

        let headers = vec![("User-Agent".to_string(), "Midoku".to_string())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let json = miniserde_json::from_str::<miniserde_json::Value>(&content)
            .map_err(|_| ())?
            .take_object()?;

        // Get the data field from the JSON response
        let manga_data = json.get_object("data")?;

        Ok(parse_manga(manga_data)?)
    }

    fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>, ()> {
        todo!("Get chapter list not implemented")
    }

    fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        todo!("Get page list not implemented")
    }
}

bindings::export!(Component with_types_in bindings);

// Implement the default trait for the types

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
