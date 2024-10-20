use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::process;
use std::task::{Context, Poll};

pub struct ChromeDriverLauncher {
    child: process::Child,
    stdout: process::ChildStdout,
    stderr: process::ChildStderr,
    stdout_str: String,
    stderr_str: String,
}

impl ChromeDriverLauncher {
    pub fn launch() -> Self {
        let mut cmd = process::Command::new("chromedriver-win64/chromedriver.exe");
        cmd.args(["--port=9515"]);
        let mut child = cmd.spawn().expect("Failed to start chromedriver");
        let mut stdout = fatal_unwrap!(child.stdout.take(), "Failed to get stdout");
        let mut stderr = fatal_unwrap!(child.stderr.take(), "Failed to get stderr");
        Self {
            child,
            stdout,
            stderr,
            stdout_str: String::new(),
            stderr_str: String::new(),
        }
    }
}

impl Drop for ChromeDriverLauncher {
    fn drop(&mut self) {
        fatal_unwrap_e!(self.child.kill(), "Failed to kill chromedriver {}");
    }
}

impl Future for ChromeDriverLauncher {
    type Output = Result<(), ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = self.get_mut();
        const EXPECTED_OUTPUT: &str = "ChromeDriver was started successfully on port 9515";
        let stdout = &mut inner.stdout;
        let stderr = &mut inner.stderr;
        fatal_unwrap_e!(stdout.read_to_string(&mut inner.stdout_str), "Failed to read stdout {}");
        fatal_unwrap_e!(stderr.read_to_string(&mut inner.stderr_str), "Failed to read stderr {}");

        if inner.stderr_str.len() > 0 {
            error!("{}", inner.stderr_str);
            return Poll::Ready(Err(()));
        }

        if inner.stdout_str.len() > 0 {
            let option = inner.stdout_str.find(EXPECTED_OUTPUT);
            if option.is_some() {
                info!("{}", inner.stdout_str);
                return Poll::Ready(Ok(()));
            }
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}