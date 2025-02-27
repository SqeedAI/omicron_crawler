use crate::errors::ClientResult;
use crate::two_captcha::res::{Solve, TaskResult};

pub trait TwoCaptchaClient {
    async fn solve(&self, website_public_key: &str, website_public_url: &str, subdomain: Option<String>) -> ClientResult<Solve>;
    async fn get_task_result(&self, task_id: u32) -> ClientResult<TaskResult>;
}
