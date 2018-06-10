// recursive implementation of Lucas sequences, including Fibonacci, 
// Mersennes numbers, Pell numbers, and more
// https://en.wikipedia.org/wiki/Lucas_sequence

 
// generates the nth number in the Lucas sequence of the form U(P,Q) or V(P,Q) 
fn lucas_sequence(n: i32, vector: char, p: i32, q: i32) -> i32 {
    if n == 0 {
        return match vector {
            'u' => 0,
            'v' => 2,
             _  => panic!("Invalid sequence! Choices are fib (u) or lucas (v).")
        }
    }

    if n == 1 {
        return 1;
    }
   
    return p * lucas_sequence(n-1, vector, p, q) + (-q) * lucas_sequence(n-2, vector, p, q)
 }


// Fibonacci number: U(1, -1)
fn fibonacci(n: i32) -> i32 {
    return lucas_sequence(n, 'u', 1, -1)
}

// Lucas number: V(1, -1)
fn lucas(n: i32) -> i32 {
    return lucas_sequence(n, 'v', 1, -1)
}

// Pell number: U(2, -1)
fn pell(n: i32) -> i32 {
    return lucas_sequence(n, 'u', 2, -1)
}

// Pell-Lucas number: V(2, -1)
fn pell_lucas(n: i32) -> i32 {
    return lucas_sequence(n, 'v', 2, -1)
}

// Jacobsthal number: U(1, -2)
fn jacobsthal(n: i32) -> i32 {
    return lucas_sequence(n, 'u', 1, -2)
}

// Jacobshal-Lucas number: V(1, -2)
fn jacobsthal_lucas(n: i32) -> i32 {
    return lucas_sequence(n, 'v', 1, -2)
}

// Mersenne number: U(3, 2)
fn mersenne(n: i32) -> i32 {
    return lucas_sequence(n, 'u', 3, 2)
}

// numbers of the form 2^n: V(3, 2)
fn mersenne_lucas(n: i32) -> i32 {
    return lucas_sequence(n, 'v', 3, 2)
}


