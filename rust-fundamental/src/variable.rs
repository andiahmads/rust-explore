pub fn variable() {
    let name = "Adiatma";
    println!("{}", name); // "Adiatma"
                          // mutable variable -> variable yang bisa diubah
    let mut change_name = "A";
    println!("{}", change_name); // "A"
    change_name = "Adiatma";
    println!("{}", change_name); // "Adiatma"
}

pub fn shadowing() {
    /* Shadow berbeda dengan konsep mutable,
    kegunaan fitur ini adalah agar kita bisa memiliki banyak nama variabel yang sama,
    dan variable selanjutnya disebut sebagai shadow atau bayangan dari variabel sebelumnya.
    Fitur ini berguna jika ingin menimpa nilai yang ada sebelumnya dengan proses selanjutnya.
     */

    let x = 1;
    let x = x * 1;

    println!("{}", x)
}
