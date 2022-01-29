#[macro_use] extern crate cpython;

use cpython::{Python, PyResult};

//
fn fibo(_py: Python, n: u64) -> PyResult<u64> {
    if n < 2 {
        return Ok(1)
    }
    let mut prev1 = 1;
    let mut prev2 = 1;
    for _ in 1..n {
        let new = prev1 + prev2;
        prev2 = prev1;
        prev1 = new;
    }
    Ok(prev1)
}

// To build the Python compatible module
py_module_initializer!(fib_example, initfib_example,
    PyInit_fib_example, |py, m| {
        m.add(py, "__doc__", "This is a test rust fibonacci method")?;
        m.add(py, "fibo", py_fn!(py, fibo(rand_int: u64)))?;
        // Initializer s macro needs a Result<> as return value
        Ok(())
    });
