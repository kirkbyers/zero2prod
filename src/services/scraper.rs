use std::collections::HashMap;

use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, DNT, HOST,
    UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use scraper::{Html, Selector};

pub struct Scraper {
    client: reqwest::Client,
    headers: HeaderMap,
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

impl Scraper {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .build()
            .expect("Failed to build client");

        let mut headers = HeaderMap::new();
        headers.insert(HOST, "www.sweetmarias.com".parse().unwrap());
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".parse().unwrap());
        headers.insert(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());
        headers.insert(DNT, "1".parse().unwrap());
        headers.insert(CONNECTION, "keep-alive".parse().unwrap());
        // Remove the COOKIE header to avoid sending potentially outdated or suspicious cookies
        headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
        headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
        headers.insert("Cache-Control", "max-age=0".parse().unwrap());

        Scraper { client, headers }
    }

    pub async fn get_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let res = self
            .client
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

    pub fn sm_item_listing_to_details(&self, html: &str) -> HashMap<String, String> {
        let document = Html::parse_document(html);

        let table_selectors: HashMap<_, _> = [
            ("region", "td[data-th='Region']"),
            ("processing", "td[data-th='Processing']"),
            ("drying", "td[data-th='Drying Method']"),
            ("arrival", "td[data-th='Arrival date']"),
            ("lot_size", "td[data-th='Lot size']"),
            ("bag_size", "td[data-th='Bag size']"),
            ("packaging", "td[data-th='Packaging']"),
            ("farm_gate", "td[data-th='Farm Gate']"),
            ("cultivar_detail", "td[data-th='Cultivar Detail']"),
            ("grade", "td[data-th='Grade']"),
            ("appearance", "td[data-th='Appearance']"),
            ("roast_rec", "td[data-th='Roast Recommendations']"),
            ("coffee_type", "td[data-th='Type']"),
            ("spro_rec", "td[data-th='Recommended for Espresso']"),
            ("score", "div.score-value"),
        ]
        .iter()
        .cloned()
        .collect();

        let table_results: HashMap<String, String> = table_selectors
            .iter()
            .map(|(key, selector)| {
                let selector = Selector::parse(selector).unwrap();
                let mut result = String::new();
                for element in document.select(&selector) {
                    result.push_str(element.inner_html().trim());
                }
                (String::from(*key), result)
            })
            .collect();

        table_results
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
            if copy.ends_with("<script") || copy.ends_with("<style") || copy.ends_with("<noscript")
            {
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
            if copy.ends_with("You may also like ...") {
                break;
            }
        }
        result = result.replace('\n', "");
        if let Some(index) = result.find("You may also like ...") {
            result.truncate(index);
        }
        let start_trim = " 0 --          0 -- ";
        if let Some(index) = result.find(start_trim) {
            result = result[start_trim.len() + index..].to_string();
        }
        result.trim().to_string()
    }
}
