fn main() {
    println!("Hello, world!");
    println!("{}", scegli("abc", "abcd"));
}

fn scegli<'a>(preferito: &'a str, altro: &'a str) -> &'a str {
   if preferito.len() > altro.len() {
       return preferito;
   } else {
       return altro;
   }
}

struct Riga<'a> {
    numero: usize,      // 1-based, riferito al testo originale
    testo: &'a str,
}

fn parse_righe<'a>(testo: &'a str) -> Vec<Riga<'a>> {
    testo
        .lines()
        .enumerate()
        .filter(|(_, riga)| !riga.trim().is_empty())
        .map(|(i, riga)| Riga { numero: i + 1, testo: riga })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scegli_works(){
        let res = scegli("abc","abcd");
        assert_eq!(res,"abcd");
    }

    #[test]
    fn parse_righe_salta_vuote_e_mantiene_numerazione() {
        let righe = parse_righe("a\n\n  \nb\r\nc");
        let v: Vec<(usize, &str)> = righe.iter().map(|r| (r.numero, r.testo)).collect();
        assert_eq!(v, vec![(1, "a"), (4, "b"), (5, "c")]);
    }
}
