 pub(crate) mod collection_test {
    use std::collections::HashMap;

    /* 
    Vector mirip seperti array tetapi berbeda.
    Strings digunakan untuk menyimpan tipe data character.
    Hash Map memungkinkan kita untuk menyimpan nilai yang terasosiasi dengan key tertentu.
    */



    #[test]
    fn test_vector() {
    /* 
        Vec<T> yang dikenal sebagai vector, 
        sebuah tipe data yang dapat menyimpan lebih dari satu nilai kedalam single data struktur yang nilainya dapat ditambahkan atau dirubah. 
    */
        let mut numbers: Vec<i32> = Vec::new();
        numbers.push(1);
        numbers.push(2);

        for number in numbers {
            println!("{}", number);
        }
    }

    #[test]
    fn test_string() {
      /*   Kita dapat menyimpan text UTF-8 encoded ke dalam tipe data String. Rust hanya punya satu tipe string yang tersedia di inti bahasanya, 
        yakni str yang dikenal dengan string slice yang biasanya terlihat seperti ini &str istilahnya borrowed string. 
        */

        let hello = String::from("Hello");
        println!("{}", hello);

        // cara ke2
        let mut hello2 = String::from("Hello ");
        hello2.push_str("rust");
        println!("{}", hello2);


        // cara ke 3
        let hello3 = String::from("Hello3");
        let rust = String::from("rust");
        let join = format!("{} {}", hello3, rust);
        println!("{}", join);

    }


    #[test]
    fn test_hashmap() {
        /* 
        Untuk menyimpan koleksi data dengan key yang terasosiasi bisa menggunakan HashMap. 
        Syntaxnya seperti ini HashMap&lt;K, V> yaitu K mewakili key dan V adalah value. 
        */
        /* Untuk menggunakan HashMap kita butuh untuk mengambilnya dari standard library, 
        bisa dengan menggunakan use seperti contoh dibawah. 
        */

        let mut scores = HashMap::new(); 
        scores.insert(String::from("Team a"), 20);
        scores.insert(String::from("Team b"), 21);

        println!("{:?}", scores); // {"Team B": 11, "Team A": 10}


        // Cara mengakses value pada HashMap bisa menggunakan method get dan key dimasukan sebagai parameter, 
        // lengkapnya seperti dibawah ini.
        println!("ini hasil get = {:?}", scores.get(&String::from("Team a"))); // Some(20)


        // Kemudian tahapan selanjutnya dari HashMap adalah cara untuk memperbaharui nilai jika key tersebut tidak memiliki nilai.
        scores.entry(String::from("Team a")).or_insert(21);
        scores.entry(String::from("Team c")).or_insert(31);

        println!("hasil perbaruhi data = {:?}", scores);


    }

    #[test]
    fn test_with_space() {
        let text = "hari ini saya belajar rust dari awal loh";

        let mut result = HashMap::new();

        for t in text.split_whitespace() {
            let count = result.entry(t).or_insert(0);
            *count +=1;
        }

        println!("{:?}", result);
    }



}