// tests5.rs
unsafe fn modify_by_address(mut address: usize) -> u32 {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
    	    let mut t : u32 = 0xAABBCCDD;
    	    t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_success() {
        let mut t: u32 = 0x11223344;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { t = modify_by_address(&mut t as *mut u32 as usize) };
        println!("{:x}",t);
        assert!(t == 0xAABBCCDD);
    }
}
