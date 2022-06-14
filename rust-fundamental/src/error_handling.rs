
mod error_handling {
    use std::{fs::File, io::ErrorKind};

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    //  }

    // #[test]
//     fn test_error_handling() {
//         // Unrecoverable Errors dengan menggunakan panic!
//         // panic!("panic!!!")
//          let file = File::open("hello.txt");
//          let file = match file {
//             Ok(file) => file,
//             Err(error) =>{
//                 panic!("Error opening file : {:?}", error);
//             },
//          };
//     }
// }



#[test]
fn test_error_handling2() {
    // refactor kode sebelumnya dengan mengecek tipe error.
    File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("{:?}", error);
            })
        } else {
            panic!("{:?}",error)
        }
    });
}

#[test]
fn test_error_handling3() {
    File::open("hello.txt").expect("File not found!");
 }



}