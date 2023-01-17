use itertools::Itertools;

fn wrap(i: isize, n: usize) -> usize {
    i.rem_euclid(n as isize) as usize
}

fn encrypt(file: &Vec<isize>, n: usize) -> Vec<isize> {
    let mut encrypted = (0..file.len()).collect_vec();
    for _ in 0..n {
        for (i, &x) in file.iter().enumerate() {
            let p = encrypted.iter().position(|&y| y == i).unwrap();
            encrypted.remove(p);
            encrypted.insert(wrap(p as isize + x, encrypted.len()), i);
        }
    }
    encrypted.iter().map(|&i| file[i]).collect_vec()
}

pub fn solve(input: &str) -> (isize, isize) {
    let file = input.lines().map(|l| l.parse::<isize>().unwrap()).collect_vec();

    let file_enc = encrypt(&file, 1);
    let p = file_enc.iter().position(|&x| x == 0).unwrap();
    let p1 = [1000, 2000, 3000].iter().map(|&i| file_enc[(p + i) % file_enc.len()]).sum();

    let file_enc = encrypt(&file.iter().map(|&x| x * 811589153).collect_vec(), 10);
    let p = file_enc.iter().position(|&x| x == 0).unwrap();
    let p2 = [1000, 2000, 3000].iter().map(|&i| file_enc[(p + i) % file_enc.len()]).sum();

    (p1, p2)
}