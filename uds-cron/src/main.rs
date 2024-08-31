use core::time::Duration;
use std::io;
use std::str::FromStr;
use std::sync::Arc;
use chrono::Utc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> io::Result<()> {
    let schedule = cron::Schedule::from_str("* * * * * * *").unwrap();
    let sched_iter = schedule.upcoming(Utc);
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        notify_clone.notify_waiters();
    }).await?;
    #[cfg(unix)]
    {
        use tokio::net::UnixListener;
        use std::path::Path;
        println!("running ");
        let socket_path = "/tmp/test.sock";
        let rm_err = std::fs::remove_file(Path::new(socket_path)).err();
        println!("failed to remove sock file {rm_err:?}, this should be ok.");
        let listener = UnixListener::bind(socket_path)?;
        loop {
            tokio::select! {
                _ = notify.notified() => break,
                ret = listener.accept() => {
                    match ret {
                        Ok((_, _)) => {
                            // perform update
                            println!("updating...")
                        }
                        Err(e) => {
                            println!("could not accept unix listener on subscription renewal sock due to {e}");
                        }
                    }
                },
            }
        }
    }
    Ok(())
}
