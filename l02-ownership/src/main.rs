#[derive(Clone, Copy)]
struct Punto { x: i32, y: i32 }

#[derive(Clone)]
struct Buffer { id: u32, dati: Vec<u8> }

fn main() {
    println!("Hello, world!");
    let b = Buffer {id:1, dati: vec![1,2,3]};
    println!("len_bytes: {}", len_bytes(&b));
    println!("{}", b.id);
}

fn len_bytes(b: &Buffer) -> usize {
    return  b.dati.len();
}

fn append(b: &mut Buffer, extra: &[u8]) {
    b.dati.extend_from_slice(extra);
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
}
