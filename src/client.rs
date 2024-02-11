use std::{sync::Arc, time::Duration};

use reqwest::redirect;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use skyscraper::html;

use crate::{login::LoginMethod, page::DetailsPage};

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
// const DEFAULT_CHOOSE_LIST_URL: &'static str =
//     "https://courseselection.ntust.edu.tw/ChooseList/D01/D01";
const DEFAULT_CHOOSE_LIST_URL: &'static str = "https://test.hayden.tw/test2";

pub struct Client {
    http_client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
    choose_list_url: String,
}

impl Client {
    pub fn new() -> Self {
        Client::build(None, None, None).unwrap()
    }

    pub fn build(
        user_agent: Option<&str>,
        timeout: Option<Duration>,
        choose_list_url: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie_store = CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);

        Ok(Client {
            cookie_store: Arc::clone(&cookie_store),
            http_client: reqwest::Client::builder()
                .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT))
                .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))
                .redirect(redirect::Policy::limited(10))
                .cookie_provider(Arc::clone(&cookie_store))
                .build()?,
            choose_list_url: String::from(choose_list_url.unwrap_or(DEFAULT_CHOOSE_LIST_URL)),
        })
    }

    pub async fn login(&self, method: &dyn LoginMethod) -> Result<(), Box<dyn std::error::Error>> {
        self.clear().await?;

        method
            .login(&self.http_client, Arc::clone(&self.cookie_store))
            .await?;

        Ok(())
    }

    pub async fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        // clear the cookie store
        self.cookie_store.lock().unwrap().clear();

        Ok(())
    }

    pub async fn refresh_details(&self) -> Result<DetailsPage, Box<dyn std::error::Error>> {
        println!("url: {}", &self.choose_list_url);
        let resp = self.http_client.get(&self.choose_list_url).send().await?;
        let text = resp.text().await?;

        Ok(DetailsPage {
            doc: html::parse(&text)?,
        })
    }
}
