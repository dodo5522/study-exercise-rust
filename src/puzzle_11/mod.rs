pub fn allocate() -> usize {
    // fatal runtime error: stack overflow, aborting (with debug build)
    let a = Box::new([0u32; 10000000]);
    a.len()
}
