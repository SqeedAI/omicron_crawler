use crate::two_captcha::traits;
use crate::{azure, linkedin};

pub struct ApiClient<TwoCaptchaClient>
where
    TwoCaptchaClient: traits::TwoCaptchaClient,
{
    pub linkedin_client: linkedin::Client,
    pub azure_client: azure::AzureClient,
    pub two_captcha_client: TwoCaptchaClient,
}

impl<TwoCaptchaClient> ApiClient<TwoCaptchaClient>
where
    TwoCaptchaClient: traits::TwoCaptchaClient,
{
    pub fn new(linkedin_client: linkedin::Client, azure_client: azure::AzureClient, two_captcha_client: TwoCaptchaClient) -> Self {
        Self {
            linkedin_client,
            azure_client,
            two_captcha_client,
        }
    }
}
