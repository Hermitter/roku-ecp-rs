use crate::Device;
use crate::Error;
use url::form_urlencoded;

impl Device {
    /// Sends a URL query string from the search properties given.
    pub async fn search<'a, 'b>(&self, search: SearchRequest<'a, 'b>) -> Result<(), Error> {
        let mut url = self.url.join("search/browse")?;
        let mut query = form_urlencoded::Serializer::new(String::new());

        query.append_pair("keyword", search.keyword);
        if search.is_title {
            query.append_pair("title", "true");
        }
        if search.show_unavailable {
            query.append_pair("show-unavailable", "true");
        }
        if search.match_any {
            query.append_pair("match-any", "true");
        }
        if search.launch {
            query.append_pair("launch", "true");
        }
        if let Some(id) = search.tmsid {
            query.append_pair("tmsid", id);
        }
        if let Some(t) = search.search_type {
            query.append_pair(
                "type",
                match t {
                    SearchType::TvShow => "tv-show",
                    SearchType::Channel => "channel",
                    SearchType::Game => "game",
                    SearchType::Movie => "movie",
                    SearchType::Person => "person",
                },
            );
        }
        if let Some(num) = search.season {
            query.append_pair("season", &num.to_string());
        }
        if let Some(ids) = search.provider_ids {
            query.append_pair("provider-id", &ids.join(","));
        }
        if let Some(titles) = search.providers {
            query.append_pair("provider", &titles.join(","));
        }

        url.set_query(Some(&query.finish()));

        self.http.post(url).await?;
        Ok(())
    }
}

/// Constraints for the type of desired search result.
#[derive(Debug)]
pub enum SearchType {
    Movie,
    TvShow,
    Person,
    Channel,
    Game,
}

/// Describes a search to the Roku Search UI to find and
/// (optionally) launch content from an available provider.
#[derive(Debug, Default)]
pub struct SearchRequest<'a, 'b> {
    /// The content title, channel name, person name, or keyword to be searched.
    pub keyword: &'a str,
    // - TODO: check if the link below is valid.
    /// Treat [self.keyword](https://docs.rs/roku_ecp/struct.Search.html) as a
    /// title. This is the exact content title, channel name, person name, or
    /// keyword to be matched (ASCII case-insensitive).
    pub is_title: bool,
    /// Type of item being searched. This parameter is recommended as otherwise
    /// the search results are unconstrained and may cause the desired item to
    /// not be found due to result limits
    pub search_type: Option<SearchType>,
    /// A TMS ID for a movie, TV show, or person. If known, this parameter
    /// is recommended to be passed in conjunction with the `search_type`
    /// parameter as this should most likely provide the desired search
    /// result.
    pub tmsid: Option<&'a str>,
    /// A season number for a TV show (series), e.g. 1, 2, 3, ... If specified
    /// for a tv-show search, and the TV show is found, the specified season
    /// will be picked in the Seasons list to be launched. If not specified or
    /// not found, the default (typically most recent) season will be selected.
    pub season: Option<u32>,
    /// Allows the general keyword search results to include upcoming movie /
    /// tv-shows that are not currently available on Roku.
    pub show_unavailable: bool,
    /// If there are multiple results matching the query, automatically selects
    /// the arbitrary first result. If this is not specified, the search will
    /// stop if the results do not indicate a unique result.
    pub match_any: bool,
    /// One or more Roku channel IDs specifying the preferred/target provider.
    /// The search will iterate through the ids until a matching provider is
    /// found.
    pub provider_ids: Option<&'b [&'a str]>,
    /// One or more Roku channel titles specifying the preferred/target
    /// provider. Titles must be a full case-insensitive match. The search will
    /// iterate through the channel titles until a matching provider is found.
    pub providers: Option<&'b [&'a str]>,
    /// Specifies that if the search content is found and a **specified provider
    /// is found/installed**, the provider's channel should be launched.
    pub launch: bool,
}

impl<'a, 'b> SearchRequest<'a, 'b> {
    /// Creates a search request for a specific keyword.
    pub fn new(keyword: &'a str) -> SearchRequest<'a, 'b> {
        SearchRequest {
            keyword,
            ..Default::default()
        }
    }

    /// Specify the type of item being searched.
    pub fn search_type(mut self, search_type: SearchType) -> SearchRequest<'a, 'b> {
        self.search_type = Some(search_type);
        self
    }

    /// Have the search prioritize a list of providers from their IDs. The search will go down
    /// the list until a listed provider is found.
    pub fn provider_ids(mut self, ids: &'b [&'a str]) -> SearchRequest<'a, 'b> {
        self.provider_ids = Some(ids);
        self
    }

    /// Have the search prioritize a list of providers from their title. The search will go down
    /// the list until a listed provider is found.
    pub fn providers(mut self, titles: &'b [&'a str]) -> SearchRequest<'a, 'b> {
        self.providers = Some(titles);
        self
    }

    /// Set the season of a TV show to search for.
    pub fn season(mut self, season: u32) -> SearchRequest<'a, 'b> {
        self.season = Some(season);
        self
    }

    /// Set the TMS ID for a movie, TV show, or person.
    pub fn tmsid(mut self, id: &'a str) -> SearchRequest<'a, 'b> {
        self.tmsid = Some(id);
        self
    }

    /// Automatically select the first result from the search query.
    pub fn match_any(mut self) -> SearchRequest<'a, 'b> {
        self.match_any = true;
        self
    }

    /// Treat the Search keyword as a title instead.
    pub fn is_title(mut self) -> SearchRequest<'a, 'b> {
        self.is_title = true;
        self
    }

    /// Launch the search result, if found with a specified provider that's installed.
    pub fn launch(mut self) -> SearchRequest<'a, 'b> {
        self.launch = true;
        self
    }

    /// Include unavailable listing in search results.
    pub fn show_unavailable(mut self) -> SearchRequest<'a, 'b> {
        self.show_unavailable = true;
        self
    }
}
