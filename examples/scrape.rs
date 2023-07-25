use std::collections::HashMap;
use reqwest::{
    Client,
};
use serde;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct SearchQuery {
    search: &'static str,
    keyword: &'static str,
    product_type: &'static str,
    product_no: String,
    card_page: String,
    card_kind: &'static str,
    rarelity: &'static str,
}

impl SearchQuery {
    fn new(product_no: String, card_page: i32) -> SearchQuery {
        SearchQuery {
            search: "",
            keyword: "",
            product_type: "booster",
            product_no: product_no.clone(),
            card_page: card_page.to_string(),
            card_kind: "",
            rarelity: "",
        }
    }

    fn into_hashmap<'a>(self) -> HashMap<&'a str, String> {
        let mut form = HashMap::new();
        form.insert("search", self.search.to_string());
        form.insert("keyword", self.keyword.to_string());
        form.insert("product_type", self.product_type.to_string());
        form.insert("product_no", self.product_no.to_string());
        form.insert("card_page", self.card_page);
        form.insert("card_kind", self.card_kind.to_string());
        form.insert("rarelity", self.rarelity.to_string());
        form
    }

    fn to_filename(self) -> String {
        format!("{}/{}_p{}.html", self.product_type, self.product_no, self.card_page)
    }
}

fn try_mkdir(rel_path: &Path) -> Result<(), std::io::Error> {
    if !rel_path.exists() {
        fs::create_dir(rel_path)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    try_mkdir(Path::new("./text_cache")).unwrap();
    simple_request().await.unwrap();
}

pub async fn simple_request() -> Result<(), reqwest::Error> {
    let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";

    let search_query = SearchQuery::new(String::from("WXi-14"), 1);
    let form = search_query.clone().into_hashmap();

    let client: Client = Client::new();
    let res = client.post(url)
        .header(reqwest::header::COOKIE, "wixAge=conf;")
        .form(&form)
        .send().await?;

    let body = res.text().await.unwrap();

    let cache_filename = std::path::PathBuf::from(format!("./text_cache/{}", search_query.clone().to_filename()));

    if let Some(parent_path) = cache_filename.parent() {
        try_mkdir(&parent_path).unwrap();

        let document = Html::parse_document(&body);
        let main_selector = Selector::parse("#dipThum").unwrap();


        let file = File::create(&cache_filename);
        if let Ok(mut file_) = file {
            for element in document.select(&main_selector) {
                println!("{}", element.inner_html());
                file_.write_all(element.inner_html().as_bytes()).unwrap();
            }
        }
    }

    Ok(())
}