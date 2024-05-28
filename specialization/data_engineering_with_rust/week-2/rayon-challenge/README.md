# rayon-challenge

- Added timers;

- Tried with different vector sizes;

- Added sequential sum for comparison:
    - In this case, using Rayon for parallelization makes it slower (almost no work done by each thread, so the cost of locking is bigger).