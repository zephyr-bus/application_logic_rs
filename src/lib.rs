/* Standard library not supported in our firmware project */
#![no_std]
#![no_builtins]

extern crate panic_halt;

/* Needed to perform the division */
use core::ops::Div;

extern "C" {
    pub fn printk(format: *const u8, ...);
    pub fn rs_msleep(ms: i32);
}
macro_rules! k_msleep_unsafe {
    ($e:expr) => {
        unsafe {
            rs_msleep($e);
        }
    };
}

macro_rules! printk_unsafe {
	($string:literal) => {
		unsafe {
			printk($string.as_ptr());
		}
	};
	($format:expr, $($e:expr),+) => {
		unsafe {
			printk($format.as_ptr(), $($e),+);
		}

	};
}

#[no_mangle]
pub extern "C" fn smean_rs(scaling: i32, ptr: *const i32, size: usize) -> i32 {
    /* Create a slice out of the C array using pointer to first element */
    let array;
    unsafe {
        array = core::slice::from_raw_parts(ptr, size);
    }
    let mean_func = smean(scaling, array);
    mean_func
}

pub fn smean(scaling: i32, array: &[i32]) -> i32 {
    let mean_func = array
        .iter()
        .map(|x| *x * scaling)
        .fold(0_i32, |sum, x| sum + x)
        .div(array.len() as i32);
    mean_func
}

#[no_mangle]
pub extern "C" fn application_logic() {
    printk_unsafe!("Hello World from rust!\n");
    let t = [2, 2, 2, 2, 2];
    loop {
        let mean = smean(1, &t);
        printk_unsafe!("smean = %d\n", mean);
        k_msleep_unsafe!(1000);
    }
}
