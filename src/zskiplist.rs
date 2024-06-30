struct ZSkipList {
    // TODO: Implementation details here
}

impl ZSkipList {
    pub fn zsl_create() -> Self {
        // TODO: Implementation of zsl_create here
        ZSkipList {
            // Initialize the skip list
        }
    }

    pub fn zsl_free(&mut self) {
        // TODO: Implementation of zsl_free here
        // Free any allocated memory and clean up resources
    }

    pub fn zsl_insert(&mut self, score: f64, element: String) {
        // TODO: Implementation of zsl_insert here
        // Insert the element with the given score into the skip list
    }

    pub fn zsl_delete(&mut self, score: f64, element: String) {
        // TODO: Implementation of zsl_delete here
        // Delete the element with the given score from the skip list
    }

    pub fn zsl_get_rank(&self, score: f64, element: String) -> Option<usize> {
        // TODO: Implementation of zsl_get_rank here
        // Get the rank of the element with the given score in the skip list
        // Return None if the element is not found
    }

    pub fn zsl_get_element_by_rank(&self, rank: usize) -> Option<String> {
        // TODO: Implementation of zsl_get_element_by_rank here
        // Get the element at the given rank in the skip list
        // Return None if the rank is out of range
    }

    pub fn zsl_is_in_range(&self, min: f64, max: f64) -> bool {
        // TODO: Implementation of zsl_is_in_range here
        // Check if there are any elements in the skip list within the given score range
    }

    pub fn zsl_first_in_range(&self, min: f64, max: f64) -> Option<String> {
        // TODO: Implementation of zsl_first_in_range here
        // Get the first element in the skip list within the given score range
        // Return None if there are no elements in the range
    }

    pub fn zsl_last_in_range(&self, min: f64, max: f64) -> Option<String> {
        // TODO: Implementation of zsl_last_in_range here
        // Get the last element in the skip list within the given score range
        // Return None if there are no elements in the range
    }

    pub fn zsl_delete_range_by_score(&mut self, min: f64, max: f64) -> usize {
        // TODO: Implementation of zsl_delete_range_by_score here
        // Delete all elements in the skip list within the given score range
        // Return the number of elements deleted
    }

    pub fn zsl_delete_range_by_rank(&mut self, start: usize, end: usize) -> usize {
        // TODO: Implementation of zsl_delete_range_by_rank here
        // Delete all elements in the skip list within the given rank range
        // Return the number of elements deleted
    }
}