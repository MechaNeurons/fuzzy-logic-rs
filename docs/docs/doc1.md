# Functions and Structs and Enums

## fuzzy_inference_systems.rs

### MamdaniFIS

This is Mamdani Inference system

```rust
pub struct MamdaniFuzzyInferenceSystem {
s_norm: SNorms,
t_norm: TNorms,
implication: Implications,
aggregation: Aggregations,
defuzzifier: Defuzzifiers,
rules: Vec<Rule>,
inputs: Vec<InputVariable>,
outputs: Vec<OutputVariable>,
}
```

You can create a this system using `::new` method witch is defined as

```rust
pub fn new(
        s_norm: SNorms,
        t_norm: TNorms,
        implication: Implications,
        aggregation: Aggregations,
        defuzzifier: Defuzzifiers,
    ) -> Self
```

Each of these fields are documented separately.

To add an input you must use `add_input` method. It needs an `InputVariables`.

```rust
pub fn add_input(&mut self, input: InputVariable)
```

The same is true for output but it needs an `OutputVariables`.

```rust
pub fn add_output(&mut self, output: OutputVariable)
```

To add fuzzy rules you can use `add_rule` which accepts a `Rule` struct as an input

```rust
pub fn add_rule(&mut self, rule: Rule)
```

After defining the instance, inputs, outputs, and rules, you can use compute the system output using the `compute_output` which excepts a `vec<f64>` of input variables and outputs a `vec<f64>` containing output variables.

!!!note
    please note that the inputs must be in the order of when you use `add_input` method, and outputs will be in the same order as you add them.

```rust
pub fn compute_outputs(&self, input_vec: Vec<f64>) -> Vec<f64>
```

### TSKFIS

This is TSK Inference system.

```rust
pub struct TSKFuzzyInferenceSystem {
    s_norm: SNorms,
    t_norm: TNorms,
    defuzzification: TSKDefuzzifiers,
    rules: Vec<Rule>,
    inputs: Vec<InputVariable>,
    outputs: Vec<TSKOutputVariable>,
}

pub type TSKFIS = TSKFuzzyInferenceSystem;
```

You can create a this system using `::new` method witch is defined as:

```rust
pub fn new(
    s_norm: SNorms, 
    t_norm: TNorms, 
    defuzzification: TSKDefuzzifiers
    ) -> Self 
```

Each of these fields are documented separately.

To add an input you must use `add_input` method. It needs an `InputVariables`.

```rust
pub fn add_input(&mut self, input: InputVariable)
```

The same is true for output but it needs an `TSKOutputVariables`.

```rust
pub fn add_output(&mut self, output: TSKOutputVariable)
```

To add fuzzy rules you can use `add_rule` which accepts a `Rule` struct as an input

```rust
pub fn add_rule(&mut self, rule: Rule)
```

After defining the instance, inputs, outputs, and rules, you can use compute the system output using the `compute_output` which excepts a `vec<f64>` of input variables and outputs a `vec<f64>` containing output variables.

```rust
pub fn compute_outputs(&self, input_vec: Vec<f64>) -> Vec<f64>
```

## s_norms.rs

S-norms are used in FIS systems to compute many thing. In this crate I used it as `or` method in the rules. It is defined as bellow:

```rust
pub enum SNorms {
    Max,
    Custom(fn (&[f64])->f64),
}
```

To use default s-norms you can use the `Snorms::Max`.

Right now only `Max` is defined by default, but you can add a custom s-norm method by defining a function who's signature is like the enum.

## t_norms.rs

T-norms are used in FIS systems to compute many thing. In this crate I used it as `and` method in the rules. You can use t-norms as the same as s-norms.

```rust
pub enum TNorms {
    Min,
    Product,
    Custom(fn(&[f64]) -> f64),
}
```

## implications.rs

Implication is the act of computing the overall membership of the rule for example

$$
IF \ x_1 \ IS \ \mu_1 \ AND \ x_2 \ IS \mu_2 \ THEN \ output \ IS \ \mu_o
$$

In this form the output of implication is how much the IF part belong to this rule and how is the output membership looks. It is defined as:

```rust
pub enum Implications {
    Min,
    Product,
    Custom(fn(f64, &Vec<f64>) -> Vec<f64>),
}
```

!!!danger "More Implication methods"
    More implication methods will be added in the future like `Godel` or `Zadeh`.

Use of this is the same as t-norms and s-norms and you can define your own methods for implications.

The 1st parameter of the custom function is $\mu$ and the 2nd argument is an output range.

## rules.rs

You can define a rule whit:

```rust
pub struct Rule {
    relations: Vec<i32>,
    weight: f64,
    method: Kind,
}
```

there are two ways to add new rules `add_or` and `add_and`. The relations vec is defined as in a rule how the inputs interact with each other.

You can change the weight to a value between [0.0 1.0].

You can complement(i.e. `is not`) a membership function or a range by adding a `-` sign in front of it.

## aggregation.rs

Aggregation is the how to compute the overall membership of the output. Basically each rule will produce a range and how to convert them to a single membership range is the overall shape will looks.

```rust
pub enum Aggregations {
    Max,
    Sum,
    Custom(fn(Vec<Vec<f64>>) -> Vec<f64>)
}
```

You can use it just like previous methods.

## defuzzification.rs

This is the process in which the fuzzy number is converted back to a crisp number.

```rust
pub enum Defuzzifiers {
    Centroid,
    Bisection,
    Custom(fn(Vec<f64>,&Vec<f64>)->f64),
}
```

It is used as before.

!!!danger "More defuzzification methods"
    More defuzzification method will be added in the future like `CenterOfMeans` or `MeanOfMax`.

For TSK systems there are other defuzzification method so we have to use another enum.

```rust
pub enum TSKDefuzzifiers {
    Mean,
    Custom(fn(&Vec<f64>, &Vec<f64>) -> f64),
}
```

## variables.rs

### InputVariable

To define a input variable you have to use this struct.

```rust
pub struct InputVariable {
    name: String,
    range: (f64, f64),
    mfs: Vec<MembershipFunction>,
}
```

You need to use `::new()` to make a new input variable. Range is the range in which this variable is valid.
!!! Note "range"
    There is no restrictions on range now but in the future this will change.

```rust
pub fn new(name: String, range: (f64, f64)) -> Self 
```

You can add new membership to a variable. Please note that in need to be a `MembershipFunction`

```rust
pub fn add_membership(&mut self, mf: MembershipFunction)
```

To fuzzify a variable(i.e. convert it from a crisp set to a fuzzy set) you can use the function below. This function will get an index of which membership function you want to fuzzify and the value that need to be converted.

```rust
pub fn fuzzify(&self, idx: usize, x: f64) -> f64 
```

### OutputVariable

To define an output variable you need to use this struct. It has a vec of membership range and a universe. The `universe` it the range in which the output is defined and `mrs` are the ranges of different memberships. Please note that any value outside of `universe` is not defined.

```rust
pub struct OutputVariable {
    name: String,
    mrs: Vec<MembershipRange>,
    universe: Vec<f64>,
}
```

You can create a new `OutputVariable` using `::new()` which need a range where the universe is defined and, and how many number of points you want tou be in the range whit `n` argument.

```rust
pub fn new(name: String, range: (f64, f64), n: i32) -> Self
```

### TSKOutputVariable

This struct is used for creating a TSK output variable. 

```rust
pub struct TSKOutputVariable {
    name: String,
    mfs: Vec<TSKMembershipFunction>,
}
```

By using `::new()` you can create a new instance.

```rust
pub fn new(name: String) -> Self 
```

You can add `constant` , `linear` or custom membership function by calling `add` methods.

```rust
pub fn add_membership(&mut self, membership: TSKMembershipFunction) {
    self.mfs.push(membership);
}

pub fn add_constant_membership(&mut self, value: f64) {
    self.mfs.push(TSKMembershipFunction::Constant(value));
}

pub fn add_linear_membership(&mut self, coefficients: Vec<f64>) {
    self.mfs.push(TSKMembershipFunction::Linear(coefficients));
}
```

## membership_functions.rs

This file will defined the membership functions that is used in input variable. Several defaults are defined but you can also define your own.

```rust
pub struct MembershipFunction {
    name: String,
    kind: Kind,
}
```

`kind` is the kind of membership function that you want to use and it defined as an enum.

```rust
pub enum Kind {
    Triangle(Triangle),
    Trapezoid(Trapezoid),
    LinearZ(LinearZ),
    LinearS(LinearS),
    StepDown(StepDown),
    StepUp(StepUp),
    Gaussian(Gaussian),
    DoubleGaussian(DoubleGaussian),
    Bell(Bell),
    Normal(Gaussian),
    Custom(Custom),
}
```

Each of these variants have a dedicated struct that you can make using `::new()`.

For TSK systems you have to use this enum.

```rust
pub enum TSKMembershipFunction {
    Constant(f64),
    Linear(Vec<f64>),
    Custom(fn(&Vec<f64>) -> f64),
}
```

## membership_ranges.rs

Membership ranges are used to define an output.

```rust
pub struct MembershipRange {
    name: String,
    mu: Vec<f64>,
}
```

## Creating a membership function or membership range

They have basically the same. You can create a new using the syntax `::new_<what_kind>` and add appropriate arguments to it. Please follow the function signature or check out the [example](https://mechaneurons.github.io/fuzzy-logic-rs/examples/speed_control/) for more information.
