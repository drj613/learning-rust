fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("yeet fam * {}", counter);

        if counter == 10 {
            break;
        }
    }
}
