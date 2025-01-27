use crate::errors::CrawlerResult;

pub trait Session
where
    Self: Send,
{
    async fn quit(self) -> CrawlerResult<()>;
}
