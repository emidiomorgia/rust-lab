fn main() {
    println!("Hello, world!");
    let input = [1,2,3];
    let (min,max) = min_max(&input);
    println!("Test su max e min: {},{}",min,max);
}

fn min_max(input: &[i32]) -> (i32,i32){
    let mut min: i32 = i32::MAX;
    let mut max: i32 = i32::MIN;
    for v in input {
        if *v < min {
            min = *v;
        }
        if *v > max {
            max = *v;
        }
    }

    return (min,max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_simple(){
        let input: [i32;3]=[1,2,3];
        let (min,max) = min_max(&input);
        assert_eq!(min,1);
        assert_eq!(max,3);
    }
}
