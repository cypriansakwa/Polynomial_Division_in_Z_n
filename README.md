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
  - function demonstrates polynomial division in $\mathbb{Z}_6\) (mod 6)$.
    - \item **Dividend:** $[-4, 4, -2, 0, 4]$ (which represents $-4x^4 + 4x^3 - 2x^2 + 0x + 4$).
    \item **Divisor:** $[1, -4, -2]$ (which represents $x^2 - 4x - 2$).
    \item **Modulus:** $6$
 ## Example Output:
 - The expected output will print the quotient and remainder of the polynomial division in $\mathbb{Z}_6$, with values reduced into the range $[0, 5]$.
>```
>Quotient: [2, 0, 2]
>Remainder: [2, 2]
## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
- Modify the input polynomials `dividend`, `divisor`) and modulus $n$ in the `main` function to test other cases.
## Notes
- The code assumes that the leading coefficient of the divisor has a modular inverse in $\mathbb{Z}_n$. If no modular inverse exists, the program will panic.
- All operations are performed in $\mathbb{Z}_n$, ensuring that results are bounded within the range $[0, n-1]$.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Polynomial_Division_in_Z_n.git
   cd Polynomial_Division_in_Z_n
