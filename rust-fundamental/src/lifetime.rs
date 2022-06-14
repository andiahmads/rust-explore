mod lifetime_test {
    // #[test]
    /* fn test_dangling() {
        /* Lifetime adalah bagian dari fitur ownership yang tersedia di Rust.
        Sebelumnya jika kalian melihat bahasan tentang borrowing & references,
        dimana references akan tetap valid jika berada didalam ruang lingkupnya,
        namun jika telah berada diluar scope maka semuanya akan terhapus dari memori.
         Nah bisa dibilang lifetime adalah scope, fitur ini memudahkan kita untuk mengatur scope yang ada di Rust.

         */

        let s;

        {
            let x = 5;
            s = &x
        }

        println!("{}", s)
    } */
    #[test]
    fn test_lifetime() {
        let first_letter = "hallo indonesia";
        let second_letter = "hallo";

        let longest = longest(first_letter, second_letter);
        println!("{}", longest); // "Hallo indonesia
    }

    fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }
}
