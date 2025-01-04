use std::{collections::HashSet, usize};

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

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: usize, size: usize },
    Empty { space: usize },
}

pub fn part_two(input: &str) -> Option<u64> {
    use Block::*;

    let chars = input.chars().collect::<Vec<char>>();

    let mut disk = Vec::new();

    chars.chunks(2).enumerate().for_each(|(id, chunk)| {
        let mut chunk_parts = chunk
            .into_iter()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_string().parse::<usize>().unwrap());

        let size = chunk_parts.next().unwrap();

        disk.push(File { id, size });
        chunk_parts.next().map(|space| disk.push(Empty { space }));
    });

    let mut current_id = *disk
        .iter()
        .rev()
        .filter_map(|f| match f {
            File { id, size: _ } => Some(id),
            _ => None,
        })
        .nth(0)
        .unwrap();

    loop {
        if current_id == 0 {
            break;
        }

        let (file_idx, file_id, file_size) = disk
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                File { id, size } => {
                    if *id == current_id {
                        Some((i, *id, *size))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .nth(0)
            .unwrap();

        let found_space = disk
            .iter()
            .enumerate()
            .filter(|(i, _)| *i < file_idx)
            .filter_map(|(i, block)| match block {
                Empty { space } => {
                    if *space >= file_size {
                        Some((i, space))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .nth(0);

        if let Some((space_idx, space)) = found_space {
            let remaining_space = space - file_size;

            disk[file_idx] = Empty { space: file_size };
            disk[space_idx] = File {
                id: file_id,
                size: file_size,
            };

            if remaining_space > 0 {
                let mut replaced = false;
                if space_idx > 0 {
                    match disk[space_idx - 1] {
                        Empty { ref mut space } => {
                            *space += remaining_space;
                            replaced = true;
                        }
                        _ => (),
                    }
                }
                if !replaced && space_idx < disk.len() - 1 {
                    match disk[space_idx + 1] {
                        Empty { ref mut space } => {
                            *space += remaining_space;
                            replaced = true;
                        }
                        _ => (),
                    }
                }
                if !replaced {
                    disk.insert(
                        space_idx + 1,
                        Empty {
                            space: remaining_space,
                        },
                    );
                }
            }
        }

        if false {
            let debug = disk
                .iter()
                .flat_map(|block| match block {
                    File { id, size } => [Some(*id)].repeat(*size),
                    Empty { space } => [None].repeat(*space),
                })
                .map(|block| match block {
                    Some(id) => id.to_string(),
                    None => ".".to_string(),
                })
                .collect::<Vec<_>>()
                .join("");
            println!("after id {}: {}", current_id, debug);
        }

        current_id -= 1;
    }

    // println!("{:#?}", disk);

    let blocks = disk
        .iter()
        .flat_map(|block| match block {
            File { id, size } => [Some(*id)].repeat(*size),
            Empty { space } => [None].repeat(*space),
        })
        .collect::<Vec<Option<usize>>>();

    // println!("{:?}", blocks);

    let checksum = blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + i * id.unwrap_or(0));

    Some(checksum as u64)
}

pub fn crazy(input: &str) -> Option<u64> {
    let mut files = expand_fs(input);

    let mut processed = HashSet::new();
    let mut i = files.len() - 1;
    loop {
        let len = files.len();

        let mut apply = None;

        {
            let current_file_size = files[i].size;
            if processed.contains(&files[i].id) {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                    continue;
                }
            }

            processed.insert(files[i].id);

            println!("Current file {} : {}", files[i].id, current_file_size);

            for j in 0..i {
                let space_after = &files[j].space_after;

                match space_after {
                    Some(FreeSpace::Empty { size }) => {
                        println!("  free space {}: {}", files[j].id, size);
                        if *size >= current_file_size {
                            apply = Some(j);
                            break;
                        }
                    }
                    Some(FreeSpace::Packed {
                        free_space,
                        files: _,
                    }) => {
                        println!("  free space {}: {}", files[j].id, free_space);
                        if *free_space >= current_file_size {
                            apply = Some(j);
                            break;
                        }
                    }
                    None => (),
                }
            }

            if let Some(j) = apply {
                let last_file = files.remove(i);

                {
                    let other_file = &mut files[j];

                    println!(
                        "Applying insert of {} ({}) after {}",
                        last_file.id, last_file.size, other_file.id
                    );

                    match &mut other_file.space_after {
                        Some(FreeSpace::Empty { ref mut size }) => *size -= last_file.size,
                        Some(FreeSpace::Packed {
                            ref mut free_space,
                            files: _,
                        }) => *free_space -= last_file.size,
                        None => (),
                    }
                }
                if j > 0 {
                    let prev_file = &mut files[j - 1];
                    match &mut prev_file.space_after {
                        Some(FreeSpace::Empty { ref mut size }) => *size += last_file.size,
                        Some(FreeSpace::Packed {
                            ref mut free_space,
                            files: _,
                        }) => *free_space += last_file.size,
                        None => (),
                    }
                }
                files.insert(j, last_file);
            }
        }

        if len == 0 || i == 0 {
            break;
        }

        i -= 1;
    }

    let blocks = files
        .into_iter()
        .map(|f| {
            let mut blocks = [Some(f.id)].repeat(f.size);

            match f.space_after {
                Some(FreeSpace::Packed { free_space, files }) => {
                    blocks.extend(files.into_iter().flat_map(|f| [Some(f.id)].repeat(f.size)));
                    blocks.extend([None].repeat(free_space));
                }
                Some(FreeSpace::Empty { size }) => blocks.extend([None].repeat(size)),
                None => (),
            }

            blocks
        })
        .flatten()
        .collect::<Vec<Option<usize>>>();

    println!("{:#?}", &blocks);

    let checksum = blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + i * id.unwrap_or(0));

    Some(checksum as u64)
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
        assert_eq!(result, Some(2858));
    }
}
