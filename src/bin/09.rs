advent_of_code::solution!(9);

#[derive(Clone, Debug)]
struct FileInfo {
    id: usize,
    size: usize,
    space_after: Option<FreeSpace>,
}
impl FileInfo {
    fn pack_file(&mut self, other: &mut FileInfo, limit: usize) -> usize {
        match self.space_after {
            Some(FreeSpace::Empty { size }) => {
                if size == 0 {
                    return 0;
                }

                let size_packed = other.size.min(size).min(limit);
                other.size -= size_packed;

                self.space_after = Some(FreeSpace::Packed {
                    free_space: size - size_packed,
                    files: vec![FileInfo {
                        size: size_packed,
                        space_after: None,
                        ..*other
                    }],
                });

                return size_packed;
            }
            Some(FreeSpace::Packed {
                ref mut free_space,
                ref mut files,
            }) => {
                if *free_space == 0 {
                    return 0;
                }

                let size_packed = other.size.min(*free_space).min(limit);
                other.size -= size_packed;
                *free_space -= size_packed;

                let last_file = files.iter_mut().rev().nth(0).unwrap();
                if last_file.id == other.id {
                    last_file.size += size_packed;
                } else {
                    files.push(FileInfo {
                        size: size_packed,
                        space_after: None,
                        ..*other
                    });
                }

                return size_packed;
            }
            None => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug)]
enum FreeSpace {
    Empty {
        size: usize,
    },
    Packed {
        free_space: usize,
        files: Vec<FileInfo>,
    },
}

fn expand_fs(input: &str) -> Vec<FileInfo> {
    let chars = input.chars().collect::<Vec<char>>();

    chars
        .chunks(2)
        .enumerate()
        .map(|(id, chunk)| {
            let mut chunk_parts = chunk
                .into_iter()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_string().parse::<usize>().unwrap());

            let size = chunk_parts.next().unwrap();
            let space_after = chunk_parts.next().map(|s| FreeSpace::Empty { size: s });

            FileInfo {
                id,
                size,
                space_after,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut unpacked_files = expand_fs(input);
    let bytes_total = unpacked_files.iter().fold(0, |acc, f| acc + f.size);

    let mut forward_files = unpacked_files.clone();
    let mut forward_files_iter = forward_files.iter_mut().filter(|f| f.space_after.is_some());

    let mut reverse_files_iter = unpacked_files.iter_mut().rev();

    let mut file_to_pack = forward_files_iter.next().unwrap();
    let mut file_at_back = reverse_files_iter.next().unwrap();

    let mut packed_files = Vec::new();

    let mut bytes_packed = file_to_pack.size;

    loop {
        let available_bytes = bytes_total - bytes_packed;

        let packed = file_to_pack.pack_file(&mut file_at_back, available_bytes);

        bytes_packed += packed;

        if packed == 0 {
            packed_files.push(file_to_pack.clone());

            if bytes_packed >= bytes_total {
                break;
            }

            file_to_pack = match forward_files_iter.next() {
                Some(file) => file,
                None => break,
            };

            file_to_pack.size = file_to_pack.size.min(bytes_total - bytes_packed);

            bytes_packed += file_to_pack.size;
        } else if file_at_back.size == 0 {
            file_at_back = match reverse_files_iter.next() {
                Some(file) => file,
                None => break,
            };
        }
    }

    let blocks = packed_files
        .into_iter()
        .map(|f| {
            let mut blocks = [f.id].repeat(f.size);

            if let Some(FreeSpace::Packed {
                free_space: _,
                files,
            }) = f.space_after
            {
                blocks.extend(files.into_iter().flat_map(|f| [f.id].repeat(f.size)));
            }

            blocks
        })
        .flatten()
        .collect::<Vec<usize>>();

    let checksum = blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + i * id);

    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
