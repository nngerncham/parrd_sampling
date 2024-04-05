#!/bin/bash

cmake --build build
export PARLAY_NUM_THREADS=1
./build/ParRandomSampling 1 my_bench_results_1_100M.csv
export PARLAY_NUM_THREADS=12
./build/ParRandomSampling 12 my_bench_results_12_100M.csv
export PARLAY_NUM_THREADS=24
./build/ParRandomSampling 24 my_bench_results_24_100M.csv
