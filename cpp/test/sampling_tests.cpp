#include "../samplers/naive_sampler.h"
#include "../samplers/permutation_sampler.h"
#include "../samplers/priority_sampler.h"

#include <catch2/catch_test_macros.hpp>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <vector>

TEST_CASE("Naive Sampler", "[sampler]") {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  std::vector<int> sample = NaiveSampler<int>::sample(data, 3);
  REQUIRE(sample.size() == 3);
}

TEST_CASE("Sequential Priority Sampler", "[priority_sampler]") {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  std::vector<int> sample = SeqPrioritySampler<int>::sample(data, 3);
  REQUIRE(sample.size() == 3);
}

TEST_CASE("Parallel Priority Sampler", "[priority_sampler]") {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  std::vector<int> sample = ParPrioritySampler<int>::sample(data, 3);
  REQUIRE(sample.size() == 3);
}

TEST_CASE("Sequential Permutation Sampler", "[permutation_sampler]") {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  std::vector<int> sample = SeqPermutationSampler<int>::sample(data, 3);
  REQUIRE(sample.size() == 3);
}

TEST_CASE("Parallel Permutation Sampler", "[permutation_sampler]") {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  std::vector<int> sample = ParPermutationSampler<int>::sample(data, 3);
  REQUIRE(sample.size() == 3);
}

TEST_CASE("Parallel Permutation outputs the same as Sequential Permutation",
          "[permutation_sampler]") {
  srand(time(NULL));

  // generate data
  size_t n = 10'000;
  std::vector<int> data1;
  std::vector<int> data2;
  for (size_t i = 0; i < n; i++) {
    int x = rand();
    data1.push_back(x);
    data2.push_back(x);
  }

  // sequential swap targets
  size_t swap_target_size = 100;
  std::vector<size_t> swap_targets;
  parlay::sequence<size_t> swap_targets_par;

  std::random_device rd;
  std::mt19937 gen(rd());
  for (size_t i = 0; i < n; i++) {
    std::uniform_int_distribution<size_t> dis(i, n - 1);
    size_t target = dis(gen);

    swap_targets.push_back(target);
    swap_targets_par.push_back(target);
  }

  SeqPermutationSampler<int>::permute(data1, 100, swap_targets);
  ParPermutationSampler<int>::permute(data2, 100, swap_targets_par);

  REQUIRE(data1 == data2);
}
