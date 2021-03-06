// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {LinkerFlavor, PanicStrategy};
use target::{TargetOptions, TargetResult, Target};
use std::default::Default;

pub fn target() -> TargetResult {
    Ok(Target {
           llvm_target: "nyuzi-elf-none".to_string(),
           target_endian: "little".to_string(),
           target_pointer_width: "32".to_string(),
           data_layout: "e-m:e-p:32:32".to_string(),
           arch: "nyuzi".to_string(),
           target_os: "none".to_string(),
           target_env: "none".to_string(),
           target_vendor: "unknown".to_string(),
           linker_flavor: LinkerFlavor::Gcc,
           options: TargetOptions {
               cpu: "nyuzi".to_string(),
               target_family: Some("none".to_string()),
               panic_strategy: PanicStrategy::Abort,
               spmd: true,
               ..Default::default()
           },
       })
}
