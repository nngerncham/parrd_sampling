#include "samplers/naive_sampler.h"
#include "samplers/permutation_sampler.h"
#include "samplers/priority_sampler.h"

#include <chrono>
#include <cmath>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>
#include <vector>

#define N 100'000'000
#define REPS 5

void benchmark(std::ofstream &wtr, size_t num_threads) {
  // prepping all the ks
  size_t sqrt_n = sqrt((size_t)N);
  size_t c = 1;
  std::vector<size_t> ks;
  std::vector<size_t> k_percent = {10, 25, 50, 75, 90};
  while (c * log2(N) < sqrt_n) {
    ks.push_back(c * log2(N));
    c += 50;
  }
  c = 1;
  while (c * sqrt_n < N / 100 * 10) {
    ks.push_back(c * sqrt_n);
    c += 50;
  }
  for (auto k : k_percent) {
    ks.push_back(N / 100 * k);
  }

  for (auto k : ks) {
    std::cout << "\nBenchmarking with k = " << k << std::endl;

    for (size_t repeat = 0; repeat < REPS; repeat++) {
      std::vector<int> data;
      data.reserve(N);
      for (size_t i = 0; i < N; i++) {
        data.push_back(rand());
      }

      std::cout << "Naive " << k << std::endl;
      auto start_naive = std::chrono::high_resolution_clock::now();
      NaiveSampler<int>::sample(data, k);
      auto end_naive = std::chrono::high_resolution_clock::now();
      auto elapsed_naive =
          std::chrono::duration_cast<std::chrono::microseconds>(end_naive -
                                                                start_naive);
      wtr << "Naive," << k << "," << num_threads << ","
          << std::to_string(elapsed_naive.count()) << "," << repeat << "\n";

      std::cout << "SeqPriority " << k << std::endl;
      auto start_seq_priority = std::chrono::high_resolution_clock::now();
      SeqPrioritySampler<int>::sample(data, k);
      auto end_seq_priority = std::chrono::high_resolution_clock::now();
      auto elapsed_seq_priority =
          std::chrono::duration_cast<std::chrono::microseconds>(
              end_seq_priority - start_seq_priority);
      wtr << "SeqPriority," << k << "," << num_threads << ","
          << std::to_string(elapsed_seq_priority.count()) << "\n";

      std::cout << "ParPriority " << k << std::endl;
      auto start_par_priority = std::chrono::high_resolution_clock::now();
      ParPrioritySampler<int>::sample(data, k);
      auto end_par_priority = std::chrono::high_resolution_clock::now();
      auto elapsed_par_priority =
          std::chrono::duration_cast<std::chrono::microseconds>(
              end_par_priority - start_par_priority);
      wtr << "ParPriority," << k << "," << num_threads << ","
          << std::to_string(elapsed_par_priority.count()) << "," << repeat
          << "\n";

      std::cout << "SeqPermutation " << k << std::endl;
      auto start_seq_permutation = std::chrono::high_resolution_clock::now();
      SeqPermutationSampler<int>::sample(data, k);
      auto end_seq_permutation = std::chrono::high_resolution_clock::now();
      auto elapsed_seq_permutation =
          std::chrono::duration_cast<std::chrono::microseconds>(
              end_seq_permutation - start_seq_permutation);
      wtr << "SeqPermutation," << k << "," << num_threads << ","
          << std::to_string(elapsed_seq_permutation.count()) << "," << repeat
          << "\n";

      std::cout << "ParPermutation " << k << std::endl;
      auto start_par_permutation = std::chrono::high_resolution_clock::now();
      ParPermutationSampler<int>::sample(data, k);
      auto end_par_permutation = std::chrono::high_resolution_clock::now();
      auto elapsed_par_permutation =
          std::chrono::duration_cast<std::chrono::microseconds>(
              end_par_permutation - start_par_permutation);
      wtr << "ParPermutation," << k << "," << num_threads << ","
          << std::to_string(elapsed_par_permutation.count()) << "," << repeat
          << "\n";

      std::cout << "ParPermutationFull " << k << std::endl;
      auto start_par_permutation_full =
          std::chrono::high_resolution_clock::now();
      ParPermutationFullSampler<int>::sample(data, k);
      auto end_par_permutation_full = std::chrono::high_resolution_clock::now();
      auto elapsed_par_permutation_full =
          std::chrono::duration_cast<std::chrono::microseconds>(
              end_par_permutation_full - start_par_permutation_full);
      wtr << "ParPermutationFull," << k << "," << num_threads << ","
          << std::to_string(elapsed_par_permutation_full.count()) << ","
          << repeat << "\n";
    }
  }
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    std::cerr << "Usage: " << argv[0] << " <num_threads> <output_file_name>"
              << std::endl;
    return 1;
  }

  std::ofstream results_file;
  std::string file_name = "analysis/" + std::string(argv[2]);
  results_file.open(file_name);
  results_file << "algo,k,threads,time(ms),rep\n";
  size_t num_threads = atoi(argv[1]);

  benchmark(results_file, num_threads);

  results_file.close();
  return 0;
}
