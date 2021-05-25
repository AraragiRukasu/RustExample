pub fn rotate_array_left(array: &mut [u32], mut positions: usize)
{
    let array_len = array.len();
    if positions > array_len {
        positions %= array_len;
    }

    let mut buffer:Vec<u32> = vec![0; positions];
    buffer.clone_from_slice(&array[..positions]);
    
    for i in 0..(array_len - positions) {
        array[i] = array[positions + i];
    }

    for j in (array_len - positions)..array_len {
        array[j] = buffer[j - (array_len - positions)];
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn rotate_left_three_positions_should_work(){
        let mut example_array = [1, 2, 3, 4, 5];
        let positions = 3;
        let expected_array = [4, 5, 1, 2, 3];

        rotate_array_left(&mut example_array, positions);

        assert_eq!(example_array, expected_array);
    }
}