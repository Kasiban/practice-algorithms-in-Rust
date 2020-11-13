//Print the lyrics to the Christmas carol "The Twelve Days of Chrismas," taking advantage of the
//repetition in the song.


fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a patridge in a pear tree",
                 "two turtle doves and",
                 "three French hens",
                 "four calling birds",
                 "five golden rings",
                 "six geese a-laying",
                 "seven swans a-swimming",
                 "eight maids a-milking",
                 "nine ladies dancing",
                 "ten lords a-leaping", 
                 "eleven pipers piping",
                 "twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love gave to me:");

        for j in 0..i + 1 {
            println!("{}", gifts[j]);
        }
        
        println!("-----");
    }

}
