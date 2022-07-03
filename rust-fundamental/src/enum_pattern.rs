mod enum_pattern {


    /*
        enum adalah tipe varian yg merupakan serangkaian kemungkinan
        enum bisa di combine dengan struct
    */
    #[derive(Debug)]
    enum IpAddKind {
        V4,
        V6,
    }

    struct IpAddr {
        Kind: IpAddKind,
        address: String,
    }

    #[test]
    fn test_enum_pattern() {
        // implementasi enum
        let four = IpAddKind::V4;
        let six = IpAddKind::V6;

        let ipv4 = IpAddr {
            Kind: four,
            address: String::from("127.0.0.1"),
        };
        let ipv6 = IpAddr {
            Kind: six,
            address: String::from("127.2.2.1"),
        };

        println!("ipv4: {:?} {:?}", ipv4.Kind, ipv4.address);
        println!("ipv6: {:?} {:?}", ipv6.Kind, ipv6.address);
    }

    #[derive(Debug)]
    enum Karnel {
        // enum assosiated value
        Linux(String),
        Unix(String),
    }

    #[derive(Debug)]
    enum VersionOS {
        Version(u32, u32),
        CodeName(String),
    }

    #[test]
    fn test_enum_pattern2() {
        let karnel1 = Karnel::Linux(String::from("Debian"));
        let karnel2 = Karnel::Unix(String::from("OpenBSD"));
        let version1 = VersionOS::Version(22, 04);
        let codeName1 = VersionOS::CodeName(String::from("Kingkong"));

        let version2 = VersionOS::Version(22, 09);
        let codeName2 = VersionOS::CodeName(String::from("Una"));

        println!("Detail: {:?} {:?} {:?}", karnel1, version1, codeName1);
        println!("Detail: {:?} {:?} {:?} ", karnel2, version2, codeName2);
    }

    /*
    keuntungan enum dari pada struct adalah syntax menjadi lebih pendek
    dan juga untuk case impl sebua struct, dengan menggunakan enum kita cukup
    implementasi 1 method atau assosiated function
    lihat pada code dibawah ini:
     */

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
        }
    }
/* 
    // konvert ke type struct
    struct QuitMessage; // sama dengan enum Quit

    struct Move {
        // sama dengan enum Move
        x: i32,
        y: i32,
    }

    struct WriteMessage(String); // struct tuple sama dengan enum Write

    struct ChangeColor(i32, i32, i32); // sama dengan enum ChangeColor
 */
    #[test]
    fn enum_pattern3() {
        let msg = Message::Write(String::from("hello"));
        println!("{:?}", msg.call())
    }
}
