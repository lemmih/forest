// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use once_cell::sync::Lazy;
use prometheus::{core::Opts, Histogram, HistogramOpts};

pub static APPLY_BLOCKS_TIME: Lazy<Box<Histogram>> = Lazy::new(|| {
    let apply_blocks_time = Box::new(
        Histogram::with_opts(HistogramOpts {
            common_opts: Opts::new(
                "apply_blocks_time",
                "Duration of routine which applied blocks",
            ),
            buckets: vec![],
        })
        .expect("Defining the apply_blocks_time metric must succeed"),
    );
    prometheus::default_registry()
        .register(apply_blocks_time.clone())
        .expect("Registering the apply_blocks_time metric with the metrics registry must succeed");
    apply_blocks_time
});
