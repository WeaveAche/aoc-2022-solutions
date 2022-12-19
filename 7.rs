use std::fs;
use std::default::Default;
use std::collections::HashMap;

struct Node {
    name : String,
    isdir : bool,
    node_size : usize,
    children : Vec <String>,
    parent : String,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            name : String::new(),
            isdir : true,
            node_size : 0,
            children : Vec::new(),
            parent : String::from("")
        }
    }
}

fn dfs(tree : &mut HashMap <String, Node>, curr : String) -> usize {
    let curr_node = tree.get_mut(&curr).unwrap();
    let mut total : usize = curr_node.node_size;

    let children = curr_node.children.clone();

    for child in children {
        total += dfs(tree, child.clone());
    }

    let mut curr_node = tree.get_mut(&curr).unwrap();
    curr_node.node_size = total;

    total
}

fn construct_tree(contents : &String) -> HashMap <String, Node> {
    let mut name_to_node : HashMap <String, Node> = HashMap::new();

    let mut root : Node = Default::default();
    root.name = String::from("/");

    name_to_node.insert(String::from("/"), root);
    let mut pwd : String = String::from("/");

    for line in contents.lines() {
        let line_split : Vec <&str> = line.split(" ").collect();

        if line_split[0] != "$" {
            if line_split[0] == "dir" {
                let mut dir : Node = Default::default();

                let mut name : String = pwd.clone();
                name.push_str(line_split[1]);
                name.push_str("/");

                dir.name = name;
                dir.parent = pwd.clone();

                name_to_node.get_mut(&pwd).unwrap().children.push(dir.name.clone());
                name_to_node.insert(dir.name.clone(), dir);
            } else {
                let mut file : Node = Default::default();

                let mut name : String = pwd.clone();
                name.push_str("/");
                name.push_str(line_split[1]);

                file.name = name;
                file.parent = pwd.clone();
                file.isdir = false;
                file.node_size = line_split[0].parse::<usize>().unwrap();

                name_to_node.get_mut(&pwd).unwrap().children.push(file.name.clone());
                name_to_node.insert(file.name.clone(), file);
            }
        } else { 
            if line_split[1] == "cd" {
                match line_split[2] {
                    ".." => {
                        let mut vec : Vec <&str> = pwd.split("/").collect();
                        vec.pop();
                        vec.pop();

                        pwd = vec.join("/");
                        pwd.push_str("/");
                    },
                    "/" => pwd = String::from("/"),
                    other => {
                        let mut name : String = pwd.clone();
                        name.push_str(other);
                        name.push_str("/");

                        pwd = name;
                    }
                }
            }
        }
    }

    dfs(&mut name_to_node, "/".to_string());
    name_to_node
}


fn part1(contents : &String) -> usize {
    let tree = construct_tree(contents);

    let mut ans = 0;
    for (_name, node) in &tree {
        if node.isdir && node.node_size < 100000 {
            ans += node.node_size;
        }
    }

    ans
}

fn part2(contents : &String) -> usize {
    let tree = construct_tree(contents);

    let disk_size : usize = 70000000;
    let used_space : usize = tree[&"/".to_string()].node_size;
    let available_space : usize = disk_size - used_space;
    let space_needed : usize = 30000000 - available_space;

    let mut ans = usize::MAX;
    for (_name, node) in &tree {
        if node.isdir && node.node_size >= space_needed {
            if node.node_size < ans {
                ans = node.node_size;
            }
        }
    }

    ans
}

fn main() {
    let contents : String = fs::read_to_string("input")
        .expect("Couldn't read file");


    println!("part1 ans: {}", part1(&contents));
    println!("part2 ans: {}", part2(&contents));
}
