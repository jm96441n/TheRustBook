fn main() {
    let mut days = ["";12];
    
    for i in 0..12 {
        let mut day = "";
        if i == 0 {
            day = "first";
            days [i] = "A Patridge in a Pear Tree";
        }

        if i == 1 {
            day = "second";
            days [i - 1] = "And A Patridge in a Pear Tree";
            days [i] = "Two turtle doves";
        }

        if i == 2 {
            day = "third";
            days [i] = "Three french hens";
        }

        if i == 3 {
            day = "fourth";
            days [i] = "Four calling birds";
        }

        if i == 4 {
            day = "fifth";
            days [i] = "Five gold rings";
        }

        if i == 5 {
            day = "sixth";
            days [i] = "Six geese a laying";
        }

        if i == 6 {
            day = "seventh";
            days [i] = "Seven swams a swimming";
        }

        if i == 7 {
            day = "eighth";
            days [i] = "Eight maids a milking";
        }

        if i == 8 {
            day = "ninth";
            days [i] = "Nine ladies dancing";
        }


        if i == 9 {
            day = "tenth";
            days [i] = "Ten lords a leaping";
        }

        if i == 10 {
            day = "eleventh";
            days [i] = "Eleven pipers piping";
        }

        if i == 11 {
            day = "twelfth";
            days [i] = "Twelve drummers drumming";
        }

        println!("On the {} day of Christmas my true love gave to me:", day);
        for ele in days.iter() {
            println!("{}", ele);
        }
    }
}
