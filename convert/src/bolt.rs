// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// BOLT pre-aggregated profile emitter.
//
// Format:
//   E cycles
//   B <from_offset_hex> <to_offset_hex> <count> <mispreds>
//   F <from_offset_hex> <to_offset_hex> <count>

use std::fs::File;
use std::io::{BufWriter, Write};

use crate::elf_index::ElfIndex;
use crate::native::Profile;

pub fn emit(profile: &Profile, elf: &ElfIndex, output: &str) -> Result<(), String> {
    let file = File::create(output).map_err(|e| format!("create {}: {}", output, e))?;
    let mut w = BufWriter::new(file);

    // The load address from the profile header is the runtime base.
    // BOLT expects offsets from the binary's load address.
    let base = if profile.header.load_addr != 0 {
        profile.header.load_addr
    } else {
        elf.load_addr
    };

    // Event type header
    writeln!(w, "E cycles").map_err(|e| format!("write: {}", e))?;

    // Branch edges → B records
    for edge in &profile.branch_edges {
        let from_off = edge.from.wrapping_sub(base);
        let to_off = edge.to.wrapping_sub(base);
        // Misprediction count = 0 (QEMU doesn't model prediction)
        writeln!(w, "B {:x} {:x} {} 0", from_off, to_off, edge.count)
            .map_err(|e| format!("write: {}", e))?;
    }

    // Fall-through edges → F records
    for edge in &profile.fall_through_edges {
        let from_off = edge.from.wrapping_sub(base);
        let to_off = edge.to.wrapping_sub(base);
        writeln!(w, "F {:x} {:x} {}", from_off, to_off, edge.count)
            .map_err(|e| format!("write: {}", e))?;
    }

    w.flush().map_err(|e| format!("flush: {}", e))?;

    let total = profile.branch_edges.len() + profile.fall_through_edges.len();
    eprintln!(
        "bolt: wrote {} records ({} B + {} F) to {}",
        total,
        profile.branch_edges.len(),
        profile.fall_through_edges.len(),
        output
    );

    Ok(())
}
