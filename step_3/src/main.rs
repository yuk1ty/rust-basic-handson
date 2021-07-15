fn main() {
    let this_is_str = "こう束縛すると &str 型になります";
    let this_is_string = "こう束縛すると String 型になります。".to_string();
    let mut this_is_mut_string = "こう束縛するとミュータブルな String 型になります。".to_string();
    this_is_mut_string.push_str("さらに、String 型はあとから文字列を追加できます。");

    println!("{}", this_is_mut_string);
}
