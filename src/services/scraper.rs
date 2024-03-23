use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_LANGUAGE, CONNECTION, COOKIE, DNT, HOST, UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};
use scraper::{Html, Selector};

pub struct Scraper {
    _client: reqwest::Client,
    headers: HeaderMap,
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

impl Scraper {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(HOST, "www.sweetmarias.com".parse().unwrap());
        headers.insert(USER_AGENT, "SOME_AGENT/1.0".parse().unwrap());
        headers.insert(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());
        headers.insert(DNT, "1".parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        headers.insert(COOKIE, "private_content_version=4425483136aa037ce455aa6e3eb78cf9; wp_ga4_customerGroup=NOT%20LOGGED%20IN; pr-cookie-consent=[%22all%22]; user_allowed_save_cookie=%7B%221%22%3A1%7D; __cf_bm=_CWMX3ty9baJczJn5ClJEpphNDwm6fr0uk_l2VPN7qw-1711162555-1.0.1.1-86wY6tvuDQZBoCp41F9LTmUC8mg12D1_Z4egDqUQK4r2G5UnSDIcRhfZE_vpShi1XImp6.cgFvDATFX3aYzT2w; form_key=e2EaordnT2xHbYUA; mage-cache-storage={}; mage-cache-storage-section-invalidation={}; PHPSESSID=7dd03719a23ed3c7b06e1ff0e85c4582; form_key=e2EaordnT2xHbYUA; X-Magento-Vary=ad2b3159231b8c9196b420e3d6a9f26286e2fbb1def2cd7a9333c730c25b9c90; mage-cache-sessid=true".parse().unwrap());
        headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
        headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
        headers.insert("TE", "trailers".parse().unwrap());

        Scraper {
            _client: client,
            headers,
        }
    }

    pub async fn get_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let res = self
            ._client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?;

        let status = res.status();
        if !status.is_success() {
            return Err(format!("Failed to scrape URL: {}", status).into());
        }
        let text_response = res.text().await?;
        Ok(text_response)
    }

    pub fn parse_directory_html(&self, html: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a.product-item-link").unwrap();
        let mut links = Vec::new();
        for element in document.select(&selector) {
            if let Some(link) = element.value().attr("href") {
                links.push(link.to_string());
            }
        }
        links
    }

    pub fn strip_html_tags(&self, html: &str) -> String {
        let mut result = String::new();
        let mut copy = String::new();
        let mut inside = false;
        let mut skip = false;
        let mut skip_ws = false;
        for c in html.chars() {
            copy.push(c);
            // skip script and style tags
            if copy.ends_with("<script") || copy.ends_with("<style") || copy.ends_with("<noscript") {
                skip = true;
            }
            if copy.ends_with("</script>")
                || copy.ends_with("</style>")
                || copy.ends_with("</noscript>")
            {
                skip = false;
            }
            if !c.is_whitespace() {
                skip_ws = false;
            }
            if c == '<' {
                inside = true;
            } else if c == '>' {
                inside = false;
                result.push(' ');
            } else if !inside && !skip && !skip_ws {
                result.push(c);
                if c.is_whitespace() {
                    skip_ws = true;
                }
            }
        }
        result.replace('\n', "")
    }
}
