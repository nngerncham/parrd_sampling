#include <vector>

template <typename DataType> class Sampler {
public:
  std::vector<DataType> static sample(std::vector<DataType> data,
                                      size_t sample_size) {
    return std::vector<DataType>();
  }
};
