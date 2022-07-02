mod search_target_with_index{

 #[test]   
fn test_search_target_with_index() {
    let arr = vec![1,3,4,5,6,7,8];
    let target = 5;

    let res = search_target(arr,target);
    println!("{}",res);

}    


fn search_target(arr: Vec<i32>, target:i32) -> usize {
    let mut index = 0;
    for (i, &items) in arr.iter().enumerate() {
        if items <= target {
            index = i;
        }
    }
    return index;
}


}


