use std::hint::black_box;
use std::time::Instant;
use std::env;

fn main() {
    let n: u64 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000_000);

    /* START */
    let start = Instant::now();
    let mut total: u64 = 0;
    for i in 0..n {
        total = black_box(total + i);
    }
    let duration = start.elapsed();
    /*  END  */

    let seconds = duration.as_secs_f64();
    let milliseconds = seconds * 1_000.0;
    let microseconds = seconds * 1_000_000.0;


    let (result, unit);
    if seconds >= 1.0  {
        result = seconds;
        unit = "s";
    } else if milliseconds >= 1.0  {
        result = milliseconds;
        unit = "ms";
    } else {
        result = microseconds;
        unit = "us";
    }

    println!("Rust: {:.1} {} ({})", result, unit, total);
}
