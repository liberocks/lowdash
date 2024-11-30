use std::any::TypeId;

/// Determines if the collection contains floating-point numbers (`f32` or `f64`).
///
/// # Arguments
///
/// * `collection` - A slice of items.
///
/// # Returns
///
/// * `true` if all elements in the collection are of type `f32` or `f64`, otherwise `false`.
pub fn is_collection_float(collection: &[Box<dyn std::any::Any>]) -> bool {
    collection.iter().all(|item| {
        let type_id = item.type_id();
        type_id == TypeId::of::<f32>() || type_id == TypeId::of::<f64>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_collection_float_with_float() {
        let collection: Vec<Box<dyn std::any::Any>> =
            vec![Box::new(1.0f64), Box::new(2.0f64), Box::new(3.0f64)];
        let result = is_collection_float(&collection);
        assert!(result);
    }

    #[test]
    fn test_is_collection_float_with_integer() {
        let collection: Vec<Box<dyn std::any::Any>> = vec![Box::new(1), Box::new(2), Box::new(3)];
        let result = is_collection_float(&collection);
        assert!(!result);
    }

    #[test]
    fn test_is_collection_float_with_mixed_types() {
        let collection: Vec<Box<dyn std::any::Any>> =
            vec![Box::new(1.0f64), Box::new(2), Box::new(3.0f64)];
        let result = is_collection_float(&collection);
        assert!(!result);
    }
}
