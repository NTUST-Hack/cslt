use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{cookie::CookieStore, header::HeaderValue};
use reqwest_cookie_store::CookieStoreMutex;

#[async_trait]
pub trait LoginMethod {
    async fn login(
        &self,
        http_client: &reqwest::Client,
        cookie_store: std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>,
    ) -> Result<(), Box<dyn std::error::Error>>;
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

#[async_trait]
impl LoginMethod for LoginBySecret {
    async fn login(
        &self,
        http_client: &reqwest::Client,
        cookie_store: Arc<CookieStoreMutex>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let secret_cookie = format!(
            "ntustsecret={}; domain=.ntust.edu.tw; expires=Tue, 19 Jan 2038 04:14:07 GMT; path=/; secure; HttpOnly",
            self.secret
        );
        let cookie = HeaderValue::from_str(&secret_cookie)?;

        let url = &self.login_page_url.parse()?;
        cookie_store.set_cookies(&mut [cookie].iter(), &url);

        let _resp = http_client.get(&self.login_page_url).send().await?;
        // {
        //     println!("{}", _resp.text().await.unwrap());

        //     let store = cookie_store.lock().unwrap();
        //     for c in store.iter_any() {
        //         println!("{:?}", c);
        //     }
        // }

        Ok(())
    }
}
