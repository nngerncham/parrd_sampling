#include "../samplers/naive_sampler.h"
#include "../samplers/permutation_sampler.h"
#include "../samplers/priority_sampler.h"

#include <benchmark/benchmark.h>
#include <cstddef>
#include <cstdlib>
#include <vector>

#define PROBLEM_SIZE 30'000'000
#define K10 (PROBLEM_SIZE / 100 * 10)
#define K25 (PROBLEM_SIZE / 100 * 25)
#define K50 (PROBLEM_SIZE / 100 * 50)
#define K75 (PROBLEM_SIZE / 100 * 75)
#define K90 (PROBLEM_SIZE / 100 * 90)

static std::vector<int> generate_data() {
  std::vector<int> data;
  data.reserve(PROBLEM_SIZE);
  for (size_t i = 0; i < PROBLEM_SIZE; i++) {
    data.push_back(rand());
  }
  return data;
}

static void DoSetup(const benchmark::State &state) {}

static void DoTeardown(const benchmark::State &state) {}

// =========== NAIVE SAMPLER ============
static void BM_naive_sampler_k10(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    NaiveSampler<int>::sample(data, K10);
  }
}

static void BM_naive_sampler_k25(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    NaiveSampler<int>::sample(data, K25);
  }
}

static void BM_naive_sampler_k50(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    NaiveSampler<int>::sample(data, K50);
  }
}

static void BM_naive_sampler_k75(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    NaiveSampler<int>::sample(data, K75);
  }
}

static void BM_naive_sampler_k90(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    NaiveSampler<int>::sample(data, K90);
  }
}

BENCHMARK(BM_naive_sampler_k10)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_naive_sampler_k25)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_naive_sampler_k50)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_naive_sampler_k75)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_naive_sampler_k90)->Threads(1)->Threads(12)->Threads(24);

// =========== SEQ PRIORITY SAMPLER ============
static void BM_seq_priority_sampler_k10(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPrioritySampler<int>::sample(data, K10);
  }
}

static void BM_seq_priority_sampler_k25(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPrioritySampler<int>::sample(data, K25);
  }
}

static void BM_seq_priority_sampler_k50(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPrioritySampler<int>::sample(data, K50);
  }
}

static void BM_seq_priority_sampler_k75(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPrioritySampler<int>::sample(data, K75);
  }
}

static void BM_seq_priority_sampler_k90(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPrioritySampler<int>::sample(data, K90);
  }
}

BENCHMARK(BM_seq_priority_sampler_k10)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_priority_sampler_k25)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_priority_sampler_k50)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_priority_sampler_k75)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_priority_sampler_k90)->Threads(1)->Threads(12)->Threads(24);

// =========== PAR PRIORITY SAMPLER ============
static void BM_par_priority_sampler_k10(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPrioritySampler<int>::sample(data, K10);
  }
}

static void BM_par_priority_sampler_k25(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPrioritySampler<int>::sample(data, K25);
  }
}

static void BM_par_priority_sampler_k50(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPrioritySampler<int>::sample(data, K50);
  }
}

static void BM_par_priority_sampler_k75(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPrioritySampler<int>::sample(data, K75);
  }
}

static void BM_par_priority_sampler_k90(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPrioritySampler<int>::sample(data, K90);
  }
}

BENCHMARK(BM_par_priority_sampler_k10)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_priority_sampler_k25)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_priority_sampler_k50)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_priority_sampler_k75)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_priority_sampler_k90)->Threads(1)->Threads(12)->Threads(24);

// =========== SEQ PERMUTATION SAMPLER ============
static void BM_seq_permutation_sampler_k10(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPermutationSampler<int>::sample(data, K10);
  }
}

static void BM_seq_permutation_sampler_k25(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPermutationSampler<int>::sample(data, K25);
  }
}

static void BM_seq_permutation_sampler_k50(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPermutationSampler<int>::sample(data, K50);
  }
}

static void BM_seq_permutation_sampler_k75(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPermutationSampler<int>::sample(data, K75);
  }
}

static void BM_seq_permutation_sampler_k90(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    SeqPermutationSampler<int>::sample(data, K90);
  }
}

BENCHMARK(BM_seq_permutation_sampler_k10)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_permutation_sampler_k25)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_permutation_sampler_k50)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_permutation_sampler_k75)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_seq_permutation_sampler_k90)->Threads(1)->Threads(12)->Threads(24);

// =========== PAR PERMUTATION SAMPLER ============
static void BM_par_permutation_sampler_k10(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPermutationSampler<int>::sample(data, K10);
  }
}

static void BM_par_permutation_sampler_k25(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPermutationSampler<int>::sample(data, K25);
  }
}

static void BM_par_permutation_sampler_k50(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPermutationSampler<int>::sample(data, K50);
  }
}

static void BM_par_permutation_sampler_k75(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPermutationSampler<int>::sample(data, K75);
  }
}

static void BM_par_permutation_sampler_k90(benchmark::State &state) {
  auto data = generate_data();
  for (auto _ : state) {
    ParPermutationSampler<int>::sample(data, K90);
  }
}

BENCHMARK(BM_par_permutation_sampler_k10)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_permutation_sampler_k25)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_permutation_sampler_k50)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_permutation_sampler_k75)->Threads(1)->Threads(12)->Threads(24);
BENCHMARK(BM_par_permutation_sampler_k90)->Threads(1)->Threads(12)->Threads(24);

BENCHMARK_MAIN();
