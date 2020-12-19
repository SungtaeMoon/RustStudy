fn main() {
    let mut x = vec!["This", "is"];
    print!("{}", x.len());
    
    x.push("a");
    print!(" {}", x.len());
    
    x.push("sentence");
    print!(" {}", x.len());

    for i in 0..x.len() {
        print!(" {}", x[i]);
    }

    print!("\n");

    let length = 5000;
    let mut y = vec![4.; length];  //5000번째. vector 5000개 생성.
    y[6] = 3.14;
    y.push(4.89);
    print!("{}, {}, {}\n", y[6], y[4999], y[5000]);

    let mut _x = vec!["a", "b", "c"];
    println!("bef: {:?}", _x);
    _x = vec!["X", "Y"];
    println!("aft: {:?}", _x);

    let mut x = vec!["This", "is", "a", "sentence"];
    //x.insert("line", 1);  //Error
    x.insert(1, "line");
    x.insert(2, "contains");
    x.remove(3);
    x.push("about Rust");
    x.pop();
    for i in 0..x.len() {
        print!("{} ", x[i]);
    }

    let mut _x = vec![12, 13, 14, 15];
    _x.insert(3, 1);
    print!("{:?}", _x);
}
