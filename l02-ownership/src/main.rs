#[derive(Clone, Copy)]
struct Punto { x: i32, y: i32 }

#[derive(Clone)]
struct Buffer { id: u32, dati: Vec<u8> }

fn main() {
    println!("Hello, world!");
    let mut b = Buffer {id:1, dati: vec![1,2,3]};
    println!("len_bytes: {}", len_bytes(&b));
    println!("{}", b.id);
    append(&mut b, &[4,5,6]);
    print!("len buffer after append: {}", b.dati.len());
    let b2 : Buffer = duplicate(&b, 2);
    println!("duplicate b2 len {} with id {}",b2.dati.len(), b2.id);
    let b3 : Buffer = Buffer { id: 1, dati : vec![1,2,3]};
    let m1 : Buffer = merge(b, b3, 3);
    
}

fn len_bytes(b: &Buffer) -> usize {
    return  b.dati.len();
}

fn append(b: &mut Buffer, extra: &[u8]) {
    b.dati.extend_from_slice(extra);
}

fn duplicate(b: &Buffer, nuovo_id: u32) -> Buffer {
    return Buffer { id: nuovo_id, dati: b.dati.clone()};
}

fn into_vec(b: Buffer) -> Vec<u8>{
    return b.dati;
}

fn merge(a: Buffer, b: Buffer, nuovo_id: u32) -> Buffer {
    let mut v = a.dati;
    v.append(&mut b.dati.clone());
    return Buffer { id: nuovo_id, dati : v};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_bytes_works(){
        let b: Buffer = Buffer { id: 1, dati: vec![1,2,3]};
        assert_eq!(len_bytes(&b),3);
    } 

    #[test]
    fn append_works() {
        let mut b: Buffer = Buffer{ id: 1, dati: vec![1,2,3]};
        let extra: [u8; 3] = [4,5,6];
        append(&mut b, &extra);
        assert_eq!(b.dati.len(),6);
        assert_eq!(b.dati,[1,2,3,4,5,6]);
        
    }

    #[test]
    fn duplicate_works(){
        let b1 : Buffer = Buffer{id: 1, dati: vec![1,2,3]};
        let b2 : Buffer = duplicate(&b1, 2);
        assert_eq!(b2.dati.len(),3);
        assert_eq!(b2.id, 2);
    }

    #[test]
    fn into_vec_works(){
        let b = Buffer{ id: 1, dati: vec![2,4,6]};
        let b2 = into_vec(b);
        assert_eq!(b2.len(),3);
    }

    #[test]
    fn merge_works(){
        let b1 = Buffer{ id: 1, dati : vec![1,2,3]};
        let b2 = Buffer{ id : 2, dati : vec![4,5]};
        let b3 = merge(b1, b2, 3);
        assert_eq!(b3.dati.len(),5);
    }
}
