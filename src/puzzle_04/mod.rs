pub fn overflow() {
    let mut counter = 0_i8;
    loop {
        if let Some(result) = counter.checked_add(1) {
            counter = result;
        } else {
            counter = 0;
        }
        println!("{counter}");
    }
}
