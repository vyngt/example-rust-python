use pyo3::prelude::{pyfunction, PyResult};
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Two Sum in Leetcode (no. 1)
fn two_sum_rs(nums: Vec<usize>, target: usize) -> Vec<usize> {
    let mut table: HashMap<usize, usize> = HashMap::new();
    for (idx, n) in nums.iter().enumerate() {
        let i = idx as usize;
        if table.contains_key(&n) {
            return vec![table[&n], i];
        } else {
            table.insert(target - n, i);
        }
    }
    return vec![0, 1];
}

#[pyfunction]
pub fn two_sum(nums: Vec<usize>, target: usize) -> PyResult<Vec<usize>> {
    Ok(two_sum_rs(nums, target))
}
