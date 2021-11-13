# Lal.rs

The Little Assertion Language provides a simple interface for
automated theorem provers.  The language replaces the Whiley Assertion
Language used in the original
[WhileyTheoremProver](https://github.com/Whiley/WhileyTheoremProver/).
In essence, Lal provides a form of first-order logic with specific
extensions designed to support programming languages such as
[Whiley](http://whiley.org).

## Example

A very simple example is the following:

```
assert:
  forall (int x):
    (x > 0) ==> (x >= 0)
```

This illustrates a simple assertion over integer values in Lal.
Another example which illustrates array values is the following:

```
assert:
  forall (int[] xs, int i):
     (xs[i] > 0) ==> (xs[i] >= 0)
```     