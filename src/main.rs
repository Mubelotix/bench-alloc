use std::time::Instant;

const V: &str = "abcedfghijklmnopqrstuvwxyz";

fn smh() -> String {
    String::from(V)
}

fn smh_opti(s: &mut String) {
    s.clear();
    s.push_str(V);
}

fn bench(n: usize) -> (u128, u128) {
    // Plein d'allocations, mémoire non trouée
    let start = Instant::now();
    let s = vec![String::from(V); n];
    let r1 = start.elapsed().as_millis();
    println!("plein d'allocations avec mémoire non-trouée: {}ms", r1);

    // On ne gare que 1/5 des allocations
    let mut finals = Vec::with_capacity(s.len() / 5);
    for (i, s) in s.into_iter().enumerate() {
        if i % 5 == 0 {
            //let s = s.leak();
            finals.push(s);
        }
    }

    // Plein d'allocations, mémoire trouée
    let mut c  = 0;
    let start = Instant::now();
    let s = vec![String::from(V); n];
    let r2 = start.elapsed().as_millis();
    println!("plein d'allocations avec mémoire trouée: {}ms", r2);
    for s in s.into_iter() {
        c += s.len();
    }
    println!("c: {}", c); // Comme ça le compilateur croit qu'on utilise tout et ne vire par le dead code

    let mut c = 0;
    for f in finals {
        c += f.len();
    }
    println!("c: {}", c); // Comme ça le compilateur croit qu'on utilise tout et ne vire par le dead code

    (r1, r2)
}

fn main() {
    // non-opti single-thread
    let mut c = 0;
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        let s = smh();
        c += s.len();
    }
    println!("non-opti single-thread: {}ms ({})", start.elapsed().as_millis(), c);

    // opti single-thread
    let mut c = 0;
    let start = Instant::now();
    let mut s = String::with_capacity(3);
    for _ in 0..1_000_000_000 {
        smh_opti(&mut s);
        c += s.len();
    }
    println!("opti multi-thread: {}ms ({})", start.elapsed().as_millis(), c);

    let mut csv = String::new();
    let mut n = 640000.0;
    csv.push_str("n, single-thread, multi-thread\n");
    while n < 150_000_000.0 {
        let (r1, r2) = bench(n as usize);
        csv.push_str(&format!("{},{},{}\n", n as usize, r1, r2));
        n *= 1.15;
        println!("n: {}", n as usize)
    }
    std::fs::write("bench.csv", csv).unwrap();
}
