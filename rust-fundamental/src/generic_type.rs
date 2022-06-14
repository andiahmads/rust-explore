mod generic_type{

    /* Generic adalah cara untuk membuat tipe data menjadi lebih flexible, 
    sehingga mudah untuk dipakai, berulang-ulang dan terhindar dari masalah duplicate.
    */
    #[test]
    fn test_get_big_number() {
        let numbers = vec![1, 2, 3, 4, 5];

        let mut largest = numbers[0];

        for number in numbers {
            if number > largest {
                largest = number;
            }
        }

        println!("{}", largest);

    }


    // refactoring
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &number in list {
            if number > largest {
                largest = number;
            }
        }
        return largest;
    }
    #[test]
    fn test_largest() {
        let numbers = vec![8, 5, 9, 10, 2, 90];
        
        let result = largest(&numbers);

        let sun_result = sun(&numbers);

        println!("find largest :{}", result);
        println!("sun result :{}", sun_result);
    }


    fn sun(building: &[i32]) -> i32 {
        let mut build = building[0];
        let mut sun_build = 0;

        for &number in building {
            if number > build {
                build = number;
                sun_build +=1;
            }
        }
        return sun_build;
    }

    


}