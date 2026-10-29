# Root Makefile for sys1own/shbt-swarm
# Builds & verifies the C11 microkernel, Rust crates, 38 Z3 SMT theorems, 144 HIL gates, and LaTeX PDF.

PYTHON ?= python3
CC     ?= gcc
CFLAGS := -std=c11 -Wall -Wextra -Werror -O2 -Iinclude -Ikernel/include

.PHONY: all c11-test rust-test z3-proofs audit pdf clean

all: c11-test z3-proofs rust-test audit

c11-test:
	$(MAKE) -C kernel test

z3-proofs:
	$(PYTHON) formal/verify_swarm_cloning.py
	$(PYTHON) formal/verify_swarm_ai_safety.py
	$(PYTHON) formal/verify_universal_synthesis.py
	$(PYTHON) formal/verify_mar312x_reactor.py
	$(PYTHON) formal/verify_omega_megastructures.py
	$(PYTHON) formal/verify_logos_hypercomputer.py

rust-test:
	cargo check --workspace --all-targets
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace

audit:
	cargo run --release -p shbt-swarm-hil --bin swarm-audit

pdf: audit
	cd docs && pdflatex -interaction=nonstopmode -halt-on-error swarm.tex && pdflatex -interaction=nonstopmode -halt-on-error swarm.tex
	cp docs/swarm.pdf swarm.pdf

clean:
	$(MAKE) -C kernel clean
	cargo clean
