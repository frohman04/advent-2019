fn main() {
    let result = std::fs::read_to_string("src/bin/day08.txt")
        .map(|file| {
            let line = file
                .lines()
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()[0];
            let bits = line
                .chars()
                .map(|item| item.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>();

            let image = parse(bits, 25, 6);
            checksum(&image)
        })
        .expect("Unable to open file");

    println!("{}", result);
}

fn parse(bits: Vec<u8>, width: u8, height: u8) -> Vec<Vec<Vec<u8>>> {
    let mut parsed: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![0u8; width as usize]; height as usize];
        bits.len() / ((width * height) as usize)
    ];

    let mut i = 0 as usize;
    let mut layer_i = 0 as usize;
    while i < bits.len() {
        for y in 0..height {
            for x in 0..width {
                parsed[layer_i][y as usize][x as usize] = bits[i];
                i += 1;
            }
        }
        layer_i += 1;
    }

    parsed
        .iter()
        .map(|layer| layer.iter().map(|col| col.to_vec()).collect())
        .collect()
}

fn checksum(image: &Vec<Vec<Vec<u8>>>) -> u32 {
    let mut smallest_layer = 0 as usize;
    let mut smallest_size = std::u32::MAX;
    for i in 0..image.len() {
        let layer = image.get(i).unwrap();
        let layer_size = layer
            .iter()
            .map(|col| col.iter().filter(|val| **val == 0).count() as u32)
            .sum();
        if layer_size < smallest_size {
            smallest_size = layer_size;
            smallest_layer = i;
        }
    }

    let num_1: usize = image
        .get(smallest_layer)
        .unwrap()
        .iter()
        .map(|col| col.iter().filter(|val| **val == 1).count())
        .sum();
    let num_2: usize = image
        .get(smallest_layer)
        .unwrap()
        .iter()
        .map(|col| col.iter().filter(|val| **val == 2).count())
        .sum();

    (num_1 * num_2) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 3, 2),
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6],],
                vec![vec![7, 8, 9], vec![0, 1, 2],]
            ]
        )
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(&vec![
                vec![vec![0, 1], vec![2, 1],],
                vec![vec![0, 0], vec![0, 0],]
            ]),
            2
        )
    }
}
