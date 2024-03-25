# Fuzzy-Logic-rs

> [!CAUTION]
> This repository is not complete yet. If you have any suggestions on how to make it better please send me an email.

In this project I've tried to implement a simple and lightweight implementation of fuzzy logic only using std.

## Membership Functions

I have implemented some useful membership functions. Also you can define a custom membership function if you like using `::new()` . Here is a List of all implemented membership functions.

1. Triangular
2. Trapezoidal
3. Linear-z
4. Linear-s
5. step-down
6. step-up
7. Gaussian
8. Double-Gaussian
9. Bell shape

## Operations

Right now all 4 main operators are implemented.(i.e. `+ - * /`)

* [ ] TODO: add compound operators

## Inference Systems

* [ ] Mamdani
* [ ] TSK

## Future Plans

* [ ] add meta-heuristics
* [ ] add ANFIS support
* [ ] add dedicated control module
* [ ] add fuzzy c-means
