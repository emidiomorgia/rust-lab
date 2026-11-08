use crate::Forma::Cerchio;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Punto { x: f64, y: f64 }

impl Punto {
    fn new(x : f64, y: f64) -> Self {
        return Self {x,y};
    }

    fn origine() -> Self {
        return Self {x:0.0, y:0.0};
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Forma {
    Cerchio { centro: Punto, raggio: f64 },
    Rettangolo { angolo: Punto, larghezza: f64, altezza: f64 },  // angolo = vertice in basso a sinistra
    Triangolo(Punto, Punto, Punto),
}

impl Forma {
    fn cerchio(centro: Punto, raggio: f64) -> Option<Forma> {
        if raggio.is_nan() || raggio.is_infinite() {
            return None;
        }
        return Some(Cerchio { centro, raggio });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works(){
        let p = Punto::new(1 as f64,2 as f64);
        assert_eq!(p.x,1 as f64);
        assert_eq!(p.y,2 as f64);
    }

    #[test]
    fn origin_works(){
        let p = Punto::origine();
        assert_eq!(p.x,0.0);
        assert_eq!(p.y,0.0);
    }

    #[test]
    fn cerchio_constructor_some() {
        let centro = Punto { x: 10.0, y: 10.0 };
        let c = Forma::cerchio(centro, 4.5);
        assert_eq!(c, Some(Forma::Cerchio { centro, raggio: 4.5 }));
    }
}
