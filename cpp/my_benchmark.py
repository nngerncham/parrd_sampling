#! python

import subprocess
import os

REPS = 10
N = 500_000_000
THREADS = [1, 12, 24]
SAMPLERS = [
    "naive",
    "seqpriority",
    "parpriority",
    "seqperm",
    "seqpermfull",
    "parperm",
    "parpermfull",
]

if __name__ == "__main__":
    os.system("cmake --build build")

    for sampler_type in SAMPLERS:
        for num_threads in THREADS:
            print(f"\nRunning with {num_threads} threads")
            file_name = f"analysis/new_bench_results_{num_threads}_500M.csv"
            if not os.path.exists(file_name):
                # only write csv header if file does not exist
                with open(file_name, "w") as f:
                    f.write("algo,k,num_threads,time\n")

            k = 10_000
            delta = 10_000
            while k < N // 100 * 10:
                print(f"Running with k = {k}")
                os.environ["PARLAY_NUM_THREADS"] = str(num_threads)
                for rep in range(REPS + 1):
                    process = subprocess.Popen(
                        [
                            "./build/ParRandomSampling",
                            str(num_threads),
                            file_name,
                            sampler_type,
                            str(k),
                            str(rep),
                        ],
                    )
                    process.wait()

                k += delta
                if k >= delta * 10:
                    delta *= 10
