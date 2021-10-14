/// A circular buffer, circular queue, ring buffer is a data structure that uses a single, fixed-size buffer as if it were connected end-to-end.
/// This structure lends itself easily to buffering data streams.
///
/// # Examples
/// ```
/// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
///
/// circular_buffer.enqueue(1);
///
/// match circular_buffer.dequeue() {
///     Some(data) => assert_eq!(*data, 1),
///     None => panic!("Data must not be empty")
/// }
/// ```
///
#[derive(Debug)]
pub struct Circular<T> {
    front_index: usize,
    rear_index: usize,
    size: usize,
    internal_vec: Vec<T>,
    capacity: usize,
    push_enabled: bool,
}

impl<T> Circular<T> {
    /// Creates a new instance of circular queue
    ///
    /// # Arguments
    /// * `capacity` - capacity of the queue. note that the real capacity is equal to capacity determined by the argument + 1
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    /// ```
    pub fn new(capacity: usize) -> Circular<T> {
        Circular {
            front_index: 0,
            rear_index: 0,
            internal_vec: Vec::with_capacity(std::cmp::max(capacity + 1, 1)),
            size: 0,
            push_enabled: true,
            capacity: std::cmp::max(capacity + 1, 1),
        }
    }

    /// Returns number of items in the queue
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    /// assert_eq!(circular_buffer.size(), 0);
    ///
    /// circular_buffer.enqueue(1);
    /// assert_eq!(circular_buffer.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Returns wether queue is empty or not. this implies wether size is equal to 0 or not.
    /// capacity will stay the same.
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    /// assert_eq!(circular_buffer.empty(), true);
    ///
    /// circular_buffer.enqueue(1);
    /// assert_eq!(circular_buffer.empty(), false);
    /// ```
    pub fn empty(&self) -> bool {
        return self.size == 0;
    }

    /// Returns true if there are no more room for inserting new items.
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    /// assert_eq!(circular_buffer.full(), false);
    ///
    /// circular_buffer.enqueue(1);
    /// assert_eq!(circular_buffer.full(), true);
    /// ```
    pub fn full(&self) -> bool {
        return (self.rear_index + 1) % self.capacity == self.front_index;
    }

    /// If queue is not full it will insert an element at the end of the queue.
    /// If queue is full, oldest item will be discarded and new item will be inserted at the end of the queue.
    ///
    /// # Arguments
    /// * `element`: item to be inserted in the queue
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    ///
    /// circular_buffer.enqueue(1);
    /// ```
    pub fn enqueue(&mut self, element: T) {
        // enqueue is only possible on queue with capacity > 1
        // if capacity of queue is not enough then do nothing
        if self.capacity <= 1 {
            return;
        }

        // check if queue is full
        if self.full() {
            self.front_index = (self.front_index + 1) % self.capacity;

            self.size -= 1;
        }

        if self.push_enabled {
            self.internal_vec.push(element);
        } else {
            self.internal_vec[self.rear_index] = element;
        }

        self.push_enabled = !(self.rear_index + 1 == self.capacity) & self.push_enabled;

        self.rear_index = (self.rear_index + 1) % self.capacity;

        self.size += 1;
    }

    /// Returns and discards the item at the front of the queue.
    /// Returns None if there are no items in the queue.
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<usize> = rudac::queue::Circular::new(1);
    ///
    /// circular_buffer.enqueue(1);
    ///
    /// match circular_buffer.dequeue() {
    ///     Some(data) => assert_eq!(*data, 1),
    ///     None => panic!("Data must not be empty")
    /// }
    /// ```
    pub fn dequeue(&mut self) -> Option<&T> {
        if self.empty() {
            return None;
        }

        let element: &T = &self.internal_vec[self.front_index];

        self.front_index = (self.front_index + 1) % self.capacity;

        self.size -= 1;

        return Some(element);
    }

    /// Transforms each element in the queue using the transform function provided
    ///
    /// # Arguments
    /// * `transform`: function that transforms each element of the queue and returns that transformed element
    ///
    /// # Examples
    /// ```
    /// fn all_caps(text: &String) -> String {
    ///     return text.to_uppercase();
    /// }
    ///
    /// let mut circular_buffer: rudac::queue::Circular<String> = rudac::queue::Circular::new(1);
    ///
    /// circular_buffer.enqueue(String::from("element"));
    ///
    /// circular_buffer.map(all_caps);
    ///
    /// match circular_buffer.dequeue() {
    ///     Some(data) => assert_eq!(*data, String::from("ELEMENT")),
    ///     None => panic!("Data must not be None")
    /// }
    /// ```
    pub fn map(&mut self, transform: fn(&T) -> T) {
        for i in 0..self.size() {
            self[i] = transform(&self[i]);
        }
    }

    /// Transforms each element in the queue using the transform closure provided
    ///
    /// # Arguments
    /// * `transform`: closure that transforms each element of the queue and returns that transformed element
    ///
    /// # Examples
    /// ```
    /// let mut circular_buffer: rudac::queue::Circular<String> = rudac::queue::Circular::new(1);
    ///
    /// circular_buffer.enqueue(String::from("element"));
    ///
    /// circular_buffer.map_closure(|text: &String| -> String{text.to_uppercase()});
    ///
    /// match circular_buffer.dequeue() {
    ///     Some(data) => assert_eq!(*data, String::from("ELEMENT")),
    ///     None => panic!("Data must not be None")
    /// }
    /// ```
    pub fn map_closure<F>(&mut self, transform: F)
    where
        F: Fn(&T) -> T,
    {
        for i in 0..self.size() {
            self[i] = transform(&self[i]);
        }
    }

    /// Clears the queue and resets internal flags
    pub fn clear(&mut self) {
        self.internal_vec.clear();

        self.rear_index = 0;
        self.front_index = 0;
        self.size = 0;
        self.push_enabled = true;
    }
}

impl<T> std::ops::Index<usize> for Circular<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size() {
            panic!("index out of bounds");
        }
        let new_index = (self.front_index + index) % self.capacity;

        return &self.internal_vec[new_index];
    }
}

impl<T> std::ops::IndexMut<usize> for Circular<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size() {
            panic!("index out of bounds");
        }
        let new_index = (self.front_index + index) % self.capacity;

        return &mut self.internal_vec[new_index];
    }
}

pub struct CircularIterator<'a, T> {
    vec_circular: &'a Circular<T>,
    index: usize,
}

impl<'a, T> std::iter::IntoIterator for &'a Circular<T> {
    type Item = &'a T;
    type IntoIter = CircularIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CircularIterator {
            vec_circular: &self,
            index: self.front_index,
        }
    }
}

impl<'a, T> std::iter::Iterator for CircularIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.index == self.vec_circular.rear_index || self.vec_circular.empty() {
            return None;
        } else {
            let item = &self.vec_circular[self.index];
            self.index = (self.index + 1) % self.vec_circular.capacity;
            return Some(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_circular_queue_1() {
        let vc: Circular<String> = Circular::new(0);

        assert_eq!(vc.capacity, 1);
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn create_circular_queue_2() {
        let vc: Circular<String> = Circular::new(1);

        assert_eq!(vc.capacity, 2);
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn enqueue_on_capacity_zero() {
        let mut vc: Circular<String> = Circular::new(0);

        vc.enqueue(String::from("element1"));

        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn enqueue_on_capacity_big() {
        let mut vc: Circular<String> = Circular::new(10);

        vc.enqueue(String::from("element1"));
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 1);

        vc.enqueue(String::from("element2"));
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 2);
        assert_eq!(vc.size, 2);

        vc.enqueue(String::from("element3"));
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 3);
        assert_eq!(vc.size, 3);
    }

    #[test]
    fn enqueue_on_full_queue() {
        let mut vc: Circular<String> = Circular::new(3);

        vc.enqueue(String::from("element1"));

        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 1);

        vc.enqueue(String::from("element2"));
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 2);
        assert_eq!(vc.size, 2);

        vc.enqueue(String::from("element3"));
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 3);
        assert_eq!(vc.size, 3);

        // now queue is full
        vc.enqueue(String::from("element4"));
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 1);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 3);

        vc.enqueue(String::from("element5"));
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 2);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 3);

        vc.enqueue(String::from("element6"));
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 3);
        assert_eq!(vc.rear_index, 2);
        assert_eq!(vc.size, 3);

        vc.enqueue(String::from("element7"));
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 3);
        assert_eq!(vc.size, 3);
    }

    #[test]
    fn dequeue_on_queue_capacity_zero() {
        let mut vc: Circular<String> = Circular::new(0);

        assert_eq!(None, vc.dequeue());
    }

    #[test]
    fn dequeue_one_element() {
        let mut vc: Circular<String> = Circular::new(1);

        vc.enqueue(String::from("element1"));

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element1")),
            None => panic!("Element should not be None!"),
        }

        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 1);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn dequeue_multiple_elements() {
        let mut vc: Circular<String> = Circular::new(5);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));
        vc.enqueue(String::from("element3"));
        vc.enqueue(String::from("element4"));
        vc.enqueue(String::from("element5"));

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element1")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 1);
        assert_eq!(vc.rear_index, 5);
        assert_eq!(vc.size, 4);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element2")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 2);
        assert_eq!(vc.rear_index, 5);
        assert_eq!(vc.size, 3);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element3")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 3);
        assert_eq!(vc.rear_index, 5);
        assert_eq!(vc.size, 2);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element4")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 4);
        assert_eq!(vc.rear_index, 5);
        assert_eq!(vc.size, 1);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element5")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 5);
        assert_eq!(vc.rear_index, 5);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn dequeue_when_rear_smaller_than_front_index() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));
        vc.enqueue(String::from("element3"));
        vc.enqueue(String::from("element4"));

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element3")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 1);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element4")),
            None => panic!("Element should not be None!"),
        }
        assert_eq!(vc.push_enabled, false);
        assert_eq!(vc.front_index, 1);
        assert_eq!(vc.rear_index, 1);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn enqueue_dequeue_of_primitive_data() {
        let mut vc: Circular<i32> = Circular::new(2);

        vc.enqueue(1);
        vc.enqueue(2);

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, 1),
            None => panic!("Element should not be None!"),
        }

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, 2),
            None => panic!("Element should not be None!"),
        }
    }

    #[test]
    fn clear_circular_queue() {
        let mut vc: Circular<String> = Circular::new(5);

        vc.clear();
        assert_eq!(vc.capacity, 6);
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn clear_queue_with_capacity_zero() {
        let mut vc: Circular<String> = Circular::new(0);

        vc.clear();
        assert_eq!(vc.capacity, 1);
        assert_eq!(vc.push_enabled, true);
        assert_eq!(vc.front_index, 0);
        assert_eq!(vc.rear_index, 0);
        assert_eq!(vc.size, 0);
    }

    #[test]
    fn index_trait() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        assert_eq!(*vc[0], String::from("element1"));
        assert_eq!(*vc[1], String::from("element2"));
    }

    #[test]
    fn index_trait_rear_before_front() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));
        vc.enqueue(String::from("element3"));
        vc.enqueue(String::from("element4"));

        assert_eq!(*vc[0], String::from("element3"));
        assert_eq!(*vc[1], String::from("element4"));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_trait_out_of_bounds() {
        let vc: Circular<String> = Circular::new(0);

        &vc[0];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_trait_out_of_bounds_1() {
        let vc: Circular<String> = Circular::new(1);

        &vc[0];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_trait_out_of_bounds_2() {
        let mut vc: Circular<String> = Circular::new(1);

        vc.enqueue(String::from("element1"));

        &vc[1];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_trait_out_of_bounds_3() {
        let mut vc: Circular<String> = Circular::new(3);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));
        vc.enqueue(String::from("element3"));

        &vc[3];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_trait_out_of_bounds_rear_before_front_index() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));
        vc.enqueue(String::from("element3"));
        vc.enqueue(String::from("element4"));

        &vc[3];
    }

    #[test]
    fn mut_index_trait() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        vc[0] = String::from("element3");
        vc[1] = String::from("element4");

        assert_eq!(*vc[0], String::from("element3"));
        assert_eq!(*vc[1], String::from("element4"));
    }

    #[test]
    fn mut_index_trait_dequeue() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        vc[0] = String::from("element3");
        vc[1] = String::from("element4");

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element3")),
            None => panic!("Element should not be None!"),
        }

        match vc.dequeue() {
            Some(elem) => assert_eq!(*elem, String::from("element4")),
            None => panic!("Element should not be None!"),
        }
    }

    #[test]
    fn iterator_trait() {
        let mut vc: Circular<String> = Circular::new(2);

        let template = vec!["element1", "element2"];
        let mut index = 0;

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        for item in &vc {
            assert_eq!(item, template[index]);
            index += 1;
        }
    }

    #[test]
    fn iterator_trait_on_empty_queue() {
        let vc: Circular<String> = Circular::new(2);

        for _ in &vc {
            panic!("Loop should not get executed");
        }
    }

    fn all_caps(text: &String) -> String {
        return text.to_uppercase();
    }

    #[test]
    fn apply_map() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        vc.map(all_caps);

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, String::from("ELEMENT1")),
            None => panic!("Data must not be None"),
        }

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, String::from("ELEMENT2")),
            None => panic!("Data must not be None"),
        }
    }

    fn plus_one(num: &usize) -> usize {
        return *num + 1;
    }

    #[test]
    fn apply_map_primitive_data() {
        let mut vc: Circular<usize> = Circular::new(2);
        vc.enqueue(1);
        vc.enqueue(2);

        vc.map(plus_one);

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, 2),
            None => panic!("Data must not be None"),
        }

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, 3),
            None => panic!("Data must not be None"),
        }
    }

    #[test]
    fn apply_map_closure() {
        let mut vc: Circular<String> = Circular::new(2);

        vc.enqueue(String::from("element1"));
        vc.enqueue(String::from("element2"));

        vc.map_closure(|text: &String| -> String { text.to_uppercase() });

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, String::from("ELEMENT1")),
            None => panic!("Data must not be None"),
        }

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, String::from("ELEMENT2")),
            None => panic!("Data must not be None"),
        }
    }

    #[test]
    fn apply_map_closure_primitive_data() {
        let mut vc: Circular<usize> = Circular::new(2);
        vc.enqueue(1);
        vc.enqueue(2);

        vc.map_closure(|num: &usize| -> usize { num + 1 });

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, 2),
            None => panic!("Data must not be None"),
        }

        match vc.dequeue() {
            Some(data) => assert_eq!(*data, 3),
            None => panic!("Data must not be None"),
        }
    }
}
