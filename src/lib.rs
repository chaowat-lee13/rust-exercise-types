pub mod lib {

    pub fn print_difference(coords: (f32, f32)) {
        let (first, second): (f32, f32) = coords;
        println!("First Index: {}\nSecond Index: {}", first, second);
    }

    pub fn print_array(coords: (f32, f32)) {
        let (first, second): (f32, f32) = coords;
        let arr: [f32; 2] = [first, second];
        print!("Turns a Tuple into Array: {:?}\n", arr)
    }

    pub fn ding(arr: [i32; 7]) {
        let last_element: i32 = arr.to_vec().pop().unwrap();
        println!("Last Element of Array: {}", last_element)
    }

    pub fn on_off(val: bool) {
        if val {
            println!("Lights are on!");
        }
    }
    pub fn print_distance((x, y): (f32, f32)) {
        // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
        // us, Rust supports destructuring function arguments.  Try replacing "z" in
        // the parameter list above with "(x, y)" and then adjust the function
        // body to use x and y.
        println!(
            "Distance to the origin is {}",
            (x.powf(2.0) + y.powf(2.0)).sqrt()
        );
    }
}
