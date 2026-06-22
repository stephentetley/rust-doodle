
fn replaces1(s: &mut String, pair: (&str, &str)) -> bool {
    let (from, to) = pair;
    if s.contains(from) {
        *s = s.replace(from, to);
        true
    } else {
        false
    }

}

fn replaces(s: &mut String, pairs: &Vec<(&str, &str)>) -> bool {
    let mut b = false;
    for pair in pairs {
        b = b | replaces1(s, *pair);
    };
    b
}

fn exhaustive(s: &mut String, pairs: &Vec<(&str, &str)>) -> i32 {
    let mut b = true;
    let mut i = 0;
    while b {
        b = replaces(s, pairs);
        if b {i += 1};
    };
    i
}


fn main() {
    let mut s = String::from("Pmp Vlv-1");
    let v = vec![("Vlv", "Valve"), ("Pmp", "Pump")];
    let ans1 = exhaustive(&mut s, &v);
    println!("{ans1}: {s}");
}

