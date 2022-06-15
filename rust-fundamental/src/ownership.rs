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




    }

}