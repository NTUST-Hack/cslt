use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::redirect;
use reqwest_cookie_store::CookieStoreMutex;

use crate::{
    login::LoginMethod,
    page::{DetailsPage, SelectResultPage},
};

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_CHOOSE_LIST_URL: &'static str =
    "https://courseselection.ntust.edu.tw/ChooseList/D01/D01";
const SELECT_COURSE_API_URL_PRE: &'static str =
    "https://courseselection.ntust.edu.tw/First/A06/ExtraJoin";
const SELECT_COURSE_API_URL_STARTED: &'static str =
    "https://courseselection.ntust.edu.tw/AddAndSub/B01/ExtraJoin";

pub struct Client {
    http_client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
    choose_list_url: String,

    // select api url
    select_course_api_url_pre: String,
    select_course_api_url_started: String,
}

impl Client {
    pub fn new() -> Self {
        Client::build(None, None, None, None, None).unwrap()
    }

    pub fn build(
        user_agent: Option<&str>,
        timeout: Option<Duration>,
        choose_list_url: Option<&str>,
        select_course_api_url_pre: Option<&str>,
        select_course_api_url_started: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie_store = reqwest_cookie_store::CookieStore::new(None);
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

            select_course_api_url_pre: String::from(
                select_course_api_url_pre.unwrap_or(SELECT_COURSE_API_URL_PRE),
            ),
            select_course_api_url_started: String::from(
                select_course_api_url_started.unwrap_or(SELECT_COURSE_API_URL_STARTED),
            ),
        })
    }

    pub async fn login(&self, method: &dyn LoginMethod) -> Result<(), Box<dyn std::error::Error>> {
        self.clear().await?;

        method
            .login(&self.http_client, Arc::clone(&self.cookie_store))
            .await?;

        Ok(())
    }

    pub async fn select_course<'a>(
        &self,
        mode: SelectMode<'a>,
        course_no: &str,
    ) -> Result<SelectResultPage, Box<dyn std::error::Error>> {
        let api_url = match mode {
            SelectMode::Pre => self.select_course_api_url_pre.as_str(),
            SelectMode::Started => self.select_course_api_url_started.as_str(),
            SelectMode::Custom(url) => url,
        };

        let mut params = HashMap::new();
        params.insert("CourseNo", course_no);
        params.insert("type", "1");

        let resp = self.http_client.post(api_url).form(&params).send().await?;
        let text = resp.text().await?;

        Ok(SelectResultPage::new(&text))
    }

    pub async fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        // clear the cookie store
        self.cookie_store.lock().unwrap().clear();

        Ok(())
    }

    pub async fn refresh_details(&self) -> Result<DetailsPage, Box<dyn std::error::Error>> {
        // println!("url: {}", &self.choose_list_url);

        // let store = self.cookie_store.lock().unwrap();
        // let url = Url::from_str(&self.choose_list_url).unwrap();
        // for c in self.cookie_store.cookies(&url) {
        //     println!("{:?}", c);
        // }

        let resp = self.http_client.get(&self.choose_list_url).send().await?;
        let text = resp.text().await?;
        // println!("{}", text);

        Ok(DetailsPage::new(&text.as_str()))
    }
}

pub enum SelectMode<'a> {
    Pre,
    Started,
    Custom(&'a str),
}
