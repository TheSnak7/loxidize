use std::{cmp, pin::Pin, ptr::NonNull};

use crate::lox_value::LoxValue;

#[derive(Debug)]
pub struct Sp<const StackSize: usize> {
    ptr: NonNull<LoxValue>,
}

impl<const StackSize: usize> Sp<StackSize> {
    pub unsafe fn create(stack: &mut Stack<StackSize>) -> Sp<StackSize> {
        let ptr = stack.bytes.as_mut_ptr();
        assert!(ptr != std::ptr::null_mut());
        Sp {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}

impl<const StackSize: usize> Sp<StackSize> {
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

impl<const StackSize: usize> PartialEq for Sp<StackSize> {
    fn eq(&self, other: &Self) -> bool {
        return self.ptr == other.ptr;
    }
}

impl<const StackSize: usize> PartialOrd for Sp<StackSize> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.ptr.partial_cmp(&other.ptr)
    }
}

impl<const StackSize: usize> Clone for Sp<StackSize> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
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

    pub fn get_base_sp(&mut self) -> Sp<StackSize> {
        unsafe { Sp::<StackSize>::create(self) }
    }

    pub fn get_stack_iterator(&mut self, up_to: Sp<StackSize>) -> StackIterator<StackSize> {
        StackIterator::new(self, &up_to)
    }
}

pub struct StackIterator<const StackSize: usize> {
    curr: Sp<StackSize>,
    top: Sp<StackSize>,
}

impl<const StackSize: usize> StackIterator<StackSize> {
    pub fn new(stack: &mut Stack<StackSize>, sp: &Sp<StackSize>) -> Self {
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
