mod host_settings;
mod schema;
mod utils;

use miniserde::json as miniserde_json;

#[allow(warnings)]
mod bindings;

use bindings::exports::midoku::bindings::api::Guest;
use bindings::exports::midoku::types::chapter::Chapter;
use bindings::exports::midoku::types::filter::Filter;
use bindings::exports::midoku::types::manga::Manga;
use bindings::exports::midoku::types::page::Page;
use bindings::midoku::http::outgoing_handler::{handle, Method};
use bindings::midoku::limiter::rate_limiter::{block, set_burst, set_period_ms};

use crate::host_settings::HostSettings;
use crate::schema::chapter::ChapterResponseSchema;
use crate::schema::manga::{MangaResponseSchema, MangaResponseSingleSchema};
use crate::schema::page::PageResponseSchema;
use crate::utils::url_encode::url_encode;

const API_URL: &str = "https://api.mangadex.org";
const HOME_URL: &str = "https://mangadex.org";

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
        let offset = page as isize * limit;

        let mut url = format!(
            "{}/manga/\
                ?includes[]=cover_art\
                &includes[]=author\
                &includes[]=artist\
                &limit={}\
                &offset={}",
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

        let headers = vec![("User-Agent".to_string(), HostSettings::get_user_agent())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let manga_response: MangaResponseSchema =
            miniserde_json::from_str(&content).map_err(|_| ())?;

        // Parse the manga data
        let mut manga_list = Vec::new();
        for manga_data in manga_response.data {
            manga_list.push(manga_data.try_into()?);
        }

        let has_next = (offset + limit) < manga_response.total;

        Ok((manga_list, has_next))
    }

    fn get_manga_details(manga_id: String) -> Result<Manga, ()> {
        // Block until the rate limiter allows the request
        block();

        let url = format!(
            "{}/manga/{}\
                ?includes[]=cover_art\
                &includes[]=author\
                &includes[]=artist",
            API_URL, manga_id,
        );

        let headers = vec![("User-Agent".to_string(), HostSettings::get_user_agent())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let manga_response: MangaResponseSingleSchema =
            miniserde_json::from_str(&content).map_err(|_| ())?;

        Ok(manga_response.data.try_into()?)
    }

    fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>, ()> {
        // Block until the rate limiter allows the request
        block();

        let limit = 500;

        let mut url = format!(
            "{}/manga/{}/feed\
                ?limit={}\
                &order[volume]=asc\
                &order[chapter]=asc\
                &contentRating[]=safe\
                &contentRating[]=suggestive\
                &contentRating[]=erotica\
                &contentRating[]=pornographic\
                &includes[]=user\
                &includes[]=scanlation_group",
            API_URL, manga_id, limit
        );

        for language in HostSettings::get_languages() {
            url.push_str(&format!("&translatedLanguage[]={}", language));
        }

        // TODO: Add the ability to filter out blocked groups and uploaders

        let headers = vec![("User-Agent".to_string(), HostSettings::get_user_agent())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let chapter_response: ChapterResponseSchema =
            miniserde_json::from_str(&content).map_err(|_| ())?;

        let mut chapter_list = Vec::with_capacity(chapter_response.total as usize);
        for chapter_data in chapter_response.data {
            chapter_list.push(chapter_data.try_into()?);
        }

        let mut offset = limit;
        while offset < chapter_response.total {
            offset += limit;

            let response = handle(
                Method::Get,
                &format!("{}&offset={}", url, offset),
                Some(&headers),
                None,
            )?;

            let bytes = response.bytes();
            let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

            // Parse the JSON response
            let chapter_response: ChapterResponseSchema =
                miniserde_json::from_str(&content).map_err(|_| ())?;

            for chapter_data in chapter_response.data {
                chapter_list.push(chapter_data.try_into()?);
            }
        }

        Ok(chapter_list)
    }

    fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        // Block until the rate limiter allows the request
        block();

        let url = format!(
            "{}/at-home/server/{}\
                ?forcePort443={}",
            API_URL,
            chapter_id,
            HostSettings::get_force_port_443()
        );

        let headers = vec![("User-Agent".to_string(), HostSettings::get_user_agent())];
        let response = handle(Method::Get, &url, Some(&headers), None)?;

        let bytes = response.bytes();
        let content = std::str::from_utf8(&bytes).map_err(|_| ())?;

        // Parse the JSON response
        let page_response: PageResponseSchema =
            miniserde_json::from_str(&content).map_err(|_| ())?;

        Ok(page_response.into())
    }
}

bindings::export!(Component with_types_in bindings);
