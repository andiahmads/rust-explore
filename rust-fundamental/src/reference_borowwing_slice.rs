mod references_borowwing_slice{


    #[test]
    fn test_references_borowwing_slice() {

        // tanpa references
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The Lengh of {} is {}", s2,len); 
       

        // dengan references
        let s1 = String::from("hello");
        let len = calculate_length_with_references(&s1);  // <- ini adalah borowwing
        println!("The Length of {} using references is {}", s1,len); 



        let mut a: String = String::from("cook");
        change(&mut a); // a mutable reference
        println!(" value of change {}", a);

        let mut b = String::from("hello references");
        b.push_str("rust");
        let c = &b;
        println!(" value of change {}", c);


        let mut v = String::from("hello references v");
        let v1 = &v;
        let v2 = &v;
        println!(" {}, {}", v1,v2); // batas akhir imutable reference digunakan

        let v3 = &mut v;
        println!("{}", v3); // batas akhir imutable reference digunakan


        // contoh dangle pointer
        // let reference_dangle_pointer = dangle();

        // program get spaci 
        let mut sample = String::from("get spasi");
        let get_space = first_word(&sample);
        sample.clear(); // menjadi string kosong = ""
        println!("string kosong {}", sample);
        // get_space masih akan mengembalikan nilai 3
        println!("spasi ditemukan di index ke {}", get_space);


        // string slice
        let slice = String::from("hello slice");
        let hello = &slice[..=5];
        let slices = &slice[6..];
        println!("string slice = {}", hello);
        println!("string slice = {}", slices);

    }
    
    // materi slice
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' '{ // byte literal
                return i;
            }
        }
    return s.len()
    }
    
    // fn dangle() -> &String {
    //     let s = String::from("hello dangle");
    //     return &s
    // } // s is droped
    
    // references itu imutable
    fn change(some_string: &mut String) {
        some_string.push_str("cccookk");
    }


    fn calculate_length_with_references(s:&String) ->  usize { // s is references to  a string
        return s.len();
    }

    // tanpa menggunakan references
    fn calculate_length(s:String) -> (String, usize) {
        let length = s.len();
        return (s,length)
    } 
   
    


}