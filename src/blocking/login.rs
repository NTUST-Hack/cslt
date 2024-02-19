use std::sync::Arc;

use reqwest::{cookie::CookieStore, header::HeaderValue};
use reqwest_cookie_store::CookieStoreMutex;

pub trait LoginMethod {
    fn login(
        &self,
        http_client: &reqwest::blocking::Client,
        cookie_store: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
    ) -> anyhow::Result<()>;
}

const DEFAULT_LOGIN_PAGE_URL: &'static str =
    "https://courseselection.ntust.edu.tw/Account/SingleSignOnLogin";

pub struct LoginBySecret {
    secret: String,
    login_page_url: String,
}

impl LoginBySecret {
    pub fn new(secret: &str) -> Self {
        LoginBySecret {
            secret: String::from(secret),
            login_page_url: String::from(DEFAULT_LOGIN_PAGE_URL),
        }
    }
}

impl LoginMethod for LoginBySecret {
    fn login(
        &self,
        http_client: &reqwest::blocking::Client,
        cookie_store: Arc<CookieStoreMutex>,
    ) -> anyhow::Result<()> {
        let secret_cookie = format!(
            "ntustsecret={}; domain=.ntust.edu.tw; expires=Tue, 19 Jan 2038 04:14:07 GMT; path=/; secure; HttpOnly",
            self.secret
        );
        let cookie = HeaderValue::from_str(&secret_cookie)?;

        let url = &self.login_page_url.parse()?;
        cookie_store.set_cookies(&mut [cookie].iter(), &url);

        let _resp = http_client.get(&self.login_page_url).send()?;

        Ok(())
    }
}
