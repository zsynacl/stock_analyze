use serde::{Deserialize, Serialize};
use reqwest::Client;
use tokio::prelude::*;
use stock_analyze::{
    database::*,
    datamodel::*
};
use diesel::sqlite::*;
use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;
use chrono::prelude::*;

// const DETAIL_STRING : &'static str = "http://www.csindex.com.cn/zh-CN/indices/index-detail/{}?earnings_performance=5年&data_type=json";
// const INDEX_LIST_ADDRESS : &'static str = "http://www.csindex.com.cn/zh-CN/indices/index?page={}&page_size=50&by=asc&order=发布时间&data_type=json";

fn save_index_info(index_list : &Vec<IndexDescription>) -> Result<u8, ()>{
    let mut result = 0;

    Ok(result)
}

/// 返回总页数和第一页内容
async fn get_page(client: &Client, page_index: u8) -> Result<IndexList, ()> {
    let url = format!("http://www.csindex.com.cn/zh-CN/indices/index?page={}&page_size=50&by=asc&order=发布时间&data_type=json", page_index);
    let send_result = client
        .get(&url)
        .send().await;
    if let Ok(rsp) = send_result {
        if let Ok(content) = rsp.text().await {
            println!("{}", content.get(0..4).unwrap());
            let result = serde_json::from_str::<IndexList>(content.trim_start_matches("\u{feff}"));
            if let Ok(list) = result {
                return Ok(list);
            };
            println!("error convert json string from url: {}", url);
            println!("reason: {}", result.unwrap_err());
        };
    };
    Err(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_file = "database.db";
    let now = Local::now();
    let list_cache_file = now.format("%Y-%m-%d").to_string();
    let file = std::path::Path::new(db_file);

    let client : Client = Client::new();
    let list : IndexList = get_page(&client, 1).await.unwrap(); //.unwrap_or(IndexList::new());
    println!("total page: {}, total record count: {}, page size: {}",
             list.total_page, list.total, list.page_size);
    let mut full_list = list.list;
    for i in 2..=list.total_page {
        let mut new_list: Vec<IndexDescription> = get_page(&client, i as u8).await.unwrap_or(IndexList::new()).list;
        full_list.append(new_list.as_mut());
    }

    Ok(())
}