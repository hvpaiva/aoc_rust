| Command       |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------ | ---------: | -------: | -------: | ----------: |
| `solution`    | 14.0 ± 5.4 |      8.1 |     43.3 | 1.09 ± 0.60 |
| `alternative` | 12.8 ± 5.0 |      7.9 |     40.5 |        1.00 |
| `nom`         | 13.2 ± 4.2 |      8.0 |     36.9 | 1.03 ± 0.52 |
| `nom_iter`    | 13.0 ± 3.1 |      7.9 |     30.5 | 1.02 ± 0.47 |

**Note:** I am not entirely confident in the accuracy of these benchmarks due to the high deviation observed. Currently, I’m using `hyperfine` to measure performance. The main reason I chose `hyperfine` is that it integrates easily into my existing project structure — it simply runs the entire binary as a command line invocation. This approach made it straightforward to test different solutions without having to refactor my code extensively. However, this also introduces some limitations.

For instance, `hyperfine` measures the runtime of the entire process, including startup time, potential I/O overhead, and any other environmental factors. I’ve tried mitigating these issues by running **100 warmups** and executing the benchmarks **200+ times**, but the variance remains noticeable. This variance can occur because `hyperfine` is not a microbenchmarking tool at the function-level; it cannot isolate the function I want to measure from the rest of the environment. System scheduling, CPU frequency scaling, and other external factors all contribute to the deviation in results.

In the future, I plan to incorporate more specialized benchmarking tools such as **Criterion** or **Divan**. These frameworks are designed for finer-grained, stable, and more statistically reliable measurements at the function-level, and I believe they will provide a more accurate assessment of performance once I have aligned them with my project’s structure. Until then, please interpret these results with caution.
