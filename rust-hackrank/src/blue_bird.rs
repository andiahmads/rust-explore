mod blue_bird{


    #[test]
    fn test_blue_bird() {
        let mut arr = vec![8,9,3,10,2,11];
        let res = blue_bird(&mut arr);

        println!("{}", res);

    }


    fn blue_bird(arr: &mut Vec<i32>) -> i32{

        let mut list_build = 0;
        let mut total_build = 0;

        for builds in arr {
            if builds > &mut list_build {
                list_build = *builds;
                total_build += 1;
            }
        } 

        return total_build;

    }

}