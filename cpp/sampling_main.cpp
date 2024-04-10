#include "samplers/naive_sampler.h"
#include "samplers/permutation_sampler.h"
#include "samplers/priority_sampler.h"

#include <chrono>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>
#include <vector>

#define N 500'000'000

enum SamplerType {
  Naive,
  SeqPriority,
  ParPriority,
  SeqPermutation,
  SeqPermutationFull,
  ParPermutation,
  ParPermutationFull,
  Unidentified,
};

SamplerType identify_sampler(std::string arg_type) {
  if (arg_type == "naive") {
    return Naive;
  } else if (arg_type == "seqpriority") {
    return SeqPriority;
  } else if (arg_type == "parpriority") {
    return ParPriority;
  } else if (arg_type == "seqperm") {
    return SeqPermutation;
  } else if (arg_type == "seqpermfull") {
    return SeqPermutationFull;
  } else if (arg_type == "parperm") {
    return ParPermutation;
  } else if (arg_type == "parpermfull") {
    return ParPermutationFull;
  }

  return Unidentified;
}

void benchmark(size_t num_threads, std::ofstream &wtr, SamplerType sampler_type,
               size_t k, size_t rep) {
  std::vector<int> data;
  data.reserve(N);
  for (size_t i = 0; i < N; i++) {
    data.push_back(rand());
  }
  std::string sampler_type_str;
  std::chrono::time_point<std::chrono::steady_clock> start, end;

  switch (sampler_type) {
  case Naive:
    start = std::chrono::steady_clock::now();
    NaiveSampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "Naive";
    break;
  case SeqPriority:
    start = std::chrono::steady_clock::now();
    SeqPrioritySampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "SeqPriority";
    break;
  case ParPriority:
    start = std::chrono::steady_clock::now();
    ParPrioritySampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "ParPriority";
    break;
  case SeqPermutation:
    start = std::chrono::steady_clock::now();
    SeqPermutationSampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "SeqPermutation";
    break;
  case SeqPermutationFull:
    start = std::chrono::steady_clock::now();
    SeqPermutationFullSampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "SeqPermutationFull";
    break;
  case ParPermutation:
    start = std::chrono::steady_clock::now();
    ParPermutationSampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "ParPermutation";
    break;
  case ParPermutationFull:
    start = std::chrono::steady_clock::now();
    ParPermutationFullSampler<int>::sample(data, k);
    end = std::chrono::steady_clock::now();
    sampler_type_str = "ParPermutationFull";
    break;
  default:
    return;
  }
  auto diff =
      std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
  if (rep > 0) {
    wtr << sampler_type_str << "," << k << "," << num_threads << ","
        << diff.count() << "\n";
  }
}

int main(int argc, char *argv[]) {
  if (argc != 6) {
    std::cerr << "Usage: " << argv[0]
              << " <num_threads> <output_file_name> <sampler_type> <k> <rep>"
              << std::endl;
    return 1;
  }

  size_t num_threads = atoi(argv[1]);
  std::ofstream results_file(std::string(argv[2]), std::ofstream::app);
  SamplerType sampler_type = identify_sampler(argv[3]);
  size_t k = atoi(argv[4]);
  size_t rep = atoi(argv[5]);

  benchmark(num_threads, results_file, sampler_type, k, rep);
  return 0;
}
