use std::{cmp, pin::Pin, ptr::NonNull};

use crate::lox_value::LoxValue;

#[derive(Debug)]
#[repr(transparent)]
pub struct Sp<const STACK_SIZE: usize> {
    ptr: NonNull<LoxValue>,
}

impl<const STACK_SIZE: usize> Sp<STACK_SIZE> {
    pub unsafe fn create(stack: &mut Stack<STACK_SIZE>) -> Sp<STACK_SIZE> {
        let ptr = stack.bytes.as_mut_ptr();
        assert!(ptr != std::ptr::null_mut());
        Sp {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}

impl<const STACK_SIZE: usize> Sp<STACK_SIZE> {
    #[inline(always)]
    pub fn get_value(&self) -> LoxValue {
        unsafe { *self.ptr.as_ref() }
    }

    #[inline(always)]
    pub fn write_value(&mut self, value: &LoxValue) {
        unsafe { *self.ptr.as_mut() = value.clone() };
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

impl<const STACK_SIZE: usize> PartialEq for Sp<STACK_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        return self.ptr == other.ptr;
    }
}

impl<const STACK_SIZE: usize> PartialOrd for Sp<STACK_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.ptr.partial_cmp(&other.ptr)
    }
}

impl<const STACK_SIZE: usize> Clone for Sp<STACK_SIZE> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

#[derive(Debug)]
pub struct Stack<const STACK_SIZE: usize> {
    bytes: Pin<Box<[LoxValue; STACK_SIZE]>>,
}

impl<const STACK_SIZE: usize> Stack<STACK_SIZE> {
    pub fn new() -> Stack<STACK_SIZE> {
        let heap_vec = vec![LoxValue::default(); STACK_SIZE];
        let heap_array: Box<[LoxValue; STACK_SIZE]> = heap_vec.try_into().unwrap();
        Self {
            bytes: Box::into_pin(heap_array),
        }
    }

    pub fn get_base_sp(&mut self) -> Sp<STACK_SIZE> {
        unsafe { Sp::<STACK_SIZE>::create(self) }
    }

    pub fn get_stack_iterator(&mut self, up_to: Sp<STACK_SIZE>) -> StackIterator<STACK_SIZE> {
        StackIterator::new(self, &up_to)
    }
}

pub struct StackIterator<const STACK_SIZE: usize> {
    curr: Sp<STACK_SIZE>,
    top: Sp<STACK_SIZE>,
}

impl<const STACK_SIZE: usize> StackIterator<STACK_SIZE> {
    pub fn new(stack: &mut Stack<STACK_SIZE>, sp: &Sp<STACK_SIZE>) -> Self {
        let base = stack.get_base_sp();
        let top = sp.clone();

        Self {
            curr: base,
            top: top,
        }
    }
}

impl<const STACK_SIZE: usize> Iterator for StackIterator<STACK_SIZE> {
    type Item = LoxValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.top {
            let value = self.curr.get_value();
            self.curr.inc(1);
            return Some(value);
        }
        None
    }
}
