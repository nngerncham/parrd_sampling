#!/bin/bash

cmake --build build
./build/ParRandomSampling 1 my_bench_results_1_100M.csv
./build/ParRandomSampling 12 my_bench_results_12_100M.csv
./build/ParRandomSampling 24 my_bench_results_24_100M.csv
