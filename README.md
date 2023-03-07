# Ray Tracing

A Re-implementation of [_Ray Tracing in One Weekend Series_](https://raytracing.github.io) written in Rust.

Extra features

- Parallel rendering using thread pool

## Run

```shell
cargo run [OPTIONS]

# Options:
#       --bin [<NAME>]            Name of the bin target to run
#       --example [<NAME>]        Name of the example target to run
#   -p, --package [<SPEC>]        Package with the target to run
#   -r, --release                 Build artifacts in release mode, with optimizations

# e.g. 'cargo run --release --bin in_one_weekend', 'cargo run --release --example 8-2_standard_cornell_box_scene'
```

- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
  ![Ray Tracing in One Weekend Final Render SPP1024](doc/assets/RayTracingInOneWeekendFinalRenderSPP1024.png)

- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
  ![Ray Tracing: The Next Week Final Render SPP10240](doc/assets/RayTracingTheNextWeekSPP10240.png)

- [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
  Working in progress
