# Benchmarking Builder Pattern (in Rust)

The builder pattern is a creational design pattern used to construct complex objects
step by step. It separates the construction of an object from its representation,
allowing the same construction process to create different representations.

In Rust, the builder pattern is commonly used to create structs with many optional
fields or fields that can have different types.
It provides a fluent API for setting values of these fields and eventually constructing the desired object.

## Types of Builder Patterns

For all the example in this topic, consider this struct

```rust
   struct Struct {
       name: String,
   }
```

### 1. StructBuilder with Owned Types (Mutable Reference)

This builder pattern uses owned types for the fields and propogates them using mutable reference.
It allows setting values using mutable references to the builder.

```rust
struct StructBuilder {
    name: Option<String>,
}

impl StructBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn build(self) -> Struct {
        Struct {
            name: self.name.unwrap_or_default(),
        }
    }
 }
```

### 2. StructBuilder with Borrowed Types (Mutable Reference)

This builder pattern uses borrowed types for the fields and propogates them using mutable reference.
It allows setting values using borrowed references to the builder.

```rust
struct StructBuilder<'a> {
    name: Option<&'a str>,
}

impl<'a> StructBuilder<'a> {
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Struct {
        Struct {
            name: self.name.unwrap_or_default().into(),
        }
    }
 }
```

### 3. StructBuilder with Owned Types (Owned Type)

This builder pattern uses owned types for the fields and propogates them using owned type.
It allows setting values using owned references to the builder.

```rust
struct StructBuilder {
    name: Option<String>,
}

impl StructBuilder {
    pub fn name(&mut self, name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            ..self
        }
    }

    // build() same as 1
 }
```

### 4. StructBuilder with Borrowed Types (Owned Type)

This builder pattern uses borrowed types for the fields and propogates them using owned type.
It allows setting values using owned references to the builder.

```rust
struct StructBuilder<'a> {
    name: Option<&'a str>,
}

impl<'a> StructBuilder<'a> {
    pub fn name(&mut self, name: &str) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }

    // build() same as 2
 }
```

## Benchmarking

Benchmarks are important, specially when performance is critical to decide which builder pattern to use

My intuitive thought were that `2. StructBuilder with Borrowed Types (Mutable Reference)` would be most performant. But to my surprise

### Types of Benchmarks performed

#### Random Data Builder

The Random Data Builder benchmarks the performance of constructing structs with randomly generated data.
This benchmark simulates real-world scenarios where the values of struct fields may vary each time an object is created.

#### Definite Data Builder

The Definite Data Builder benchmarks the performance of constructing structs with definite data, where the values of struct fields are predetermined.
This benchmark provides insights into the overhead introduced by the builder pattern when using fixed values for object creation.

### Results

Reading the titles in results:

Title format: `rand|def, mutref|owned, brw|owned`

1. `rand|def`: `rand` means random data was generated and supplied, `def` means definite (predetermined) data was provided
2. `mutref|owned`: `muref` means builder was propogated using mutable reference, `owned` means builder was propogated via owned type
3. `brw|owned`: `brw` means the builder struct contains borrowed types, `owned` means builder struct contains owned types

```shell

randmutrefowned         time:   [449.74 ns 453.26 ns 456.70 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low severe
  1 (1.00%) high severe

randmutrefbrw           time:   [541.01 ns 548.86 ns 557.56 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

randownedowned          time:   [478.31 ns 482.57 ns 487.14 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild

randownedbrw            time:   [506.05 ns 509.67 ns 513.56 ns]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild

defmutrefowned          time:   [164.30 ns 165.26 ns 166.20 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

defmutrefbrw            time:   [93.536 ns 94.207 ns 94.943 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

defownedowned           time:   [67.587 ns 67.979 ns 68.396 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild

defownedbrw             time:   [91.504 ns 92.225 ns 93.050 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```

## Conclusion
