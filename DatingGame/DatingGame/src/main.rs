#[derive(Debug, Clone)]
enum Card {
    King,
    Queen,
}

#[derive(Debug, Clone)]
struct Deck(Vec<Card>);



fn main () {
    let alice = Deck(vec![Card::King, Card::Queen]);
    let bob = Deck(vec![Card::King, Card::Queen]);
    let separator = Deck(vec![Card::King]);
    let join = Deck([alice.0.clone(), separator.0.clone(), bob.0.clone()].concat());
    println!("Fuck");
    println!("{:?}", join.0);
}