use bevy::{ecs::system::SystemId, prelude::*};
use bevy_http::{
    BevyHttpPlugin, HttpRequest, RequestTask,
    tools::{HttpMethod, HttpTool},
};

use crate::{
    model::{error::MyError, weav3r::favorites::FavoritesResponse},
    resource::items_data::OfficeItemsDbRes,
};

fn trigger_request(
    In((target_ids, next_action, cookie)): In<(String, String, String)>,
    mut cmd: Commands,
    items_database: Res<OfficeItemsDbRes>,
) {
    let f_target_ids = target_ids
        .split(',')
        .filter_map(|x| x.trim().parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let target_ids = items_database
        .items
        .iter()
        .filter(|x| x.tradeable && x.sell_price >= 300)
        .map(|x| x.id)
        .chain(f_target_ids)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let hp = Weav3rFavoriteHttp {
        target_ids,
        next_action: next_action.clone(),
        cookie: cookie.clone(),
    };

    cmd.spawn(hp);
}

pub struct Weav3rFavoriteHttpPlugin;

impl Plugin for Weav3rFavoriteHttpPlugin {
    fn build(&self, app: &mut App) {
        let sys_id = app.register_system(trigger_request);

        app.add_plugins(BevyHttpPlugin::<Weav3rFavoriteHttp>::default())
            .insert_resource(Weav3rSysResource(sys_id))
            .add_systems(Update, deal_response);
    }
}

// 出发请求的系统id
#[derive(Resource)]
pub struct Weav3rSysResource(pub SystemId<In<(String, String, String)>>);

#[derive(Component, Default)]
pub struct Weav3rRespComp {
    pub responses: FavoritesResponse,
    pub err: Option<MyError>,
}

#[derive(Clone, Component)]
pub struct Weav3rFavoriteHttp {
    pub target_ids: String,
    pub next_action: String,
    pub cookie: String,
}

impl HttpRequest for Weav3rFavoriteHttp {
    type R = FavoritesResponse;
    type E = MyError;

    fn build_request(&self) -> HttpTool {
        let mut http = HttpTool::default();
        http.set_url("https://weav3r.dev/favorites");
        // http.set_url("http://127.0.0.1:8666/favorites");
        http.set_method(HttpMethod::POST);
        http.add_header("Connection", "keep-alive");
        http.add_header("Accept", "text/x-component");
        http.add_header("Accept-Encoding", "gzip, deflate, br");
        http.add_header("Accept-Language", "zh-CN,zh;q=0.8");
        http.add_header("Content-Type", "text/plain;charset=UTF-8");
        http.add_header("Next-Action", self.next_action.as_str());
        http.add_header("Cookie", self.cookie.as_str());
        http.set_body(format!("[[{}]]", self.target_ids).as_bytes().to_vec());
        http
    }

    fn parse_response(response: &bevy_http::Response) -> std::result::Result<Self::R, Self::E> {
        if response.status != 200 {
            bevy::log::error!(
                "weav3r favorites api is not ok, status: {}",
                response.status
            );
            return Err(MyError::NetworkCode(
                response.status as i64,
                "weav3r favorites api is not ok".to_string(),
            ));
        }
        let Some(text) = response.text() else {
            return Err(MyError::ResponseTextIsNone);
        };
        let text = text.to_string();
        match FavoritesResponse::from_text(&text) {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

fn deal_response(mut cmd: Commands, requests: Query<(Entity, &Weav3rFavoriteHttp, &RequestTask)>) {
    for (entity, _, task) in requests.iter() {
        let Ok(response) = task.receiver.try_recv() else {
            continue;
        };
        let r = match response {
            Ok(r) => r,
            Err(e) => {
                bevy::log::error!("channel error: {:?}", e);
                cmd.entity(entity).despawn();
                cmd.spawn(Weav3rRespComp {
                    responses: FavoritesResponse::default(),
                    err: Some(MyError::ChannelError(e)),
                });
                continue;
            }
        };
        let res = match Weav3rFavoriteHttp::parse_response(&r) {
            Ok(r) => r,
            Err(e) => {
                bevy::log::error!("parse response is Err: {:?}", e);
                cmd.entity(entity).despawn();
                cmd.spawn(Weav3rRespComp {
                    responses: FavoritesResponse::default(),
                    err: Some(e),
                });
                continue;
            }
        };
        cmd.entity(entity).despawn();
        cmd.spawn(Weav3rRespComp {
            responses: res,
            err: None,
        });
    }
}
