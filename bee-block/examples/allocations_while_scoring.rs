// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example aims at counting the number of allocations that are performed when we score the Proof of Work of a
//! block by wrapping `GlobalAlloc`. Ideally, this method should not allocate at all, which would lead to a better
//! performance.
//!
//! The code was adapted from: https://kanejaku.org/posts/2021/01/2021-01-27/ (CC-BY 4.0)

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

use bee_block::{protocol::protocol_parameters, rand::parents::rand_parents, BlockBuilder};
use bee_pow::{
    providers::{miner::MinerBuilder, NonceProviderBuilder},
    score::PoWScorer,
};
use packable::PackableExt;

struct CheckAlloc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CheckAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(1, SeqCst);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static A: CheckAlloc = CheckAlloc;

fn main() {
    let block = BlockBuilder::new(rand_parents())
        .with_nonce_provider(MinerBuilder::new().with_num_workers(num_cpus::get()).finish())
        .finish(protocol_parameters().min_pow_score())
        .unwrap();

    let block_bytes = block.pack_to_vec();

    let before_count = ALLOCATED.load(SeqCst);
    PoWScorer::new().score(&block_bytes);
    let after_count = ALLOCATED.load(SeqCst);

    println!("Number of allocations: {}", after_count - before_count);
}
