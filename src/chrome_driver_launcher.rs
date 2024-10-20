use std::future::Future;
use std::io::{BufRead, BufReader, Read};
use std::pin::Pin;
use std::process;
use std::task::{Context, Poll};

pub struct ChromeDriverLauncher {
    child: process::Child,
    stdout_reader: BufReader<process::ChildStdout>,
    stdout_str: String,
    port: String,
    expected_output: String,
}

impl ChromeDriverLauncher {
    pub fn launch(port: String) -> Self {
        let mut cmd = process::Command::new("./chromedriver-win64/chromedriver.exe");
        cmd.args([format!("--port={}", port)]);
        cmd.stdout(process::Stdio::piped());
        cmd.stderr(process::Stdio::piped());
        let mut child = fatal_unwrap_e!(cmd.spawn(), "Failed to start chromedriver {}");
        let stdout = fatal_unwrap!(child.stdout.take(), "Failed to get stdout");
        let expected_output = format!("ChromeDriver was started successfully on port {}", port);
        Self {
            child,
            stdout_reader: BufReader::new(stdout),
            stdout_str: String::new(),
            port,
            expected_output,
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

        let stdout_reader = &mut inner.stdout_reader;
        inner.stdout_str.clear();

        fatal_unwrap_e!(stdout_reader.read_line(&mut inner.stdout_str), "Failed to read stdout {}");

        if inner.stdout_str.len() > 0 {
            info!("{}", inner.stdout_str);
            let option = inner.stdout_str.find(&inner.expected_output);
            if option.is_some() {
                return Poll::Ready(Ok(()));
            }
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
