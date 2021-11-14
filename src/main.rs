use std::io;
fn main() {
    println!("Input the number : ");
    let mut num = String :: new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let n:i32=num.trim().parse().expect("error ");

    if check_prime(n.try_into().unwrap()) == true {
        println!("No such pair exists");
        return ();
    }


    let  left_mid:i32;
    
    if n%2 == 0 {
        left_mid = n/2 ;
        
        }
    else {
        left_mid = (n-1)/2;
    }
    let mut flag = 0;

    for i in 1..left_mid  {
        if check_prime(i.try_into().unwrap())==true {
            let j:u32 = (n-i).try_into().unwrap();
            if check_prime(j)==true{
                println!("({},{})",i,j);
                flag = 1;
            }
        }
    }
    if flag == 0 {
        println!("No such pair exists");
   }

}

pub fn check_prime(x : u32) -> bool {
    let root = (x as f32).sqrt() as u32;
    
    for i in 2..(root+1) {
        if x%i == 0{
          return false;
        }
    }
    true

}
