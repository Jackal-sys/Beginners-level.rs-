fn main() {
    let mut arr = [[1; 5]; 5];
    println!("{:?}", arr);

    arr[3][1] = 0;
    arr[1][4] = 0;
    arr[4][1] = 0;
    arr[1][2] = 0;
    arr[1][0] = 0;

    for row in arr {
        println!("{row:?}");
    }
}
