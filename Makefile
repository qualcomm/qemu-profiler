# SPDX-License-Identifier: BSD-3-Clause-Clear
# Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
#
# Builds two plugin variants:
#   libtcg_prof_2.so — tiers 0-2, API v2, QEMU 9.0+
#   libtcg_prof_6.so — all tiers, API v6, QEMU 11.0+

.PHONY: all clean

all:
	cargo build --release -p qemu-pgo-plugin --features tier3
	cp target/release/libtcg_prof.so target/release/libtcg_prof_6.so
	cargo build --release -p qemu-pgo-plugin
	cp target/release/libtcg_prof.so target/release/libtcg_prof_2.so
	@echo "Built target/release/libtcg_prof_2.so (API v2, tiers 0-2)"
	@echo "Built target/release/libtcg_prof_6.so (API v6, all tiers)"

clean:
	cargo clean
