# Multi-Language Benchmark

This repository contains the same benchmark implemented across multiple programming languages to compare **raw execution performance**, **compiler/runtime behavior**, and **language overhead** under identical logic.

Each language implementation follows **exactly the same algorithm**, differing only where the language syntax or runtime requires it.

---

## Handled Languages

- [Java](./docs/REQUIREMENTS.md#Java)
- [JavaScript](./docs/REQUIREMENTS.md#JavaScript)
- [PHP](./docs/REQUIREMENTS.md#PHP)
- [Python](./docs/REQUIREMENTS.md#Python)
- [Rust](./docs/REQUIREMENTS.md#Rust)
- [TypeScript](./docs/REQUIREMENTS.md#TypeScript)

---

## Requirements

Follow this file requirements for each languages: [REQUIREMENTS.md](./docs/REQUIREMENTS.md)

---

## What Is Being Benchmarked?

The benchmark measures the time required to execute a **tight computational loop** that performs integer arithmetic.

Specifically, it:

- Iterates from `1` to `n`
- Accumulates the sum of the loop index
- Outputs the elapsed time in seconds, milliseconds or microseconds

This type of benchmark is intentionally simple so that results primarily reflect:

- Loop performance
- Integer arithmetic speed
- Compiler optimizations
- Runtime overhead (GC, JIT, VM startup, etc.)

---

## Benchmark Rules

All implementations must:

- Use a **single-threaded** execution model
- Perform the **same number of iterations**
- Use **native integer types**
- Avoid unnecessary I/O inside the loop
- Print or return the final result

No language-specific optimizations (SIMD, unsafe code, parallelism) are used.

---

## Pseudocode (Used by Every Implementation)

```txt
BEGIN PROGRAM

// 1. Read input argument for n
IF command-line argument exists THEN
    n = parse argument as integer
ELSE
    n = 100_000_000  // default value
END IF

// 2. Start measuring time
start_time = current time

// 3. Perform a simple summation loop
total = 0
FOR i FROM 0 TO n-1 DO
    total = total + i
END FOR

// 4. Measure elapsed time
duration = current time - start_time

// 5. Convert duration to different units
seconds = duration in seconds
milliseconds = seconds * 1000
microseconds = seconds * 1_000_000

// 6. Choose the most appropriate unit to display
IF seconds >= 1.0 THEN
    result = seconds
    unit = "s"
ELSE IF milliseconds >= 1.0 THEN
    result = milliseconds
    unit = "ms"
ELSE
    result = microseconds
    unit = "µs"
END IF

// 7. Print the result
PRINT "Rust: ", result (rounded to 1 decimal), unit

END PROGRAM
```

This pseudocode is the **source of truth** for every implementation in this repository.

---

## Project Structure

Each folder contains the benchmark written in a different language.

``benchmark.sh`` is used to compile and run all implementations in a consistent way.

---

## Notes on Fairness

- Results may vary depending on compiler flags, runtime versions, and hardware.
- Interpreted and JIT-compiled languages may require warm-up to reach peak performance.
- This benchmark favors raw loop throughput, not real-world application behavior.

---

## Purpose

This repository is intended for:

- Performance comparisons
- Compiler/runtime exploration
- Educational benchmarking
- Curiosity-driven experimentation

Not for declaring a “best” language.
