mod konsep_umum{

    #[test]
    fn test_konsep_umum() {

        // secara default variable dirust itu mutable

        let a = 10; //mutable
        let mut b = 20; //imutable


        const HIGH_PRICE: i32 = 10_000_000; //rust mendukung human readable


        // konsep shadowing
        let x = 10;

        let x = "ten";

        // data Type
        // didalam rust itu ada 2 jenis type data, scalar type & compound type

        /*
         sclar : integer, number, floating, bool, character
         
         i = signed integer = nilai bisa menyimpan nilai +/-
         u = Unsigned = hanya bisa menyimpan nilai + saja
         floating = secara default 64 bit, untuk 32 tinggal di definisikan
         char = harus menggunakan single quote => ''
        */
        let y: i16 = 32;
        let f = 2.0; 
        let f: f32 = 8.0;

        let valid = true;
        let invalid = false;
        let c = 't';

        /* 
        compound type : tuple &  array
        tuple = length bersifat fix length ketika awal di deklarasi 
        array = length bersifat fix length ketika awal di deklarasi 
        */
        let tup: (i32,f64,u8) = (100,2.0,1);
        let (x,y,z) = tup; //akses tuple
        let akses_index = tup.0; 

        let tup2:() = (); //unit type

        // array
        let arr = [1,3,2,4];
        let b: [i32; 5] = [1,2,3,4,5];
        let b = [3,5]; // => [3,3,3,3,3]
        let get_index = b[0];

        // varaiable dengan if else
        let cond = true;
        let num2 = if cond {5} else {6};
        
        
        // looping
        loop {
            println!("infinity loop")
        }

        let mut counter = 0;
        let res = loop {
            counter += 1;
            if  counter == 10 {
                break counter *2
        }
    };
    println!("{}", res);

    //while 
    let nums = 3;
    while nums != 0 {
        println!("{}", nums);
    };

    //for loop
    let arr4 = [1,3,4,5,];
    for element in arr4 {
        println!("{}", element);
    }

    for num5 in 1..5{ // 1 sampai 5
        println!("{}", num5);
    }




 
        // pemanggilan function di rust
        my_function();
        my_function_with_param(3,'u');
        let res = return_function();
        println!("{}",res);
    }
    

    fn my_function() {
        println!("hello cokkks");
    }

    // dengan parama
    fn my_function_with_param(value:i32, label:char) {
        println!("{}{}",value,label);
    }

    // dengan return
    fn return_function() -> i32 {
        return 5;
    }

}