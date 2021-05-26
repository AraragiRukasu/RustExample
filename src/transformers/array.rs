pub fn rotate_array_left(array: &[u32], mut positions: usize) -> Vec<u32>
{
    let array_len = array.len();
    if positions > array_len {
        positions %= array_len;
    }

    let mut result:Vec<u32> = vec![0; array_len];
    result[..(array_len - positions)].clone_from_slice(&array[positions..]);
    result[(array_len - positions)..].clone_from_slice(&array[..positions]);

    return result;
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn rotate_left_three_positions_should_work(){
        let example_array = [1, 2, 3, 4, 5];
        let positions = 3;
        let expected_array = [4, 5, 1, 2, 3];

        let result = rotate_array_left(&example_array, positions);

        assert_eq!(result, expected_array);
    }
}