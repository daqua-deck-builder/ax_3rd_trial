use std::collections::HashMap;
use reqwest::{Client, Response};
use serde;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use async_recursion::async_recursion;

#[derive(Clone)]
struct SearchQuery {
    search: String,
    keyword: String,
    product_type: ProductType,
    card_page: String,
    card_kind: String,
    rarelity: String,
}

#[derive(Clone)]
pub enum ProductType {
    Booster(String),
    Starter(String),
    PromotionCard,
    SpecialCard,
}

impl ProductType {
    fn get_path_relative(&self) -> String {
         match self {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard => String::from("special"),
            _ => "unknown".into(),
        }
    }
}

impl SearchQuery {
    fn new(product_type: &ProductType, card_page: i32) -> SearchQuery {
        SearchQuery {
            search: "".into(),
            keyword: "".into(),
            product_type: product_type.clone(),
            card_page: card_page.to_string(),
            card_kind: "".into(),
            rarelity: "".into(),
        }
    }

    fn get_product_type(&self) -> String {
        match &self.product_type {
            ProductType::Booster(product_no) => "booster".into(),
            ProductType::Starter(product_no) => "starter".into(),
            ProductType::PromotionCard => "-".into(),
            ProductType::SpecialCard => "-".into(),
            _ => "unknown".into(),
        }
    }

    fn get_cache_path_relative(&self) -> String {
        match &self.product_type {
            ProductType::Booster(product_no) => format!("booster/{}", product_no),
            ProductType::Starter(product_no) => format!("starter/{}", product_no),
            ProductType::PromotionCard => String::from("promotion"),
            ProductType::SpecialCard => String::from("special"),
            _ => "unknown".into(),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let empty_product_no = String::from("");

        let product_no = match &self.product_type {
            ProductType::Booster(product_no) => product_no,
            ProductType::Starter(product_no) => product_no,
            ProductType::PromotionCard => &empty_product_no,
            ProductType::SpecialCard => &empty_product_no,
        };

        HashMap::from_iter(vec![
            ("search".into(), self.search.clone()),
            ("keyword".into(), self.keyword.clone()),
            ("product_type".into(), self.get_product_type()),
            ("product_no".into(), product_no.clone()),
            ("card_page".into(), self.card_page.clone()),
            ("card_kind".into(), self.card_kind.clone()),
            ("rarelity".into(), self.rarelity.clone()),
        ])
    }

    fn to_filename(&self) -> String {
        format!("{}/p{}.html", &self.product_type.get_path_relative(), &self.card_page)
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

    let product_type = ProductType::Starter(String::from("WDA-F03"));
    // let product_type = ProductType::Booster(String::from("WXi-12"));

    simple_request(&product_type, 1).await.unwrap();

    Ok(())
}

#[async_recursion]
pub async fn simple_request(product_type: &ProductType, card_page: i32) -> Result<(), reqwest::Error> {
    let p_no = product_type.get_path_relative();
    println!("{} {}", p_no, card_page);


    let url = "https://www.takaratomy.co.jp/products/wixoss/card/card_list.php";

    let search_query: SearchQuery = SearchQuery::new(&product_type, card_page);

    let main: Option<String> = match search_query.cache_check("./text_cache".to_string()) {
        Ok(content_) => {
            Some(content_)
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

                let content = find_one(&body, ".cardDip".into());

                if let Some(element) = &content {
                    let file: Result<File, std::io::Error> = File::create(&cache_filename);
                    if let Ok(mut file_) = file {
                        file_.write_all(element.as_bytes()).unwrap();
                    }
                }
                content
            } else {
                None
            }
        }
    };

    if let Some(count) = find_one(&main.unwrap(), "h3 p span".into()) {
        let count = extract_number(&count);

        if let Some(count) = count {
            let pages = (count / 21) + 1;

            if card_page < pages {
                simple_request(&product_type, card_page + 1).await.unwrap();
            }
        }
    } else {
        println!("not found");
    }


    Ok(())
}

fn find_one(content: &String, selector: String) -> Option<String> {
    let document: Html = Html::parse_document(&content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();

    if let Some(element) = document.select(&main_selector).next() {
        Some(element.inner_html())
    } else {
        None
    }
}

fn find_many(content: &String, selector: String) -> Vec<String> {
    let document: Html = Html::parse_document(&content);
    let main_selector: Selector = Selector::parse(selector.as_str()).unwrap();
    let mut result: Vec<String> = Vec::new();
    for element in document.select(&main_selector) {
        result.push(element.inner_html());
    }
    result
}

fn extract_number(s: &String) -> Option<i32> {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse().ok()
}