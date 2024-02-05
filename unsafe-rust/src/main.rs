use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // 1. Dereferencing a raw pointer
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //2. Calling Unsafe function or method
    unsafe fn dangerous() {

    }

    unsafe {
        dangerous();
    }

    // Creating a safe abstraction over unsafe code

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using extern Functions to Call External code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 3.) Access or modify a mutable static variable
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // -------------------------------

    let u = MyUnion {f1: 10};

    let f = unsafe {u.f1};
    println!("Accessing 1 field of a union {}", f);

}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 4.) Implement an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// 5.) Accessing the fields of union
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}


