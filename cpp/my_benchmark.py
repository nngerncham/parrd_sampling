#! python

import os

REPS = 3
N = 500_000_000
THREADS = [1, 12, 24]
SAMPLERS = [
    "naive",
    "seqpriority",
    "parpriority",
    "seqperm",
    "seqpermcopy",
    "seqpermfull",
    "parperm",
    "parpermfull",
]

if __name__ == "__main__":
    for sampler_type in SAMPLERS:
        print(f"\nRunning with {sampler_type} sampler")

        for num_threads in THREADS:
            print(f"\nRunning with {num_threads} threads")

            os.environ["PARLAY_NUM_THREADS"] = str(num_threads)
            file_name = f"analysis/new_bench_results_{num_threads}_500M.csv"
            if not os.path.exists(file_name):
                # only write csv header if file does not exist
                with open(file_name, "w") as f:
                    f.write("algo,k,num_threads,time\n")

            flag = False
            k = 25_000
            delta = 25_000
            while k <= N // 100 * 10:
                print(f"Running with k = {k}")

                print("Running repeat ", end="", flush=True)
                for rep in range(REPS + 1):
                    print(f"{rep} ", end="", flush=True)
                    os.system(
                        " ".join(
                            [
                                "./build/ParRandomSampling",
                                str(num_threads),
                                file_name,
                                sampler_type,
                                str(k),
                                str(rep),
                            ]
                        )
                    )

                k += delta
                if k >= delta * 4:
                    delta *= 10
                    flag = True
                elif flag:
                    k = delta
                    flag = False
                print("")
