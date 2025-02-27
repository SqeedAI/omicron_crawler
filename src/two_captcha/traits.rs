use crate::errors::ApiResult;
use crate::two_captcha::res::{Solve, TaskResult};

pub trait TwoCaptchaClient {
    async fn solve(&self, website_public_key: &str, website_public_url: Option<&str>, subdomain: Option<&str>) -> ApiResult<Solve>;
    async fn get_task_result(&self, task_id: &str) -> ApiResult<TaskResult>;
}
