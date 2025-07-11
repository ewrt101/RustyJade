#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Ensure bindings.rs contains the declaration for paramSetInteger
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


#[unsafe(no_mangle)]
// DLL export with same signature as original
pub extern "system" fn sanity(
    pbuffer: *mut DskBuffer,
    pParams: *mut DskParam,
    pReturn: *mut DskParam,
) -> i32 {
    
    // Return a 1 to Jade
    //unsafe {paramSetInteger(pReturn, 1, 0) }
    return 100;
}

#[unsafe(no_mangle)]
// DLL export with same signature as original
pub extern "system" fn add_two_numbers(
    pbuffer: *mut DskBuffer,
    pParams: *mut DskParam,
    pReturn: *mut DskParam,
) -> i32 {
    
    //empty pointer for holding the first parameter
    let mut holder1: *mut DskParam = std::ptr::null_mut();
    let mut holder2: *mut DskParam = std::ptr::null_mut();

    unsafe { paramGetParameter(pParams, 1, &mut holder1) };
    unsafe { paramGetParameter(pParams, 2, &mut holder2) };

    //var for holding the two numbers
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    unsafe { paramGetInteger(holder1, &mut num1 as *mut i32) };
    unsafe { paramGetInteger(holder2, &mut num2 as *mut i32) };
    
    //unsafe {paramSetInteger(pReturn, 1, 0) }
    return num1 + num2;
}



/*
#[unsafe(no_mangle)]
// DLL export with same signature as original
pub extern "C" fn PassOneParam(
    pbuffer: *mut DskBuffer,
    pParams: *mut DskParam,
    pReturn: *mut DskParam,
) -> i32 {
    let mut my_param: *mut Character = std::ptr::null_mut();
    // Replace 'param_get_string' with the correct function name from bindings.rs
    let result = unsafe {paramGetString(pParams, &mut my_param) };
    if result != 0 {
        // CHECK_RESULT: return error code if paramGetString failed
        return result;
    }
    // Return a 1 to Jade
    unsafe {paramSetInteger(pReturn, 1) }
}
*/
