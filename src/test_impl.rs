struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut v = vec![1, 2, 3];
    let mut nums: Vec<i32> = vec![1, 2, 5, 9, 5, 9, 5, 5, 5];
    print!("&v        ");
    for x in &v {
        print!("{} ", x);
    }

    for x in &v {
        print!("{} ", x);
    }

    for num in nums {
        print!("{} ", num);
    }


    for num in nums {
        print!("{} ", num);
    }


    println!(" ");
    print!("len  get  ");
    for x in 0..v.len() {
        if let Some(num) = v.get(x) {
            print!("{} ", num);
        }
    }
    println!(" ");
    print!("iter      ");
    for x in v.iter() {
        print!("{} ", x);
    }
    println!(" ");
    print!("into_iter ");
    for x in v.into_iter() {
        print!("{} ", x);
    }
    println!(" ");

}