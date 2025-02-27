use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(serde::Deserialize)]
pub enum Error {
    ErrorKeyDoesNotExist = 1,
    ErrorNoSlotAvailable = 2,
    ErrorZeroCaptchaFilesize = 3,
    ErrorTooBigCaptchaFilesize = 4,
    ErrorPageUrl = 5,
    ErrorZeroBalance = 10,
    ErrorIpNotAllowed = 11,
    ErrorCaptchaUnsolvable = 12,
    ErrorBadDuplicates = 13,
    ErrorNoSuchMethod = 14,
    ErrorImageTypeNotSupported = 15,
    ErrorNoSuchCaptchaId = 16,
    ErrorIpBlocked = 21,
    ErrorTaskAbsent = 22,
    ErrorTaskNotSupported = 23,
    ErrorRecaptchaInvalidSiteKey = 31,
    ErrorAccountSuspended = 55,
    ErrorBadProxy = 130,
    ErrorBadParameters = 110,
    ErrorBadImgInstructions = 115,
}

#[derive(serde::Deserialize)]
pub enum Status {
    Processing,
    Ready,
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Processing => write!(f, "processing"),
            Status::Ready => write!(f, "ready"),
            Status::Error => write!(f, "error"),
        }
    }
}
#[derive(serde::Deserialize)]
pub struct Solve {
    error_id: u8,
    task_id: u32,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResult {
    error_id: Error,
    status: String,
    solution: Solution,
    cost: String,
    ip: String,
    create_time: u64,
    end_time: u64,
    solve_count: u32,
}

#[derive(serde::Deserialize)]
pub struct Solution {
    token: String,
}
