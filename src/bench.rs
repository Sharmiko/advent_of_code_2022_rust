use took::Timer;
use crate::calorie_counting;


pub fn main() {
    let timer = Timer::new();
    calorie_counting::main_part01();
    println!("Calorie counting took: {:?}", timer.took().into_std().as_secs_f32());
}