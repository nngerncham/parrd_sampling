#include "sampler_model.h"
#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <unordered_set>
#include <vector>

template <typename DataType> class NaiveSampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    size_t n = data.size();
    std::unordered_set<size_t> picked_indexes;
    std::vector<DataType> sample;
    sample.reserve(k);

    srand(time(NULL));
    while (sample.size() < k) {
      size_t target_idx = random() % n;
      if (picked_indexes.find(target_idx) == picked_indexes.end()) {
        picked_indexes.insert(target_idx);
        sample.push_back(data[target_idx]);
      }
    }

    return sample;
  }
};
