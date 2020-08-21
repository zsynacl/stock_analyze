use serde::{Deserialize, Serialize};
use diesel;
use diesel::prelude::*;
use diesel::sqlite::*;
use dotenv::dotenv;
use std::env;
/// 代表了指数详细信息
///
/// example json data
/// ```json
/// {
///     "index_id": 1,
///     "index_code": "000001",
///     "indx_sname": "上证指数",
///     "index_ename": "SSE Index",
///     "num": "1548",
///     "tclose": "2887.43",
///     "yld_1_mon": "0.25",
///     "base_point": "100.00",
///     "base_date": "1990-12-19 00:00:00",
///     "online_date": "1991-07-15",
///     "is_custom": 0,
///     "is_show": 1,
///     "index_c_intro": "<p><p>上证综合指数由上海证券交易所上市的全部股票组成，包含A股和B股，综合反映上海证券市场上市股票的价格表现。</p></p>",
///     "index_e_intro": "<p><p>SSE&nbsp;Composite&nbsp;Index&nbsp;consists&nbsp;of&nbsp;all&nbsp;listed&nbsp;stocks&nbsp;(including&nbsp;A&nbsp;shares&nbsp;and&nbsp;B&nbsp;shares)&nbsp;on&nbsp;Shanghai&nbsp;Stock&nbsp;Exchange.&nbsp;The&nbsp;Index&nbsp;aims&nbsp;to&nbsp;reflect&nbsp;the&nbsp;overall&nbsp;performance&nbsp;of&nbsp;the&nbsp;Shanghai&nbsp;stock&nbsp;market.</p></p>",
///     "index_c_fullname": "上证综合指数",
///     "index_e_fullname": "SSE Composite Index",
///     "class_series": "上证系列指数",
///     "class_eseries": "SSE Indices",
///     "class_region": "境内",
///     "class_eregion": "China Mainland",
///     "class_assets": "股票",
///     "class_eassets": "Equity",
///     "class_classify": "规模",
///     "class_eclassify": "Size",
///     "class_currency": "人民币",
///     "class_ecurrency": "CNY",
///     "class_hot": "",
///     "class_ehot": null
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexDescription {
    pub index_id : u32,
    pub index_code : String,
    pub indx_sname : String,
    pub index_ename : String,
    pub num : String,
    pub tclose : String,
    pub yld_1_mon : String,
    pub base_point : String,
    pub base_date : String,
    pub online_date : String,
    pub is_custom : u8,
    pub is_show : u8,
    pub index_c_intro : String,
    pub index_e_intro : String,
    pub index_c_fullname : String,
    pub index_e_fullname : String,
    pub class_series : String,
    pub class_eseries : String,
    pub class_region : String,
    pub class_eregion : String,
    pub class_assets : String,
    pub class_eassets : String,
    pub class_classify : String,
    pub class_eclassify : String,
    pub class_currency : String,
    pub class_ecurrency : String,
    pub class_hot : String,
    pub class_ehot : Option<String>,
}

/// 指数列表
/// 示例json：
/// ```json
/// {
///     "total":1722,
///     "page_size":"50",
///     "total_page":36,
///     "list":[{}]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexList {
    /// 总记录数
    pub total : u16,
    /// 每页记录数
    pub page_size : String,
    /// 总页数
    pub total_page : u16,
    /// 当前页详细数据
    pub list : Vec<IndexDescription>,
}

impl IndexList {
    pub fn new() -> IndexList {
        IndexList {
            total : 0,
            page_size : String::from("0"),
            total_page : 0,
            list : Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexDetail {
    // "indx_code":"000001","tclose":"3095.12","tradedate":"2015-02-09 00:00:00","changes":"-2.77"}
    pub indx_code: String,
    pub tclose: String,
    pub tradedate: String,
    pub changes: String,
}