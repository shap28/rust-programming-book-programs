fn main() {
    let a = [
        "twelfth", "eleventh", "tenth", "ninth", "eighth", "seventh", "sixth", "fifth", "fourth",
        "third", "second",
    ];
    let addons = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
    ];

    println!(
        "On the first day of Christmas,
my true love gave to me
A partridge in a pear tree.\n"
    );

    for i in (0..11).rev() {
        let day = a[i];
        println!("On the {day} day of Christmas,\nmy true love gave to me");
        for j in i..11 {
            let element = addons[j];
            println!("{element}");
        }
        println!("A partridge in a pear tree.");
        println!();
    }
}
