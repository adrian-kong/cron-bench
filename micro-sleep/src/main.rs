use std::io;
use std::str::FromStr;
use std::time::Duration;
use chrono::Utc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let schedule = cron::Schedule::from_str("* * * * *").unwrap();
    let mut sched_iter = schedule.upcoming(Utc);
    // loop {
    //     let _next = sched_iter.next();
    //     tokio::select! {
    //         // _ = tokio::time::sleep_until() =
    //         //             //
    //         //             // },> {
    //         _ = tokio::time::sleep(Duration::from_secs(60)) => {
    //             break;
    //         }
    //     }
    // }

    Ok(())
}
