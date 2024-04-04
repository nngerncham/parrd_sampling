#!/bin/bash

cmake -S . -B build/
cmake --build build/
./build/benchmark/g_benchmark \
	--benchmark_out_format=json \
	--benchmark_out=benchmark_results.json \
	benchmark_time_unit=ms
