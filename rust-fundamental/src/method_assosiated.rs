mod method_assosiated {

    #[derive(Debug)]
    struct Segitiga {
        width:u32,
        height:u32,
    }

    // implementasi struct segitiga
    impl Segitiga {
        // didalam scope ini terdapat tipe self
        // tipe self merujuk kedalam struct Segitiga
        // maka setiap fungsi asosiated wajib menuliskan self
        // borrowed pada paramter self bertujuan untuk membaca data saja, dan tidak mengambil alih ownership
        fn area(&self) -> u32 { // asosiated function
            return self.height * self.width;
        }

        fn width(&self) -> bool {
            return self.width > 0;
        }
    }


    #[test]
    fn test_method_assosiated() {
        let segitiga1 = Segitiga{
            width: 12,
            height: 12,
        };
        let segitiga2 = Segitiga{
            width: 13,
            height: 13,
        };

        let segitiga3 = Segitiga{
            width: 14,
            height: 14,
        };

        println!("cant segitiga1 hold segitiga2");

/*      let res = segitiga1.area();
        println!("hasil =  {}", res);
        println!("width besar dari 0 =  {}", segitiga1.width());
        println!("hasil =  {:?}", segitiga1);
 */

        



    }

}