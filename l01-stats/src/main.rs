fn main() {
    println!("Hello, world!");
}

fn min_max(input: &[i32]) -> (i32,i32){
    let min: i32 = 0;
    let max: i32 = 0;
    for v in input {

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
