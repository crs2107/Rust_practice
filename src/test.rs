
fn main(){
  let x = check_prime(2);
  println!("{}",x);
}

pub fn check_prime(x : u32) -> bool{
    let root = (x as f32).sqrt() as u32;
    
    for i in 2..(root+1) {
        if x%i == 0{
          return false;
        }
    }
    true

}
