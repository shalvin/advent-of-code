use petgraph::{
    prelude::*,
    visit::{NodeRef, Walker},
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let mut graph: StableDiGraph<u32, i32> = StableDiGraph::new();

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .map(|h| graph.add_node(h))
                .collect()
        })
        .collect::<Vec<Vec<NodeIndex>>>();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let this_node = map[row][col];

            let row = row as i32;
            let col = col as i32;

            [(0i32, -1i32), (1i32, 0i32), (0i32, 1i32), (-1i32, 0i32)]
                .into_iter()
                .filter_map(|(x, y)| {
                    map.get(row.wrapping_add(y) as usize)
                        .and_then(|r| r.get(col.wrapping_add(x) as usize))
                })
                .for_each(|neighbour| {
                    graph.add_edge(
                        this_node,
                        *neighbour,
                        graph[*neighbour].saturating_sub(graph[this_node]) as i32,
                    );
                })
        }
    }

    let trail_graph =
        graph.filter_map(|_, n| Some(n), |_, e| if *e == 1 { Some(*e) } else { None });

    let mut score = 0;
    for starting_node in trail_graph.node_indices().filter(|n| *trail_graph[*n] == 0) {
        let dfs = Dfs::new(&trail_graph, starting_node);

        score += dfs
            .iter(&trail_graph)
            .filter(|n| *trail_graph[*n] == 9)
            .count();
    }

    Some(score as u64)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
