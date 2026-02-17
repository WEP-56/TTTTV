use crate::services::{SearchService, PlayParser, SourceCrawler, DefaultSourceCrawler};
use crate::models::Site;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub search_service: Arc<SearchService>,
    pub play_parser: Arc<PlayParser>,
}

impl AppState {
    pub async fn new(sites: Vec<Site>) -> Self {
        let crawler: Arc<dyn SourceCrawler> = Arc::new(DefaultSourceCrawler::new(
            std::time::Duration::from_secs(10),
        ));

        let search_service = SearchService::new(
            crawler,
            sites,
            Vec::new(),
            Vec::new(),
        );

        let play_parser = PlayParser::new();

        AppState {
            search_service: Arc::new(search_service),
            play_parser: Arc::new(play_parser),
        }
    }
}
