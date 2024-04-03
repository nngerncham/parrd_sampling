#include "parlay/primitives.h"
#include "parlay/sequence.h"

#include "samplers/sampler_model.cpp"

#include <iostream>
#include <vector>

int main(int argc, char *argv[]) {
  std::vector<int> data = {1, 2, 3, 4, 5};
  std::vector<int> sample = Sampler<int>::sample(data, 3);
  std::cout << sample.size() << "\n";
  return 0;
}
