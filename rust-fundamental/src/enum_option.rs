mod enum_options {



    // contoh enum option

    // enum Option<T> { // Generic Type Parameter
    //     Node,
    //     Some(T), // akan menyimpan 1 data dari segala macam jenis type
    // }


    #[test]
    fn test_enum_options() {
        let some_number = Option::Some(5);
        let some_str = Some("hello");
        let number: Option<i32> = None; // untuk variant None wajib menyertakan Type Anotation

        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y.unwrap();
        println!("{:?}", sum);

    }

}