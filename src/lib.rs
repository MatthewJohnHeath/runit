#[macro_export]
macro_rules! make_array {
    ( $iterable:expr )  => {
        {
           const LENGTH :usize = $iterable.len();
           let mut array = [0; LENGTH];
           for (&from, &mut ref mut to ) in $iterable.iter().zip(array.iter_mut()){
            *to = from
            }
            array
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    const NUMBERS : [u32; 4] = [0,1,2,3]; 

    #[test]
    fn with_array() {
        let also_numbers = make_array!(&NUMBERS);
        assert_eq!(also_numbers, NUMBERS);
    }

}
