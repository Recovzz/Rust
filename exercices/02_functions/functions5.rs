// J'ai retiré le ; à la fin du num * num ce qui fait de cette expression une instruction et non une expression retournant une valeur. 

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
