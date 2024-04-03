#include "../parlay/primitives.h"
#include "../parlay/random.h"
#include "sampler_model.h"
#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <random>
#include <utility>
#include <vector>

template <typename DataType> class SeqPrioritySampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    time_t seed = time(NULL);
    srand(seed);
    std::vector<int> priority;
    priority.reserve(data.size());
    for (size_t i = 0; i < data.size(); i++) {
      priority.push_back(rand());
    }

    std::nth_element(priority.begin(), priority.begin() + k, priority.end());
    int kth_priority = priority[k];

    // "regenerates" the priorities again so need to reset RNG
    srand(seed);
    std::vector<DataType> sample;
    for (size_t i = 0; i < data.size() && sample.size() < k; i++) {
      if (rand() <= kth_priority) {
        sample.push_back(data[i]);
      }
    }

    return sample;
  }
};

template <typename DataType> class ParPrioritySampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    parlay::random_generator gen(time(NULL));
    std::uniform_int_distribution<int> dis;
    parlay::sequence<std::pair<DataType, int>> priority =
        parlay::tabulate(data.size(), [&](size_t i) {
          auto r = gen[i];
          return std::make_pair(data[i], dis(r));
        });

    auto kth_element = parlay::kth_smallest(
        priority, k, [&](auto a, auto b) { return a.second <= b.second; });
    auto leq = parlay::filter(
        priority, [&](auto elm) { return elm.second <= kth_element->second; });

    std::vector<DataType> sample;
    sample.reserve(k);
    for (size_t i = 0; i < k; i++) {
      sample.push_back(leq[i].first);
    }
    return sample;
  }
};
