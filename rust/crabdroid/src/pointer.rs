use std::{sync::Arc, marker::PhantomData};

pub struct ArcPointer<T: ?Sized> {
    long: i64,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> ArcPointer<T> {
    pub fn new(arc: Arc<T>) -> Self {
        let p = Box::into_raw(Box::new(arc));
        Self {
            long: p as *const () as i64,
            phantom: PhantomData
        }
        
    }

    pub fn of(long: i64) -> Self {
        Self { long: long, phantom: PhantomData }
    }

    pub fn arc(&self) -> Arc<T> {
        let p = self.long as *mut Arc<T>;
        let arc = Box::leak(unsafe { Box::from_raw(p) });
        Arc::clone(arc)
    }

    pub fn destroy(mut self) {
        let p = self.long as *mut Arc<T>;
        self.long = 0;
         let b = unsafe{Box::from_raw(p)};
         drop(b);
    }
}

impl<T: ?Sized> From<ArcPointer<T>> for i64 {
    fn from(p: ArcPointer<T>) -> Self {
        p.long
    }
}