//! RWLock 

extern crate alloc;
use alloc::boxed::Box;

extern crate core;
use core::marker::Sync;
use core::cell::UnsafeCell;
//use core::marker::Sized;
use core::ops::Deref;

enum ReferenceType {
    NULL,
    Read,
    Write,
}

pub struct RwLockWriteGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
    //poison: poison::Guard,
}

impl<'rwlock, T: ?Sized> RwLockWriteGuard<'rwlock, T> {
    unsafe fn new(lock: &'rwlock RwLock<T>) -> Result<RwLockWriteGuard<'rwlock, T>, &str> {
        Ok(RwLockWriteGuard{lock: lock, }) //NG未実装
    }
}

impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

pub struct RwLock<T: ?Sized> {
    inner: Box<SRWLock>,
    data: UnsafeCell<T>,
    //poison: poison::Flag,
}

impl<T> RwLock<T> {

    pub fn new(t: T) -> RwLock<T> {
        RwLock {
            inner: Box::new(SRWLock::new()),
            data: UnsafeCell::new(t),
            //poison: poison::Flag::new(),
        }
    }
}

impl<T: ?Sized> RwLock<T> {

    #[inline]
    pub fn read(&self) -> Result<&RwLock<T>, &str> {
        unsafe {
            //self.inner.read()?;
            Ok(&self)
            //RwLockReadGuard::new(self)
        }
    }

    #[inline]
    pub fn write(&self) -> Result<RwLockWriteGuard<'_, T>, &str> {
        unsafe {
            //self.inner.write()?;
            RwLockWriteGuard::new(self)
        }
    }
}

//unsafe impl<#[may_dangle] T: ?Sized> Drop for RwLock<T> {
    /*
unsafe impl<T: ?Sized> Drop for RwLock<T> {
    fn drop(&mut self) {
        unsafe { /* self.inner.destroy() */ }
    }
}
*/

struct SRWLock {
    RRefCount: u32,
    WRefCount: u32,
}

impl SRWLock {
    pub fn new() -> Self {
        SRWLock {RRefCount: 0, WRefCount: 0, }
    }

    pub fn read(&mut self) -> Result<i32, &str> {
        if self.WRefCount >= 1 {
            Err("Already Write Referenced")
        } else {
            self.RRefCount += 1;
            Ok(0)
        }
    }
    pub fn write(&mut self) -> Result<i32, &str> {
        if self.WRefCount >= 1 || self.RRefCount >= 1 {
            Err("Already Referenced")
        } else {
            self.WRefCount += 1;
            Ok(0)
        }
    }
    pub fn release_r_ref(&mut self) -> Result<i32, &str> {
        if self.RRefCount <= 0 {
            Err("Already Released")
        } else {
            self.RRefCount -= 1;
            Ok(0)
        }
    }
    pub fn release_w_ref(&mut self) -> Result<i32, &str> {
        if self.WRefCount <= 0 {
            Err("Already Released")
        } else {
            self.WRefCount -= 1;
            Ok(0)
        }
    }
}


/* 
pub struct RwLockReadGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
}

unsafe impl<T: ?Sized + Sync> Sync for RwLockReadGuard<'_, T> {}

pub struct RwLockWriteGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
    poison: poison::Guard,
}

unsafe impl<T: ?Sized + Sync> Sync for RwLockWriteGuard<'_, T> {}
 */
