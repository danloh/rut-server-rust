// a simple page crawle

use scraper::{Html, Selector};
use reqwest;
use regex::Regex;

use crate::model::item::NewItem;

#[derive(Debug,Clone)]
pub struct WebPage {
    url: String,
    html: String,
    domain: String,
}

impl WebPage {
    pub fn new(url: &str) -> Self {
        let mut res = reqwest::get(url).unwrap().text().unwrap();

        lazy_static! {
            static ref Scheme_re: Regex = Regex::new(r"^https?://$").unwrap();  
            static ref Path_re: Regex = Regex::new(r"^/.*$").unwrap();
        }

        let uri = Scheme_re.replace_all(url, "");
        let host = Path_re.replace_all(&uri, "");
        let domain = host.replace("www.", "");

        Self {
            url: url.to_string(),
            html: res,
            domain,
        }
    }

    // URL getter
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    // HTML parser
    pub fn get_html(&self) -> Html {
        Html::parse_document(&self.html)
    }

    pub fn into_item(&self) -> NewItem {
        let url = self.get_url();
        let html = self.get_html();
        parse_amz_page(url, html)
    }
}

pub fn parse_amz_page(url: String, html: Html) -> NewItem {

    let title_selector = Selector::parse("head > title").unwrap();
    let img_selector = Selector::parse("#imgBlkFront").unwrap();

    // get title
    let titles: Vec<_> = html.select(&title_selector).collect();
    
    let mut title_text: String;
    if titles.len() > 0 {
        let title = titles[0];
        title_text = title.inner_html();
    } else {
        title_text = "untitled, please help to update".to_owned();
    }

    // let title_parts: Vec<&str> = title_text.split(":").collect();
    
    // get cover image url
    let imgs: Vec<_> = html.select(&img_selector).collect();
    let mut img_src: String;
    if imgs.len() > 0 {
        let img = imgs[0];
        match img.value().attr("src") {
            Some(src) => { img_src = src.to_owned(); },
            None => { img_src = "".to_owned(); },
        }
    } else {
        img_src = "".to_owned();
    }

    NewItem{
        title: title_text.clone(),
        cover: img_src,
        url: url,
        detail: title_text,
        ..NewItem::new()
    }
}
