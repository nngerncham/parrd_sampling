#include "samplers/naive_sampler.h"

#include <iostream>
#include <vector>

int main(int argc, char *argv[]) {
  std::vector<int> data = {1, 2, 3, 4, 5};
  std::vector<int> naive_sample = NaiveSampler<int>::sample(data, 3);
  for (auto e : naive_sample) {
    std::cout << e << " ";
  }
  std::cout << "\n";

  return 0;
}
