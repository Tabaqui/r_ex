const SIZE: usize = 32;

pub struct MyVec<T> {
    data: [Option<T>; SIZE], // упрощение: capacity = 32
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        let a = [const { None }; 32];
        Self { data: a, len: 0 }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= SIZE {
            return Err(value);
        }

        self.data[self.len] = Some(value);
        self.len += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        self.data[self.len].take()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            return None;
        }

        self.data[idx].as_ref()
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.len {
            return None;
        }

        self.data[idx].as_mut()
    }

    pub fn iter(&self) -> MyVecIter<'_, T> {
        MyVecIter { vec: self, pos: 0 }
    }

    pub fn clear(&mut self) {
        self.data = [const { None }; SIZE];
        self.len = 0;
    }
}

pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,
    pos: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > SIZE {
            return None;
        }

        let next = self.vec.get(self.pos);
        self.pos += 1;

        next
    }
}

#[test]
fn push_pop_basic() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1).unwrap();
    v.push(2).unwrap();
    v.push(3).unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v.pop(), Some(2));
    assert_eq!(v.len(), 1);
}

#[test]
fn iter_borrows() {
    let mut v: MyVec<String> = MyVec::new();
    v.push("a".into()).unwrap();
    v.push("b".into()).unwrap();
    let collected: Vec<&String> = v.iter().collect();
    assert_eq!(collected.len(), 2);
}

#[test]
fn capacity_overflow() {
    let mut v: MyVec<i32> = MyVec::new();
    for i in 0..32 {
        v.push(i).unwrap();
    }
    assert!(v.push(99).is_err());
}

#[test]
fn get_multiple_some() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1).unwrap();
    v.push(2).unwrap();
    assert_eq!(v.len(), 2);
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.get(1), Some(&2));
    assert_eq!(v.len(), 2);
}

#[test]
fn get_none() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1).unwrap();
    v.push(2).unwrap();
    assert_eq!(v.len(), 2);
    assert_eq!(v.get(3), None);
    assert_eq!(v.len(), 2);
}

#[test]
fn mutate_get() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1).unwrap();
    v.push(3).unwrap();
    if let Some(a) = v.get_mut(1) {
        *a = 2;
    };
    assert_eq!(v.pop(), Some(2));
}

#[test]
fn cleared() {
    let mut v: MyVec<i32> = MyVec::new();
    v.push(1).unwrap();
    v.push(2).unwrap();
    v.clear();
    assert_eq!(v.len(), 0);
}