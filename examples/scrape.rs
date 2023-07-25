use std::collections::HashMap;
use reqwest::{Client, Response};
use serde;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct SearchQuery {
    search: String,
    keyword: String,
    product_type: ProductType,
    product_no: String,
    card_page: String,
    card_kind: String,
    rarelity: String,
}

#[derive(Clone)]
enum ProductType {
    Booster,
    Starter,
}

impl SearchQuery {
    fn new(product_no: String, product_type: ProductType, card_page: i32) -> SearchQuery {
        SearchQuery {
            search: "".into(),
            keyword: "".into(),
            product_type,
            product_no,
            card_page: card_page.to_string(),
            card_kind: "".into(),
            rarelity: "".into(),
        }
    }

    fn get_product_type(&self) -> String {
        match &self.product_type {
            ProductType::Booster => "booster".into(),
            _ => "starter".into(),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.get_product_type()),
            ("product_no".into(), self.product_no.clone()),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
        ])
    }

    fn to_filename(&self) -> String {
        format!("{}/{}_p{}.html", &self.get_product_type(), &self.product_no, &self.card_page)
    }

    fn cache_check(&self, dir: String) -> Result<String, std::io::Error> {
        let path: PathBuf = PathBuf::from(format!("{}/{}", dir, &self.to_filename()));
        if path.exists() {
            println!("cache found");
            let mut file: File = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            println!("cache not found");
            Err(std::io::Error::new(std::io::ErrorKind::Other, "An unexpected error occurred."))
        }
    }
}

fn try_mkdir(rel_path: &Path) -> Result<(), std::io::Error> {
    if !rel_path.exists() {
        fs::create_dir_all(rel_path)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_mkdir(Path::new("./text_cache")).unwrap();
    simple_request().await.unwrap();

    Ok(())
}

pub async fn simple_request() -> Result<(), reqwest::Error> {
    let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";

    let search_query: SearchQuery = SearchQuery::new(String::from("WXi-14"), ProductType::Booster, 1);

    match search_query.cache_check("./text_cache".to_string()) {
        Ok(content) => {
            println!("{}", content);
        }
        _ => {
            let form: HashMap<String, String> = search_query.to_hashmap();

            let client: Client = Client::new();
            let res: Response = client.post(url)
                .header(reqwest::header::COOKIE, "wixAge=conf;")
                .form(&form)
                .send().await?;

            let body: String = res.text().await.unwrap();

            let cache_filename: PathBuf = PathBuf::from(format!("./text_cache/{}", &search_query.to_filename()));

            if let Some(parent_path) = cache_filename.parent() {
                try_mkdir(&parent_path).unwrap();

                let document: Html = Html::parse_document(&body);
                let main_selector: Selector = Selector::parse("#dipThum").unwrap();

                let file: Result<File, std::io::Error> = File::create(&cache_filename);
                if let Ok(mut file_) = file {
                    for element in document.select(&main_selector) {
                        println!("{}", element.inner_html());
                        file_.write_all(element.inner_html().as_bytes()).unwrap();
                    }
                }
            }
        }
    }

    Ok(())
}