#include "../parlay/parallel.h"
#include "../parlay/primitives.h"
#include "../parlay/sequence.h"

#include "sampler_model.h"

#include <algorithm>
#include <atomic>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <random>
#include <stdatomic.h>
#include <utility>
#include <vector>

#define PREFIX_DIVISOR 50

template <typename DataType> class SeqPermutationSampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    std::random_device rd;
    std::mt19937 gen(rd());

    std::vector<size_t> swap_targets;
    swap_targets.reserve(data.size());
    for (size_t i = 0; i < data.size(); i++) {
      std::uniform_int_distribution<size_t> dis(i, data.size() - 1);
      swap_targets.push_back(dis(gen));
    }

    std::vector<DataType> ans(data);
    permute(ans, k, swap_targets);
    ans.resize(k);
    return ans;
  }

  void static permute(std::vector<DataType> &arr, size_t iters,
                      std::vector<size_t> &swap_target) {
    for (size_t i = 0; i < iters; i++) {
      std::swap(arr[i], arr[swap_target[i]]);
    }
  }
};

template <typename DataType> class ParPermutationSampler : Sampler<DataType> {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    size_t n = data.size();

    // generates swap targets
    parlay::random_generator gen(time(NULL));
    parlay::sequence<size_t> swap_target =
        parlay::tabulate(data.size(), [&](size_t i) {
          std::uniform_int_distribution<size_t> dis(i, n - 1);
          auto r = gen[i];
          return dis(r);
        });

    // reserve and commit
    std::vector<DataType> ans(data);
    permute(ans, k, swap_target);
    ans.resize(k);
    return ans;
  }

  void static permute(std::vector<DataType> &ans, size_t k,
                      parlay::sequence<size_t> &swap_target) {
    size_t n = ans.size();
    parlay::sequence<std::atomic_size_t> reservations =
        parlay::tabulate(n, [&](size_t i) { return std::atomic_size_t(n); });
    auto reserve = [&](size_t i) {
      fetch_min(reservations[i], i);
      fetch_min(reservations[swap_target[i]], i);
    };
    auto commit = [&](size_t i) {
      size_t swap_idx = swap_target[i];
      if (reservations[i].load() == i && reservations[swap_idx].load() == i) {
        std::swap(ans[i], ans[swap_idx]);
        return false;
      } else {
        return true;
      }
    };

    parlay::sequence<size_t> idx_left =
        parlay::tabulate(k, [&](size_t i) { return i; });
    size_t prefix_size =
        std::max<size_t>(idx_left.size() / PREFIX_DIVISOR, PREFIX_DIVISOR);

    while (!idx_left.empty()) {
      // reserve phase
      size_t to_reserve_size = std::min<size_t>(idx_left.size(), prefix_size);
      parlay::parallel_for(0, to_reserve_size,
                           [&](size_t i) { reserve(idx_left[i]); });

      // commit + repacking phase
      parlay::sequence<size_t> new_idx_left =
          parlay::filter(idx_left.head(to_reserve_size),
                         [&](size_t idx) { return commit(idx); });
      if (idx_left.size() > to_reserve_size) { // prefix shorter # reserved
        new_idx_left.append(idx_left.tail(idx_left.size() - to_reserve_size));
      }

      // update reservations for next round, required!!
      parlay::parallel_for(0, to_reserve_size, [&](size_t i) {
        size_t idx = idx_left[i];
        reservations[swap_target[idx]].store(n);
      });

      idx_left = new_idx_left;
      prefix_size =
          std::max<size_t>(idx_left.size() / PREFIX_DIVISOR, PREFIX_DIVISOR);
    }
  }

private:
  void static fetch_min(std::atomic_size_t &var, size_t new_value) {
    size_t current_value = var.load();
    while (new_value < current_value &&
           !var.compare_exchange_weak(current_value, new_value)) {
    }
  }
};
