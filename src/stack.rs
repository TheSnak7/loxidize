use std::pin::Pin;

use crate::{
    lox_value::LoxValue,
    states::{Initialized, Uninitialized},
};

pub struct Sp<S, const StackSize: usize> {
    ptr: *mut LoxValue,
    state: S,
}

impl<S, const StackSize: usize> Sp<S, StackSize> {
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
}
