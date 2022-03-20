 
fn main(){
    let a :u128 = 1000000;
    let mut b :u128 = 1;
    while b <= a{
        let mut c :u128 = b;
        let mut check :bool = false;
        while !check{
            if c % 2 == 0{
                c = c / 2;
            }
            else{
                c = c * 3 + 1;
            }
            if c == 4 || c == 2 || c == 4{
                println!("number {} check", b);
                check = true;
            }
        }
        b = b + 1;
    }
}
