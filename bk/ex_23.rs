fn main() {
 let twenty = 20;
 let twenty_one: i32 = 21;
 let tewnty_two = 22i32;

 let addition = twenty + twenty_one + tewnty_two;
 println!("{} + {} + {} = {}", twenty, twenty_one, tewnty_two, addition);

 let one_million:i64 = 1_000_000;
 println!("{}", one_million.pow(2));

 let forty_twos = [
     42.0,
     42f32,
     42.0_f32,
 ];
 println!("{:02}", forty_twos[0]);
}
