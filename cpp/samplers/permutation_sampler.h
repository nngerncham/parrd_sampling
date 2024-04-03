#include "sampler_model.h"
#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <utility>
#include <vector>

template <typename DataType> class SeqPermutationSampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    srand(time(NULL));
    std::vector<size_t> swap_targets;
    swap_targets.reserve(data.size());
    for (size_t i = 0; i < data.size(); i++) {
      size_t unshifted_index = rand() % (data.size() - i);
      swap_targets.push_back(unshifted_index + i);
    }

    std::vector<DataType> ans(data);
    permute(ans, k, swap_targets);
    ans.resize(k);
    return ans;
  }

private:
  void static permute(std::vector<DataType> &arr, size_t iters,
                      std::vector<size_t> swap_target) {
    for (size_t i = 0; i < iters; i++) {
      std::swap(arr[i], arr[swap_target[i]]);
    }
  }
};

template <typename DataType> class ParPermutationSampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    return std::vector<DataType>();
  }
};
