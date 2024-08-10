use std::{cmp, pin::Pin};

use crate::{
    lox_value::LoxValue,
    states::{Initialized, State, Uninitialized},
};

#[derive(Debug)]
pub struct Sp<S: State, const StackSize: usize> {
    ptr: *mut LoxValue,
    state: S,
}

impl<S: State, const StackSize: usize> Sp<S, StackSize> {
    pub unsafe fn create(stack: &mut Stack<StackSize>) -> Sp<Initialized, StackSize> {
        let ptr = stack.bytes.as_mut_ptr();
        assert!(ptr != std::ptr::null_mut());
        Sp {
            ptr: ptr,
            state: Initialized,
        }
    }

    pub fn create_uninitialized() -> Sp<Uninitialized, StackSize> {
        Sp {
            ptr: std::ptr::null_mut(),
            state: Uninitialized,
        }
    }
}

impl<const StackSize: usize> Sp<Initialized, StackSize> {
    #[inline(always)]
    pub fn get_value(&self) -> LoxValue {
        unsafe { *self.ptr }
    }

    #[inline(always)]
    pub fn write_value(&mut self, value: &LoxValue) {
        unsafe { *self.ptr = *value };
    }

    #[inline(always)]
    pub fn read_value(&self) -> LoxValue {
        unsafe { *self.ptr }
    }

    #[inline(always)]
    pub fn inc(&mut self, offset: usize) {
        unsafe { self.ptr = self.ptr.add(offset) };
    }

    #[inline(always)]
    pub fn dec(&mut self, offset: usize) {
        unsafe { self.ptr = self.ptr.sub(offset) };
    }
}

impl<const StackSize: usize> PartialEq for Sp<Initialized, StackSize> {
    fn eq(&self, other: &Self) -> bool {
        return self.ptr == other.ptr;
    }
}

impl<const StackSize: usize> PartialOrd for Sp<Initialized, StackSize> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.ptr.partial_cmp(&other.ptr)
    }
}

impl<const StackSize: usize> Clone for Sp<Initialized, StackSize> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            state: Initialized,
        }
    }
}

#[derive(Debug)]
pub struct Stack<const StackSize: usize> {
    bytes: Pin<Box<[LoxValue; StackSize]>>,
}

impl<const StackSize: usize> Stack<StackSize> {
    pub fn new() -> Stack<StackSize> {
        let heap_vec = vec![LoxValue::default(); StackSize];
        let heap_array: Box<[LoxValue; StackSize]> = heap_vec.try_into().unwrap();
        Self {
            bytes: Box::into_pin(heap_array),
        }
    }

    pub fn get_base_sp(&mut self) -> Sp<Initialized, StackSize> {
        unsafe { Sp::<Initialized, StackSize>::create(self) }
    }

    pub fn get_stack_iterator(
        &mut self,
        up_to: Sp<Initialized, StackSize>,
    ) -> StackIterator<StackSize> {
        StackIterator::new(self, &up_to)
    }
}

pub struct StackIterator<const StackSize: usize> {
    curr: Sp<Initialized, StackSize>,
    top: Sp<Initialized, StackSize>,
}

impl<const StackSize: usize> StackIterator<StackSize> {
    pub fn new(stack: &mut Stack<StackSize>, sp: &Sp<Initialized, StackSize>) -> Self {
        let base = stack.get_base_sp();
        let top = sp.clone();

        Self {
            curr: base,
            top: top,
        }
    }
}

impl<const StackSize: usize> Iterator for StackIterator<StackSize> {
    type Item = LoxValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.top {
            let value = self.curr.get_value();
            self.curr.inc(1);
            return Some(value);
        }
        return None;
    }
}
