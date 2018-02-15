extern crate rand;
use rand::Rng;

fn main() {

    print!("Give number: ");
    println!("");
    let secret_number = rand::thread_rng().gen_range(1,101);

loop{
    let mut guess: String = String::new(); // mut - can change value(mutable for const from c? lol)
//    let static_num: u32 = 667; // 
//    print!("{}\n",static_num);
    std::io::stdin().read_line(&mut guess).expect("Cannot read from stdin");

    let guess: i32 = 
    	match guess.trim().parse() {
		Ok(rn) => rn,
		Err(_) =>{
			 println!("Введите число!");
			 continue;
			},
    	};
    
    match guess.cmp(&secret_number){
         std::cmp::Ordering::Less => println!("{} Меньше чем нужное число", guess ),
         std::cmp::Ordering::Equal => {
		println!("Вы угадали");
		break;
	},
         std::cmp::Ordering::Greater => println!("{} Является больше чем нужное число", guess)
    }
}
  
//    panic!("Test");
}
