#!/bin/bash

cmake --build build
echo "Running ParRandomSampling benchmark with 1 thread"
export PARLAY_NUM_THREADS=1
./build/ParRandomSampling 1 my_bench_results_1_650M.csv
echo "Running ParRandomSampling benchmark with 12 threads"
export PARLAY_NUM_THREADS=12
./build/ParRandomSampling 12 my_bench_results_12_650M.csv
echo "Running ParRandomSampling benchmark with 24 threads"
export PARLAY_NUM_THREADS=24
./build/ParRandomSampling 24 my_bench_results_24_650M.csv
