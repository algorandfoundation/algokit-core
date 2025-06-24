#!/bin/bash

set -ex

cargo fmt
cargo clippy --fix --allow-dirty
