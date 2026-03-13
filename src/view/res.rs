use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::weav3r::profit::FavoritesData;

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct SettingConfigRes {
    // 启动开关
    pub is_run: bool,
    // 音频开关
    pub audio_switch: bool,
    // 间隔时间
    pub interval: f32,
    // 新商品置顶时间
    pub product_top_time: u32,
    // 利润百分比
    pub profit_percent: f32,
    // 最低利润
    pub min_profit: i64,
    // 官方回收最低价
    pub office_price_start: u64,
    // 官方回收利润
    pub office_profit: u64,
    // 目标id
    pub target_ids: Vec<i32>,
    pub token: String,
    pub cookie: String,
}

impl Default for SettingConfigRes {
    fn default() -> Self {
        Self {
            is_run:true,
            audio_switch: true,
            product_top_time: 30,
            interval: 5.0,
            profit_percent: 4.0,
            min_profit: 20000,
            office_price_start: 300,
            office_profit: 1000,
            target_ids: vec![385,260,903,263,617,272,264,271,267,277,282,276,186,187,215,261,618,273,258,266,268,269,281,274,384,533,555,532,554,530,553,987,986,985,206,586,587,151,556,529,528,36,527,310,35,210,39,37,209,38,541,552,542,638,551,531,550,818,283,370,364,1080,1079,1082,1083,1078,1081,367,366,369],
            token: "40c02b7759e44962c766b731e69c0b233256163e6c".to_string(),
            cookie: "session_id=8d38c2aa35c9065ba739e166c9bd95c5334a2c02d5dc259442369aad1a459d4a; cf_clearance=DjAcxoAoJZ.PC6ohHgwp2HImMTwq47vTAnamRJhXG_A-1772550463-1.2.1.1-1Kl9Rxm_SNTXq7EZqlhS1YVHojJH67azadS3NgNKsxYSRM0u1bMtY9Sp4OD66SS.vSgqW6G447V9exY416GqNGE1oai47zqzKOPZML0llW.B6jcvbEUvCHCLNRqvNdXDbZ9Jhc94sAX4nhz3S3MTeRWo27kFnMqoe1lKDeIpdd6vCD7sVWUHVmCcabXOiQfhnp0gg3vewAS7KHmK7OcWEZPVWYF1mCNyvYzI6THUGaI".to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct Weav3rFavRes(pub FavoritesData);
