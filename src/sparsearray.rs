fn main() {
    let mut map = [[0; 11]; 11];
    map[1][2] = 1;
    map[2][3] = 2;

    println!("map is {:?}", map);
    to_sparse_array(map);
}

fn to_sparse_array(map: [[i32; 11]; 11]) {
    let mut sum: i32 = 0;
    for i in 0..11 {
        for j in 0..11 {
            if map[i][j] != 0 {
                sum += 1;
            }
        }
    }
    let mut resut: Vec<Vec<i32>> = vec![vec![0; 3]; (sum + 1) as usize];

    resut[0][0] = 11;
    resut[0][1] = 11;
    resut[0][2] = sum;

    let mut index = 1;
    for i in 0..11 {
        for j in 0..11 {
            if map[i][j] != 0 {
                resut[index][0] = i as i32;
                resut[index][1] = j as i32;
                resut[index][2] = map[i][j];
                index += 1;
            }
        }
    }

    println!("resutl , {:?}", resut);
}
