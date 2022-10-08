use std::collections::BTreeMap;

struct MyCalendarThree {
    data: BTreeMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.data.entry(start).or_default() += 1;
        *self.data.entry(end).or_default() -= 1;
        let mut ans = 0;
        let mut prev = 0;
        for event in self.data.values() {
            prev += event;
            ans = ans.max(prev);
        }
        ans
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_732() {
        let mut calendar = MyCalendarThree::new();
        assert_eq!(calendar.book(10, 20), 1);
        assert_eq!(calendar.book(50, 60), 1);
        assert_eq!(calendar.book(10, 40), 2);
        assert_eq!(calendar.book(5, 15), 3);
        assert_eq!(calendar.book(5, 10), 3);
        assert_eq!(calendar.book(25, 55), 3);
    }
}
