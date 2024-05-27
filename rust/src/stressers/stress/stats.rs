use std::ops::Sub;
use std::time::{Duration, Instant};

use hdrhistogram::Histogram;
use log::*;

pub struct Stats {
    hist: Histogram<u64>,

    ts: Instant,
    total: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            hist: Histogram::new_with_bounds(1, 1000, 2).unwrap(),
            ts: Instant::now(),
            total: 0,
        }
    }

    pub fn record(&mut self, ts_start: Instant) {
        let duration = ts_start.elapsed();

        self.hist
            .record(duration.as_millis() as u64)
            .map_err(|err| {
                error!(
                    "Invalid record value {}: {:?}",
                    humantime::Duration::from(duration),
                    err
                )
            })
            .ok();
    }

    pub fn print(&mut self, prefix: &str) {
        let t = Instant::now();
        if t.sub(self.ts) < Duration::from_secs(1) {
            return;
        }

        let len = self.hist.len();
        let cd = self
            .hist
            .iter_quantiles(1)
            .enumerate()
            .filter_map(|(i, val)| match i {
                4 | 6 | 14 => Some((
                    val.percentile(),
                    humantime::Duration::from(Duration::from_millis(val.value_iterated_to())),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();

        if cd.len() == 3 {
            info!(
                "{} total = {}, DT = {}, mean/stddev {:.2}/{:.2} {:.2}%[{}]/{:.2}%[{}]/{:.2}%[{}]",
                prefix,
                len,
                len - self.total,
                self.hist.mean(),
                self.hist.stdev(),
                cd[0].0,
                cd[0].1,
                cd[1].0,
                cd[1].1,
                cd[2].0,
                cd[2].1,
            );
        } else {
            info!(
                "{} total = {}, DT = {}, mean/stddev {:.2}/{:.2}",
                prefix,
                len,
                len - self.total,
                self.hist.mean(),
                self.hist.stdev(),
            );
        }

        self.ts = t;
        self.total = len;
    }
}
