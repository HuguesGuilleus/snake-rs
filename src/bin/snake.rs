fn main() {
    println!("Hello, world!");

    for _ in 0..5 {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_millis(200));
        println!("yolo!");
    }
}
