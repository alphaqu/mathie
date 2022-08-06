# Mathie

## Overview
Mathie is a rust math type library which contains the 3 basic types
for doing anything 2D.
- `Value` A single number value.
- `Vec2` A two-dimensional position.
- `Rect` A two-dimensional area that has an origin and a size.

## Math
Doing math with the builtin types is designed to be simple and powerful.
All of them support Addition, Subtraction, Multiplication, Division and Remainder.
```rust
fn add() {
    assert_eq!(Vec2D::new(0.5, 0.5) + Vec2D::new(1.0, 1.0), Vec2D::new(1.5, 1.5));
    assert_eq!(Vec2D::new(0.5, 0.5) + 1.0,                  Vec2D::new(1.5, 1.5));
}
```

## Units
This library runs on the optional concept of units which may be sized
and are designed to be easily converted between without thinking about it.

```rust
fn cm_to_m() {
    let v0 = Vec2D::<f32, Centimeter>::new_def(250.0, 250.0);
    // () is base unit. In all cases its Meter
    let v1 = v0.convert_def::<()>().unwrap();
    assert_eq!(v1, Vec2D::new_def(2.5, 2.5));
}
```

There are builtin units that are optional.
- `metric_units` 20 Metric prefixes
- `imperial_units` 10 Imperial prefixes
- `nautical_units` Nautical mile

## Compatibility
This library has optional features for compatibility between other
math type libraries like `euclid`.

```rust
fn euclid_compat() {
    let _: Vec2D<f32, ()> = euclid::Vector2D::new(1.0, 1.0).into();
}
```
