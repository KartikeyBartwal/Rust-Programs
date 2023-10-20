fn main() {
    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to 'y'  (no semicolon means this is going to be the value for y)
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and '()' is assigned to 'z'  
        2 * x;

        // This expression will be assigned to 'z'  (no semicolon means this is going to be the value for z)
        2 * x
    };

    println!("x is {:?}" , x);
    println!("y is {:?}" , y);
    println!("z is {:?}" , z);
}