use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = HashMap::new();
    let input = std::fs::read_to_string("input.txt")?;
    let entries = input
        .lines()
        .filter_map(|line| {
            let mut splitted = line.split('-');
            if let Some(a) = splitted.next() {
                if let Some(b) = splitted.next() {
                    graph.insert(a.to_string(), vec![]);
                    graph.insert(b.to_string(), vec![]);
                    return Some((a, b));
                }
            }
            None
        })
        .collect::<Vec<(&str, &str)>>();
    for (a, b) in entries {
        graph.get_mut(a).unwrap().push(b.to_string());
        graph.get_mut(b).unwrap().push(a.to_string());
    }
    println!("A: {}", visit(&mut graph, false));
    println!("B: {}", visit(&mut graph, true));
    Ok(())
}

fn visit(graph: &mut HashMap<String, Vec<String>>, allow_small_twice: bool) -> usize {
    let small_caves = graph.keys().filter(|s| is_small_cave(s)).count();
    let mut counter = 0usize;
    let mut visited = graph
        .keys()
        .map(|x| (x.clone(), 0usize))
        .collect::<HashMap<String, usize>>();
    visit_r(
        "start".to_string(),
        graph,
        small_caves,
        &mut visited,
        &mut counter,
        allow_small_twice,
    );
    counter
}

fn visit_r(
    current: String,
    graph: &HashMap<String, Vec<String>>,
    small_caves: usize,
    visited: &mut HashMap<String, usize>,
    counter: &mut usize,
    allow_small_twice: bool,
) {
    for e in graph.get(&current).unwrap() {
        match e.as_str() {
            "start" => continue,
            "end" => {
                *counter += 1;
                continue;
            }
            _ => {
                let visited_twice_small =
                    allow_small_twice && visited.iter().any(|(s, v)| is_small_cave(s) && *v == 2);
                if *visited.get(e).unwrap() == 0
                    || (*visited.get(e).unwrap() > 0 && !is_small_cave(e))
                    || (*visited.get(e).unwrap() < 2
                        && is_small_cave(e)
                        && !visited_twice_small
                        && allow_small_twice)
                {
                    *visited.get_mut(e).unwrap() += 1;
                    visit_r(
                        e.clone(),
                        graph,
                        small_caves,
                        visited,
                        counter,
                        allow_small_twice,
                    );
                    *visited.get_mut(e).unwrap() -= 1;
                }
            }
        }
    }
}

fn is_small_cave(input: &str) -> bool {
    input.contains(|c| matches!(c, 'a'..='z')) && (input != "start" && input != "end")
}
