# fuzzy-logic_rs

This is the documentation for fuzzy logic crate.

## Installation

```bash
    cargo add fuzzy-logic_rs
```

## Project layout

    src/
        lib.rs
        aggregations.rs
        defuzzification.rs
        fuzzy_inference_systems.rs
        implications.rs
        membership_functions.rs
        membership_ranges.rs
        rules.rs
        s_norms.rs
        t_norms.rs
        variables.rs
    examples/
        function_approximation.rs
        speed_control.rs
        tipper.rs
    Cargo.toml
    .gitignore
    readme.md
    LICENSE
    -- mkdocs files

## Inference Systems

* [x] Mamdani Type 1
* [ ] Mamdani Type 2
* [x] TSK Type 1
* [ ] TSK Type 2

## Future Plans

* [ ] Import and Export systems to and from a file
* [ ] Add plot support
* [ ] add meta-heuristics
* [ ] add ANFIS support
* [ ] add dedicated control module
* [ ] add fuzzy c-means
