pub fn overflow() {
    let mut counter: i8 = 0;
    loop {
        // crash with 'attempt to add with overflow' on debug build.
        counter += 1;
        println!("{counter}");
    }
}
