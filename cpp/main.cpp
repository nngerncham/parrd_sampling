#include "samplers/naive_sampler.h"
#include "samplers/permutation_sampler.h"
#include "samplers/priority_sampler.h"

#include <iostream>
#include <vector>

int main(int argc, char *argv[]) {
  std::vector<int> data = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

  std::vector<int> naive_sample = NaiveSampler<int>::sample(data, 5);
  for (auto e : naive_sample) {
    std::cout << e << " ";
  }
  std::cout << "\n";

  std::vector<int> priority_sample = SeqPrioritySampler<int>::sample(data, 5);
  for (auto e : priority_sample) {
    std::cout << e << " ";
  }
  std::cout << "\n";

  std::vector<int> par_priority_sample =
      ParPrioritySampler<int>::sample(data, 5);
  for (auto e : par_priority_sample) {
    std::cout << e << " ";
  }
  std::cout << "\n";

  std::vector<int> permutation_sample =
      SeqPermutationSampler<int>::sample(data, 5);
  for (auto e : permutation_sample) {
    std::cout << e << " ";
  }
  std::cout << "\n";

  return 0;
}
