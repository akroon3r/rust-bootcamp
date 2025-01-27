// Declare a `TOUCHDOWN_POINTS` constant at the file leverl set to the value of 6
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    // Declare a `season` variable set to a string with your favourite season
    let season: &str = "Fall";
    // Declare a `points_scored` variable set to 28 provide explicity type annotation of integer i32
    // Declare the `points scored` variable to be mutable.
    let mut _points_scored: i32 = 28;
    // Set `points_scored` to a new value of 35
    let _points_scored = 35;
    // Declare an `event_time` variable set to a string of "06:00"
    let _event_time: &str = "06:00";
    // Use interpolation to print out all of the
    // declared vaiables and constants in a println! call.
    // Practice using the direct interpolation {value}, sequential arguments ( {} ), and numeric arguments
    
    // Direct
    println!("My favourite season is {season}");

    // Sequential
    println!("My favourite season is {}, and in football a touchdown is worth {}", season, TOUCHDOWN_POINTS);

    // Numeric
    println!("My favoute sport is football. In football, a touch down is worth {0} points. Football is generally played in the {1}", TOUCHDOWN_POINTS, season);

    // Declare a `favourite_beverage` variable set to a string of your favourite drink.
    // Use an underscore to silence the compiler warning about the variable being unused.
    // let _favourite_beverage: str = 'Cherry Coke';

    // Remove the underscore. Provide the compiler directive to silence the compiler warning about the variable being unused
    #[allow(unused)]
    let favourite_beverage: &str = "Cherry Coke";

}
