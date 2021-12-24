/**
 * In Rust, all elements in an Array must be
 * of the same type.
 * 
 * Given these limitations, Arrays in Rust
 * are useful for lists with known sizes and 
 * elements of an homogenous type.
*/

fn main() {
    let spidermen = ["Tobey Maguire", "Andrew Garfield", "Tom Holland"];
    println!("spidermen: {:?}", spidermen);

    let top_scores: [u32; 3] = [292, 170, 140];
    println!("top_scores: {:?}", top_scores);
    println!("top_scores.len(): {}", top_scores.len());

    let mut product = ("iPhone 12 Pro Max", 1099, true);
    println!("initial: {:?}", product);
    product = ("PS5", 499, false);
    // not allowed: product = ("PS5", 499.99, false);
    println!("after change: {:?}", product);
    
    let [my_score, _, _] = top_scores;
    println!("my_score: {}", my_score);
    let (_, _, is_available) = product;
    println!("is_available: {}", is_available);
}