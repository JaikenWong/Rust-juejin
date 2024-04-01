
fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("final sentence: {sentence}");
    println!("{:?}", &sentence[0..5]);
    //println!("{:?}", &sentence[12..13]);
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);

    let n = 4;
    println!("{n}! = {}", factorial(n));
    fizzbuzz(3);

}


fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

#[test]
fn reference() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}