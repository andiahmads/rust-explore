mod ownership {
/* 
    ownership adalah sekumpulan rule yg mengatur bagaimana sebuh program memanage sebuah memory.
    momory tersebut adalah stack & heap.
    ownership menjamin memroy sefty tanpa memerlukan garbage collector.

    rule ownership
    1. setiap nilai didalam rust memiliki sebuah variable yg dikenal dg owner & hanya boleh ada 1 owner pada 1 waktu.
    2. ketika owner keluar dari scope maka nilai akan diblock
 */
    #[test]
    fn test_ownership() {
        /* { // s is not valid here, it's not yet declared
            // println!("{}",s);
            let s = "hello"; // s is valid from this point forward
            println!("{}",s);
        } // this scope is over, s no longer valid 
        */

        // contoh lain
        let mut s = String::from("hello"); //string literal / string builder
        s.push_str(",world");
        println!("{}",s);

        let x = 5;
        let y = x; //copy <= copy trait / disimpan dalam stack (tipe data scalar type)

        println!("x = {}, y = {},",x,y);

        // contoh lain
        let s1 = String::from("testing clone");
        // let s2 = s1; // moved (dihandle oleh ownership) //ERROR
        let s2 = s1.clone(); //Success

        println!("s1 = {}, s2 = {},",s1,s2);


        // todo: ownership & function

        let ss = String::from("Hellowwwws");

        take_ownerhip(ss);

        
        let x = 5;

        make_copy(x);


        let x1 = gives_ownership();
        println!("{}",x1);   

        let x2 = String::from("GGkan>");
        let x3 = take_give_back(x2);
        println!("{}",x3);   

        let x4 = String::from("HELLO TUPLE");
        let (s5,size) = calculate_length(x4);
        println!("the leng of {} is {}",s5,size);


    }


    // return tuple
    fn calculate_length(s:String) ->(String,usize) {
        let length = s.len();
        return (s,length)
    }

    fn take_ownerhip(somestring: String) {
        println!("somestring = {}",somestring);
    }

    fn make_copy(some_int: i64) {
        println!("someINT = {}",some_int);
    }

    fn gives_ownership() -> String {
        let sr = String::from("coooksssssssss");
        return sr;
    }
    fn take_give_back(a_string: String) -> String {
        return a_string;
    }
    

}