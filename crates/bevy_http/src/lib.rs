use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::prelude::*;
pub use ehttp::{Request, Response};

use crate::tools::HttpTool;

pub mod tools;

pub struct BevyHttpPlugin<T: HttpRequest> {
    marker: PhantomData<T>,
}

impl<T: HttpRequest> Default for BevyHttpPlugin<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T: HttpRequest> Plugin for BevyHttpPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_request_task::<T>);
    }
}

/// HTTP 请求 trait，用于构建 ehttp::Request
pub trait HttpRequest: Clone + Component {
    /// 成功结果类型
    type R: Send + Sync + 'static;
    /// 错误类型
    type E: From<String> + Send + Sync + 'static;

    /// 构建 ehttp::Request
    fn build_request(&self) -> HttpTool;

    /// 解析响应
    fn parse_response(response: &Response) -> std::result::Result<Self::R, Self::E>;
}

#[derive(Component)]
pub struct RequestTask {
    pub receiver: crossbeam_channel::Receiver<Result<Response, String>>,
}

impl RequestTask {
    pub fn new(receiver: crossbeam_channel::Receiver<Result<Response, String>>) -> Self {
        Self { receiver }
    }
}

fn create_request_task<T: HttpRequest>(
    mut commands: Commands,
    requests: Query<(Entity, &T), Without<RequestTask>>,
) {
    for (entity, request) in requests.iter() {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let request = request.clone();
        IoTaskPool::get()
            .spawn(async move {
                let req: Request = request.build_request().into();
                let result = ehttp::fetch_async(req).await;
                sender.send(result).unwrap();
            })
            .detach();

        commands.entity(entity).insert(RequestTask::new(receiver));
    }
}
