fn main() {
    println!("Hello, world!");
    let input = [1,2,3];
    let (min,max) = min_max(&input);
    println!("Test su max e min: {},{}",min,max);
    let sum = sum_wide(&input);
    println!("Test su sum_wide: {}",sum);
    let avg = mean(&input);
    println!("Test su mean: {}",avg);   
}

fn min_max(input: &[i32]) -> (i32,i32){
    let mut min: i32 = i32::MAX;
    let mut max: i32 = i32::MIN;
    for v in input.iter() {
        if *v < min {
            min = *v;
        }
        if *v > max {
            max = *v;
        }
    }

    return (min,max);
}

fn sum_wide(data: &[i32]) -> i64 {
    let mut result : i64=0;
    for v in data.iter() {
        result += *v as i64;
    }
    return result;
}

fn mean(data: &[i32]) -> f64 {
    let n: usize  = data.len();
    let mut sum: i64 = 0;
    for v in data {
        sum += *v as i64;
    }
    let result: f64 = sum as f64 / n as f64;
    return result;
}

fn median(data: &[i32]) -> f64 {
    let n = data.len();
    let even = n % 2 == 0;
    let mut ordered_list: Vec<i32> = Vec::from(data);
    ordered_list.sort();
    let pos : usize;
    pos = n / 2;

    if !even {
        return ordered_list[pos] as f64;
    } else {
        return (ordered_list[pos-1] + ordered_list[pos]) as f64 / 2 as f64;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_simple(){
        let input: [i32;6]=[4,7,3,9,8,2];
        let (min,max) = min_max(&input);
        assert_eq!(min,2);
        assert_eq!(max,9);
    }

    #[test]
    fn min_max_same_num(){
        let input = [34];
        let (min,max) = min_max(&input);
        assert_eq!(min,34);
        assert_eq!(max,34);
    }

    #[test]
    fn sum_wide_simple(){
        let input = [23,45];
        let result = sum_wide(&input);
        assert_eq!(result,68);

    }

    #[test]
    fn mean_simple() {
        let input = [3,4,5];
        let res = mean(&input);
        assert_eq!(res,4 as f64);
    }

    #[test]
    fn median_even(){
        let result = median(&[2,7,4,3]);
        assert_eq!(result,3.5 as f64);
    }

        #[test]
    fn median_odd(){
        let result = median(&[2,7,4]);
        assert_eq!(result,4 as f64);
    }



}
