#include "sampler_model.h"
#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <vector>

template <typename DataType> class SeqPrioritySampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    srand(time(NULL));
    std::vector<size_t> priority;
    priority.reserve(data.size());
    for (int i = 0; i < data.size(); i++) {
      priority.push_back(rand());
    }
    std::vector<size_t> search_priority(priority);

    std::nth_element(search_priority.begin(), search_priority.begin() + k,
                     search_priority.end());
    size_t kth_priority = search_priority[k];

    std::vector<DataType> sample;
    for (int i = 0; i < data.size() && sample.size() < k; i++) {
      if (priority[i] <= kth_priority) {
        sample.push_back(data[i]);
      }
    }

    return sample;
  }
};
