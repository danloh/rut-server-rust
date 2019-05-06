// a simple page crawle

use scraper::{Html, Selector};
use reqwest;
use regex::Regex;

use crate::model::item::NewItem;
use crate::model::{ re_test_img_url, replace_sep, trim_url_qry };

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
            static ref Scheme_re: Regex = Regex::new(r"https?://").unwrap();  
            static ref Path_re: Regex = Regex::new(r"/.*").unwrap();
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

    // Domain getter
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    // HTML parser
    pub fn get_html(&self) -> Html {
        Html::parse_document(&self.html)
    }

    pub fn into_item(&self) -> NewItem {
        let url = self.get_url();
        let html = self.get_html();
        let domain = self.get_domain();
        match domain.trim() {
            "amazon.com" => parse_amz_page(url, html),
            _ => parse_other_page(url, html),
        }  
    }
}

pub fn parse_amz_page(url: String, html: Html) -> NewItem {

    let title_selector = Selector::parse("head > title").unwrap();
    let protitle_selector = Selector::parse("#productTitle").unwrap();
    let img_selector = Selector::parse("#imgBlkFront").unwrap();

    // get html title
    let titles: Vec<_> = html.select(&title_selector).collect();
    
    let mut title_text: String;
    if titles.len() > 0 {
        let title = titles[0];
        title_text = title.inner_html();
    } else {
        title_text = "untitled".to_owned();
    }

    let title_parts: Vec<&str> = title_text.split(":").collect();
    // try get uid, author, not always works
    let p_len = title_parts.len() as i32;
    //println!("{:#?} and len {}", title_parts, p_len);
    let idx_uid = std::cmp::max(p_len - 3, 0 ) as usize;
    let uid = title_parts[idx_uid];
    let uiid = replace_sep(uid, "");
    let idx_author = std::cmp::max(p_len - 4, 0 ) as usize;
    let author = title_parts[idx_author];

    // get product title
    let ptitles: Vec<_> = html.select(&protitle_selector).collect();
    
    let mut ptitle_text: String;
    if ptitles.len() > 0 {
        let title = ptitles[0];
        ptitle_text = title.inner_html();
    } else {
        ptitle_text = title_text.clone();
    }
    
    // get cover image url
    let imgs: Vec<_> = html.select(&img_selector).collect();
    let mut img_src: String;
    if imgs.len() > 0 {
        let img = imgs[0];
        match img.value().attr("data-a-dynamic-image") {
            Some(src) => {
                let src_urls: Vec<&str> = src.split(":").collect();
                let src_url = src_urls[1];
                let img_src_url = ("https:".to_owned() + src_url).replace("\"", "");
                //println!("{}", img_src_url);
                img_src =
                if re_test_img_url(&img_src_url) { 
                    img_src_url
                } else { 
                    "".to_owned()
                };
            },
            None => { img_src = "".to_owned(); },
        }
    } else {
        img_src = "".to_owned();
    }

    NewItem{
        title: ptitle_text,
        uiid: uiid,
        authors: author.to_owned(),
        cover: img_src,
        url: trim_url_qry(&url, ""),
        category: "Book".to_owned(),
        detail: title_text,
        ..NewItem::new()
    }
}

pub fn parse_other_page(url: String, html: Html) -> NewItem {

    let title_selector = Selector::parse("head > title").unwrap();
    let img_selector = Selector::parse("img").unwrap();

    // get title
    let titles: Vec<_> = html.select(&title_selector).collect();
    
    let mut title_text: String;
    if titles.len() > 0 {
        let title = titles[0];
        title_text = title.inner_html();
    } else {
        title_text = "untitled, please help to update".to_owned();
    }
    
    // get cover image url
    let imgs: Vec<_> = html.select(&img_selector).collect();
    let mut img_src: String;
    if imgs.len() > 0 {
        let img = imgs[0];
        match img.value().attr("src") {
            Some(src) => { img_src = 
                if re_test_img_url(src) { 
                    src.to_owned() 
                } else { 
                    "".to_owned() 
                };
            },
            None => { img_src = "".to_owned(); },
        }
    } else {
        img_src = "".to_owned();
    }

    NewItem{
        title: title_text.clone(),
        cover: img_src,
        url: url,
        category: "WebPage".to_owned(),
        detail: title_text,
        ..NewItem::new()
    }
}

// todo: more specific parse
