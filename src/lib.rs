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

#[macro_export]
macro_rules! count_non_zero {
    ( $iterable:expr )  => {
        {
           $iterable.iter().filter(|&n| n != &0).count()
        }
    };
}

#[macro_export]
macro_rules! remove_zeroes {
    ( $iterable:expr )  => {
        {
           let mut array = [0; count_non_zero!($iterable)];
           for (&from, &mut ref mut to ) in $iterable.iter().filter(|&n| n != &0).zip(array.iter_mut()){
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

    #[test]
    fn counts_non_zero() {
        let count  = count_non_zero!(&NUMBERS);
        assert_eq!(count, 3);
    }

    #[test]
    fn removes_zeros() {
        let one_two_three  = remove_zeroes!(&NUMBERS);
        assert_eq!([1,2,3], one_two_three);
    }

}
