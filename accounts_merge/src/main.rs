#![allow(unused_variables, clippy::all, unused_mut, dead_code)]

use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

struct Node {
    user_name: String,
    email: String,
    adj: Vec<usize>,
}

struct Account {
    user_name: String,
    emails: Vec<String>,
}

// accounts * (1 + emails) + emails + (n(n-1)/2) + n * eloge
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // First build the node graph
    let mut email_to_node_id: HashMap<String, usize> = HashMap::new();
    let mut nodes: Vec<Node> = vec![];

    // O(accounts * (1 + emails))
    for account_data in accounts {
        let mut root: Option<usize> = None;

        let user_name = &account_data[0];
        for email in account_data.iter().skip(1) {
            let mut email_node_id: usize;

            if let Some(id) = email_to_node_id.get(email) {
                // that node exists
                email_node_id = *id;
            } else {
                // node doesn't already exist. So make one
                let new_id = nodes.len();
                nodes.push(Node {
                    user_name: user_name.into(),
                    email: email.into(),
                    adj: vec![],
                });
                email_node_id = new_id;

                email_to_node_id.insert(email.into(), email_node_id);
            }

            match root {
                // we do have a root so set this node adjacent to that root
                Some(r) => {
                    nodes[r].adj.push(email_node_id);
                    nodes[email_node_id].adj.push(r);
                }

                // We have no root, so make this node the root.
                None => root = Some(email_node_id),
            }
        }
    }

    // Second run dfs to build the accounts list
    // O(emails + e)
    // For a fully connected graph n(n-1)/2
    // O(emails + (n(n-1)/2))
    let mut merged_accounts: Vec<Account> = vec![];

    fn dfs(current: usize, nodes: &Vec<Node>, visited: &mut HashSet<usize>, account: &mut Account) {
        if visited.contains(&current) {
            return;
        }
        visited.insert(current);

        account.emails.push(nodes[current].email.clone());

        for adj in &nodes[current].adj {
            dfs(*adj, nodes, visited, account);
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    for (i, node) in nodes.iter().enumerate() {
        if !visited.contains(&i) {
            let mut account = Account {
                user_name: node.user_name.clone(),
                emails: vec![],
            };
            dfs(i, &nodes, &mut visited, &mut account);

            merged_accounts.push(account);
        }
    }

    // Third sort the lists.
    // And build return format.
    // O(n * eloge)
    let mut return_format: Vec<Vec<String>> = vec![];
    for ac in &mut merged_accounts {
        ac.emails.sort();

        let mut v: Vec<String> = vec![];
        v.push(ac.user_name.clone());
        v.append(&mut ac.emails);

        return_format.push(v);
    }

    return_format
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn working() {
        let ret = accounts_merge(vec![
            vec![
                "John".into(),
                "johnsmith@mail.com".into(),
                "john_newyork@mail.com".into(),
            ],
            vec![
                "John".into(),
                "john00@mail.com".into(),
                "johnsmith@mail.com".into(),
            ],
            vec!["Mary".into(), "mary@mail.com".into()],
            vec!["John".into(), "johnnybravo@mail.com".into()],
        ]);

        assert_eq!(
            ret,
            vec![
                vec![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com"
                ],
                vec!["Mary", "mary@mail.com"],
                vec!["John", "johnnybravo@mail.com"]
            ]
        );
    }
}
