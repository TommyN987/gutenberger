pub fn add_to_vec<T: PartialEq + Clone>(vec_option: &mut Option<Vec<T>>, item: T) {
    match vec_option {
        Some(vec) => {
            if !vec.iter().any(|i| *i == item) {
                vec.push(item);
            }
        }
        None => *vec_option = Some(vec![item]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_vec_empty() {
        let mut vec_option: Option<Vec<i32>> = None;
        add_to_vec(&mut vec_option, 5);
        assert_eq!(vec_option, Some(vec![5]));
    }

    #[test]
    fn test_add_to_vec_duplicate() {
        let mut vec_option = Some(vec![5, 6, 7]);
        add_to_vec(&mut vec_option, 5);
        assert_eq!(vec_option, Some(vec![5, 6, 7]));
    }

    #[test]
    fn test_add_to_vec_new_item() {
        let mut vec_option = Some(vec![5, 6, 7]);
        add_to_vec(&mut vec_option, 8);
        assert_eq!(vec_option, Some(vec![5, 6, 7, 8]));
    }
}
