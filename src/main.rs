use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use winit::event_loop::EventLoop;

#[cfg(unix)]
use winit::platform::unix::EventLoopExtUnix;
#[cfg(windows)]
use winit::platform::windows::EventLoopExtWindows;

#[cfg(any(unix, windows))]
fn main() -> thread::Result<()> {
    let thread_handles = (0..10)
        .into_iter()
        .map(|_| {
            thread::spawn(|| {
                let _el = EventLoop::<()>::new_any_thread();
                thread::sleep(Duration::from_secs(1))
            })
        })
        .collect::<Vec<_>>();

    thread_handles.into_iter().try_for_each(JoinHandle::join)?;

    Ok(())
}
