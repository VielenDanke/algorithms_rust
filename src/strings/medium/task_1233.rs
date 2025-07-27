use std::collections::HashMap;

pub struct Solution;

#[derive(Debug, Clone)]
pub struct Tree {
    val: String,
    is_leaf: bool,
    children: HashMap<String, Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            val: "".to_string(),
            is_leaf: false,
            children: HashMap::new(),
        }
    }

    pub fn build(val: String, is_leaf: bool) -> Self {
        Tree {
            val,
            is_leaf,
            children: HashMap::new(),
        }
    }
}

impl Solution {
    pub fn remove_subfolders_prefix_tree(mut folders: Vec<String>) -> Vec<String> {
        folders.sort_unstable_by_key(|v| v.len());

        let mut result = vec![];

        let mut root = Tree::new();

        let mut temp = &mut root;

        for folder in folders {
            let split_folders = folder
                .split("/")
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<String>>();

            for (i, folder) in split_folders.iter().enumerate() {
                let possible_child = temp
                    .children
                    .entry(folder.clone())
                    .or_insert(Tree::build(folder.clone(), false));

                if possible_child.is_leaf {
                    break;
                }

                if i == split_folders.len() - 1 {
                    possible_child.is_leaf = true;
                }
                temp = possible_child;
            }
            temp = &mut root;
        }

        fn dfs(tree: &Tree, result: &mut Vec<String>, temp: &mut Vec<String>) {
            if tree.is_leaf {
                result.push(format!("/{}", temp.join("/")));
                return;
            }
            for (_, val) in tree.children.iter() {
                temp.push(val.val.clone());
                dfs(val, result, temp);
                temp.pop();
            }
        }
        dfs(&root, &mut result, &mut vec![]);

        result
    }

    pub fn remove_subfolders_faster(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut answer = vec![folder[0].to_string()];

        for i in 1..folder.len() {
            if !folder[i].starts_with(&format!("{}/", answer.last().unwrap())) {
                answer.push(folder[i].to_string());
            }
        }

        answer
    }
}
