## Overview
This document provides details on the Rust program that performs polynomial division in the modular arithmetic system $\mathbb{Z}_n$. The program calculates both the quotient and remainder of polynomial division while ensuring that all calculations adhere to modular arithmetic rules, particularly ensuring that negative values are reduced to the range $[0, n-1]$.
## Features
- **Modular Inverse Calculation:** The `mod\_inverse` function implements the Extended Euclidean Algorithm to compute the modular inverse of a given number.
- **Polynomial Division in $\mathbb{Z}_n$:** The `poly\_division\_zn` function divides two polynomials in the modular arithmetic system $\mathbb{Z}_n$, returning both the quotient and remainder in $\mathbb{Z}_n$.
- **Handling Negative Values:** Intermediate and final values of the quotient and remainder are reduced to the appropriate range $[0, n-1]$ to maintain correctness in modular arithmetic.
## Function Descriptions
- `mod\_inverse(a: i32, n: i32) -> Option<i32>`
   - **Purpose:** Computes the modular inverse of $a$ under modulo $n$.
   - **Returns:**
      - **Some(inverse)** if the modular inverse exists, or
      - **None** if no modular inverse exists (i.e., when $\gcd(a, n) \neq 1$).
  - `poly\_division\_zn(dividend: Vec<i32>, divisor: Vec<i32>, n: i32) -> (Vec<i32>, Vec<i32>)`
    - **Purpose:** Divides the `dividend` polynomial by the `divisor` polynomial in the modular arithmetic system $\mathbb{Z}_n$, returning both the quotient and remainder.
    - **Parameters:**
      - **dividend:** Coefficients of the dividend polynomial.
      - **divisor:** Coefficients of the divisor polynomial.
      - `n`: Modulus for the calculations.
   - **Returns:** A tuple (`quotient`, `remainder`), where both are vectors of coefficients in $\mathbb{Z}_n$.
## Example
    
