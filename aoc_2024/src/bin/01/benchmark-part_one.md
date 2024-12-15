| Command       |      Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------ | -------------: | -------: | -------: | ----------: |
| `solutions`   |   919.5 ± 49.8 |    843.0 |    984.2 | 1.00 ± 0.15 |
| `alternative` |  917.1 ± 132.0 |    736.5 |   1230.2 |        1.00 |
| `nom`         |   937.5 ± 49.5 |    860.8 |   1026.8 | 1.02 ± 0.16 |
| `nom_iter`    | 1027.0 ± 138.8 |    809.1 |   1228.5 | 1.12 ± 0.22 |

This benchmark was executed on a MacBook Pro (13-inch, 2020) with a Apple M1 processor and 16 GB memory. The operating system was macOS Sonoma version 14.6.1.

The difference between the `solutions` and `alternative` is not significant, so we can conclude that both solutions are equally efficient. The `nom` and `nom_iter` solutions are slightly slower, but the difference is not significant either. Therefore, we can say that all four solutions have similar performance.
