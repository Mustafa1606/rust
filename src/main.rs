fn add(x: i32, y: i32) -> i32 {
	x+y
}

fn diverges() -> ! {
    panic!("Эта функция не возвращает управление!");
}

fn main() {
    println!("Арсен кончай ломаться!");
	let x = 5;
	let (y,z) = (2,3);
	let b: i32 = 546;
	let mut g: i32 = 4897;
	g = 7894;
	println!("g = {}",g);
	g = 9871;
	let z = add;
	println!("g = {}",g);
	println!("k = {}",z(x,y));
	let r: char = '💕';
	let h: bool = true;
	let c: [u8;3] = [1,2,3];
	println!("r,h = {}, {}, {}",r,h,c[0]);
	//diverges();
}