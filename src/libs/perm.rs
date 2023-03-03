pub fn generate_permutations(radix: u32, len: usize) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut permulation = vec![0; len];
    generate_permutations_helper(&mut permulation, &mut results, 0, radix);
    results
}

pub fn generate_permutations_helper(
    permulation: &mut [u8],
    results: &mut Vec<Vec<u8>>,
    index: usize,
    radix: u32,
) {
    if index == permulation.len() {
        results.push(permulation.to_vec());
        return;
    }

    for i in 0..(radix as u8) {
        permulation[index] = i;
        generate_permutations_helper(permulation, results, index + 1, radix);
    }
}
