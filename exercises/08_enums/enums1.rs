#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Echo,
    Move,
    Resize,
    ChangeColor,
    Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
