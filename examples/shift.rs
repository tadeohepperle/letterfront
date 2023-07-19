use letterfront::models::{array2d::Int2, corpus::Corpus, letterfield::Letterfield};

pub fn main() {
    let corpus = Corpus::from_txt_file("assets/english3000.txt", 4).unwrap();
    let mut letterfield = Letterfield::random(4, 3, &corpus);
    println!("{}", letterfield.to_detail_string());
    letterfield.move_letter(Int2 { x: 1, y: 2 }, Int2 { x: 1, y: 0 });
    println!("///");
    println!("{}", letterfield.to_detail_string())
}
