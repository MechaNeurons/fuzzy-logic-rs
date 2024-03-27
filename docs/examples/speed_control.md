# Overview

In This example we will solving a problem in which we want to control the acceleration of car.

## run the example

you can run the example with the following command.

```bash
cargo run --example speed-control
```

## Problem Definition

Control the acceleration of a vehicle using data from speedometer and a radar that measures the distance between the ego vehicle and the vehicle in the front.

### Import

We will import the necessary modules:

```rust
use fuzzy_logic_rs::{
    aggregations::Aggregations,
    defuzzifications::Defuzzifiers,
    fuzzy_inference_systems::MamdaniFIS,
    implications::Implications,
    membership_functions::{MFKind, Triangle, MF},
    membership_ranges::MembershipRange,
    rules::Rule,
    s_norms::SNorms,
    t_norms::TNorms,
    variables::{InputVariables, OutputVariables},
};
```

### Setup

We will using Mamdani FIS with the following configurations.

```rust
let mut fis = MamdaniFIS::new(
        SNorms::Max,
        TNorms::Min,
        Implications::Min,
        Aggregations::Max,
        Defuzzifiers::Bisection,
    );
```

### Inputs

As the problem stated we have to input that we can use in our calculation, speed and distance.
I'll assume the speed will vary in the range of [0 140] and distance in the range of [0 50].

!!!danger "Input range"
    please do not use the values outside of these ranges, in the current setup. change the membership that are also defined outside of this range like `LinearZ` and `LinearS` or a custom one.

I choose 3 triangular membership functions for each input.

speed: S - M - L

```rust
let mut v1 = InputVariables::new("speed".to_string(), (0.0, 140.0));
v1.add_membership(MF::new(
    "S".to_string(),
    MFKind::Triangle(Triangle::new(-58.3, 0.0, 58.3)),
));
v1.add_membership(MF::new(
    "M".to_string(),
    MFKind::Triangle(Triangle::new(11.67, 70.0, 128.3)),
));
v1.add_membership(MF::new(
    "L".to_string(),
    MFKind::Triangle(Triangle::new(81.67, 140.0, 198.3)),
));
fis.add_input(v1);
```

distance: S - M - L

```rust
let mut v2 = InputVariables::new("Distance".to_string(), (0.0, 50.0));
v2.add_membership(MF::new(
    "S".to_string(),
    MFKind::Triangle(Triangle::new(-20.83, 0.0, 20.83)),
));
v2.add_membership(MF::new(
    "M".to_string(),
    MFKind::Triangle(Triangle::new(4.168, 25.0, 45.82)),
));
v2.add_membership(MF::new(
    "L".to_string(),
    MFKind::Triangle(Triangle::new(29.17, 50.0, 70.82)),
));
fis.add_input(v2);
```

### Outputs

We have to control the acceleration so I will assume the accusation will change between [-1 1]. I choose 5 Gaussian membership function for acceleration.

acceleration: NB(negative big) - NS(negative small) - ZR(zero) - PS(positive small) - PB(positive big)

```rust
let mut o1 = OutputVariables::new(String::from("Acceleration"), (-1.0, 1.0), 100);
o1.add_membership(MembershipRange::new_gaussian(
    o1.get_universe(),
    "NB".to_string(),
    -1.0,
    0.2123,
));
o1.add_membership(MembershipRange::new_gaussian(
    o1.get_universe(),
    "NS".to_string(),
    -0.5,
    0.2123,
));
o1.add_membership(MembershipRange::new_gaussian(
    o1.get_universe(),
    "ZR".to_string(),
    0.0,
    0.2123,
));
o1.add_membership(MembershipRange::new_gaussian(
    o1.get_universe(),
    "PS".to_string(),
    0.5,
    0.2123,
));
o1.add_membership(MembershipRange::new_gaussian(
    o1.get_universe(),
    "PB".to_string(),
    1.0,
    0.2123,
));
fis.add_input(o1);
```

### Rules

We can define the rules as follows:

1. IF speed IS S AND distance IS S Then acceleration is ZR
2. IF speed IS S AND distance IS M Then acceleration is PS
3. IF speed IS S AND distance IS L Then acceleration is PB
4. IF speed IS M AND distance IS S Then acceleration is NS
5. IF speed IS M AND distance IS M Then acceleration is ZR
6. IF speed IS M AND distance IS L Then acceleration is PS
7. IF speed IS L AND distance IS S Then acceleration is NB
8. IF speed IS L AND distance IS M Then acceleration is NS
9. IF speed IS L AND distance IS L Then acceleration is ZR

We can define the rules:

```rust
fis.add_rule(Rule::new_and(vec![0, 0, 2], 1.0));
fis.add_rule(Rule::new_and(vec![0, 1, 3], 1.0));
fis.add_rule(Rule::new_and(vec![0, 2, 4], 1.0));

fis.add_rule(Rule::new_and(vec![1, 0, 1], 1.0));
fis.add_rule(Rule::new_and(vec![1, 1, 2], 1.0));
fis.add_rule(Rule::new_and(vec![1, 2, 3], 1.0));

fis.add_rule(Rule::new_and(vec![2, 0, 0], 1.0));
fis.add_rule(Rule::new_and(vec![2, 1, 1], 1.0));
fis.add_rule(Rule::new_and(vec![2, 2, 2], 1.0));
```

### Output

The problem formulation is done and we can use this to compute the output of the system:

```rust
let output = fis.compute_outputs(vec![40.0, 43.0]);
println!("{:#?}", output);
```

If we run it the output will be:

```bash
>cargo run --example speed-control 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\examples\speed-control.exe`
[
    0.45999999999999996,
]
```
