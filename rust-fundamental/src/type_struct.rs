mod belajar_struct {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    // tuple struct

    struct MyStruct; // unit like struct

    #[test]
    fn test_struct() {

        let mut user1 = User{
            active: false,
            username: String::from("user1"),
            email: String::from("user1@example.com"),
            sign_in_count: 0,
        };
        println!(" hallo user1 {:?}", user1);


        let mut user2 = User{
            email: String::from("user2@example.com"),
            ..user1
        };
        println!(" hallo user2 {:?}", user2);

        // call fn return_struct
        let res = return_struct(String::from("saipul@gmail.com"), String::from("saipul@"));
        println!("{:?}", res);


        // implementasi tuple struct
        let persegi_panjang = (2,3);
        let hasil = hitung_luas(persegi_panjang);
        println!("hasil dari p * l = {}", hasil);


    // implementasi struct tuple dan struc biasa

    let panjang_persegiv2 = Rectangel{
        width: 100,
        height: 100,
    };
    println!("{:#?}", panjang_persegiv2);

    let hasilv2 = hitung_luasv2(&panjang_persegiv2);
    println!("hasil dari p * l v2 = {}", hasilv2);



    
    }


    fn return_struct(email: String,username: String) -> User {
        return User{
            email: email,
            username: username,
            active: false,
            sign_in_count: 210,
        }
    }

    fn hitung_luas(dimension:(i32,i32)) -> i32 {
        return dimension.0 * dimension.1;
    }

    // combine struct tuple dan struc biasa
    #[derive(Debug)]
    struct Rectangel {
        width: u32,
        height: u32,
    }

    fn hitung_luasv2(dimension:&Rectangel) -> u32 {
        return dimension.width * dimension.height;
    }
    
}