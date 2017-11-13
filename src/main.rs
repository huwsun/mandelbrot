// type Atom = (String, usize);
// type Molecule = Vec<Atom>;
fn foo<'a>(x: &'a str,y:&'a str) ->&'a str {
    if 9%2==0 {x} else {y}
}

fn check(s:&str) ->&str
// where 'a:'b
 {
    //println!("{}",s);
    return s;
}

fn main() {
    let x="hi";
 let e;
 {
    let y="you";
    e=y;
    //e=check(y);

    println!("{:p}",y );
 }
    println!("Hello, world! {:p}",e);
}
