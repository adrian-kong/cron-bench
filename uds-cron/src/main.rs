use std::io;
use std::str::FromStr;
use chrono::Utc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let schedule = cron::Schedule::from_str("* * * * * * *").unwrap();
    let sched_iter = schedule.upcoming(Utc);
    #[cfg(unix)]
    {
        println!("running ");
        use tokio::net::UnixListener;
        use std::time::Duration;

        let listener = UnixListener::bind("/tmp/test.sock")?;
        loop {
            tokio::select! {
                ret = listener.accept() => {
                    match ret {
                        Ok((stream, _)) => {
                            // perform update
                            println!("updating...")
                        }
                        Err(e) => {
                            // println!("could not accept unix listener on subscription renewal sock due to {e}");
                        }
                    }
                },
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    break;
                }
            }
        }
    }
    println!("stopped ");
    Ok(())
}
