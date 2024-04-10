#include "../samplers/naive_sampler.h"
#include "../samplers/permutation_sampler.h"
#include "../samplers/priority_sampler.h"

#include <catch2/catch_test_macros.hpp>
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
