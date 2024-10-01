// Function to perform modular division in Z_n (modular inverse)
fn mod_inverse(a: i32, n: i32) -> Option<i32> {
    // Use Extended Euclidean Algorithm to find the modular inverse
    let mut t = 0;
    let mut new_t = 1;
    let mut r = n;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        t = t - quotient * new_t;
        r = r - quotient * new_r;

        std::mem::swap(&mut t, &mut new_t);
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        return None; // No inverse exists
    }
    if t < 0 {
        t = t + n;
    }
    Some(t)
}

// Polynomial division in Z_n (modular arithmetic)
fn poly_division_zn(dividend: Vec<i32>, divisor: Vec<i32>, n: i32) -> (Vec<i32>, Vec<i32>) {
    let mut quotient = vec![0; dividend.len() - divisor.len() + 1];
    let mut remainder = dividend.clone();

    while remainder.len() >= divisor.len() {
        let leading_coeff_dividend = remainder[0];
        let leading_coeff_divisor = divisor[0];

        // Find the modular inverse of the leading coefficient of the divisor in Z_n
        let leading_inv = match mod_inverse(leading_coeff_divisor, n) {
            Some(inv) => inv,
            None => panic!("No modular inverse exists for divisor leading coefficient"),
        };

        // Calculate factor and ensure it stays within [0, n-1] by applying (mod n + n) % n
        let factor = (leading_coeff_dividend * leading_inv) % n;
        quotient[remainder.len() - divisor.len()] = (factor % n + n) % n;

        // Subtract factor * divisor from remainder
        for i in 0..divisor.len() {
            remainder[i] = (remainder[i] - factor * divisor[i]) % n;
            if remainder[i] < 0 {
                remainder[i] += n;
            }
        }

        // Remove leading zeros from the remainder
        while remainder.len() > 0 && remainder[0] == 0 {
            remainder.remove(0);
        }
    }

    (quotient, remainder)
}

fn main() {
    // Example: Polynomial division in Z_6 (mod 6)
    let dividend = vec![-4, 4, -2, 0, 4];  // Example polynomial
    let divisor = vec![1, -4, -2];         // Example divisor
    let n = 6;                             // Z_6 (mod 6)

    let (quotient, remainder) = poly_division_zn(dividend, divisor, n);

    println!("Quotient: {:?}", quotient);   // Quotient in Z_6
    println!("Remainder: {:?}", remainder); // Remainder in Z_6
}
