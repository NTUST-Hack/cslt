use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::redirect;
use reqwest_cookie_store::CookieStoreMutex;

use crate::{
    login::LoginMethod,
    page::{DetailsPage, SelectResultPage},
    DEFAULT_CHOOSE_LIST_URL, DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, SELECT_COURSE_API_URL_PRE,
    SELECT_COURSE_API_URL_STARTED, SELECT_COURSE_PAGE_URL_PRE, SELECT_COURSE_PAGE_URL_STARTED,
};

pub struct ClientBuilder<'a> {
    reqwest_builder: reqwest::ClientBuilder,

    user_agent: &'a str,
    timeout: Duration,

    choose_list_url: &'a str,

    select_course_page_url_pre: &'a str,
    select_course_page_url_started: &'a str,

    select_course_api_url_pre: &'a str,
    select_course_api_url_started: &'a str,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> Self {
        ClientBuilder {
            reqwest_builder: reqwest::ClientBuilder::new(),
            user_agent: DEFAULT_USER_AGENT,
            timeout: DEFAULT_TIMEOUT,

            choose_list_url: DEFAULT_CHOOSE_LIST_URL,

            select_course_page_url_pre: SELECT_COURSE_PAGE_URL_PRE,
            select_course_page_url_started: SELECT_COURSE_PAGE_URL_STARTED,

            select_course_api_url_pre: SELECT_COURSE_API_URL_PRE,
            select_course_api_url_started: SELECT_COURSE_API_URL_STARTED,
        }
    }

    pub fn user_agent(mut self, user_agent: &'a str) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn choose_list_url(mut self, url: &'a str) -> Self {
        self.choose_list_url = url;
        self
    }

    pub fn select_course_page_url_pre(mut self, url: &'a str) -> Self {
        self.select_course_page_url_pre = url;
        self
    }

    pub fn select_course_page_url_started(mut self, url: &'a str) -> Self {
        self.select_course_page_url_started = url;
        self
    }

    pub fn select_course_api_url_pre(mut self, url: &'a str) -> Self {
        self.select_course_api_url_pre = url;
        self
    }

    pub fn select_course_api_url_started(mut self, url: &'a str) -> Self {
        self.select_course_api_url_started = url;
        self
    }

    pub fn local_address(mut self, addr: std::net::IpAddr) -> Self {
        self.reqwest_builder = self.reqwest_builder.local_address(addr);
        self
    }

    pub fn build(self) -> anyhow::Result<Client> {
        let cookie_store = reqwest_cookie_store::CookieStore::new(None);
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);

        Ok(Client {
            cookie_store: Arc::clone(&cookie_store),
            http_client: self
                .reqwest_builder
                .user_agent(self.user_agent)
                .timeout(self.timeout)
                .redirect(redirect::Policy::limited(10))
                .cookie_provider(Arc::clone(&cookie_store))
                .build()?,
            choose_list_url: String::from(self.choose_list_url),

            select_course_page_url_pre: String::from(self.select_course_page_url_pre),
            select_course_page_url_started: String::from(self.select_course_page_url_started),

            select_course_api_url_pre: String::from(self.select_course_api_url_pre),
            select_course_api_url_started: String::from(self.select_course_api_url_started),
        })
    }
}

pub struct SelectResult<'a> {
    http_client: &'a reqwest::Client,

    page_url: String,
}

pub struct Client {
    http_client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
    choose_list_url: String,

    // select page url
    select_course_page_url_pre: String,
    select_course_page_url_started: String,

    // select api url
    select_course_api_url_pre: String,
    select_course_api_url_started: String,
}

impl Client {
    pub fn new() -> Self {
        ClientBuilder::new().build().unwrap()
    }

    pub async fn login(&self, method: &dyn LoginMethod) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<SelectResult> {
        let (page_url, api_url) = match mode {
            SelectMode::Pre => (
                self.select_course_page_url_pre.as_str(),
                self.select_course_api_url_pre.as_str(),
            ),
            SelectMode::Started => (
                self.select_course_page_url_started.as_str(),
                self.select_course_api_url_started.as_str(),
            ),
            SelectMode::Custom(page_url, api_url) => (page_url, api_url),
        };

        let mut params = HashMap::new();
        params.insert("CourseNo", course_no);
        params.insert("type", "1");

        let _resp = self.http_client.post(api_url).form(&params).send().await?;
        // let text = resp.text().await?;

        Ok(SelectResult {
            http_client: &self.http_client,

            page_url: String::from(page_url),
        })
    }

    pub async fn clear(&self) -> anyhow::Result<()> {
        // clear the cookie store
        self.cookie_store.lock().unwrap().clear();

        Ok(())
    }

    pub async fn refresh_details(&self) -> anyhow::Result<DetailsPage> {
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

impl<'a> SelectResult<'a> {
    pub async fn result(&self) -> anyhow::Result<SelectResultPage> {
        let resp = self.http_client.get(&self.page_url).send().await?;
        let text = resp.text().await?;

        // println!("{text}");

        Ok(SelectResultPage::new(&text))
    }
}

pub enum SelectMode<'a> {
    Pre,
    Started,
    Custom(&'a str, &'a str), // page_url, api_url
}
