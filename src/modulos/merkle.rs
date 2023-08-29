pub mod encrypt;

pub struct Tree{
    pub root: Node
}

pub struct Node{
    pub hash: String
}

pub fn make_merkle_tree( transactions: &Vec<String>) -> Tree{
    let mut node_list: Vec<Node> = Vec::new();
    for transaction in transactions.iter() {
        node_list.push(Node{
            hash: transaction.clone()
        })
    }

    while(node_list.len() > 2){
        let mut aux_node_list:  Vec<Node> = Vec::new();
        for(index, value) in node_list.iter().enumerate() {
            if index % 2 == 0 {
                aux_node_list.push(Node{
                    hash: generate_new_hash(&node_list[index], &node_list[index + 1]),
                })
            }
        }
        node_list.clear();
        for(index, value) in aux_node_list.iter().enumerate(){
            node_list.push(Node { hash: value.hash.clone() })
        }
    }

    let merkle_root = Node{
        hash: generate_new_hash(&node_list[0], &node_list[1])
    };

    return Tree{
        root: merkle_root
    }
}

fn generate_new_hash(node1: &Node, node2: &Node) -> String{
    let hash_concatenated = node1.hash.clone() + &node2.hash;
    println!("hash1 : {}", node1.hash.clone());
    println!("hash2 : {}", node2.hash.clone());
    println!("hash3 : {}", hash_concatenated);
    println!("new hash : {}", encrypt::encrypt(&hash_concatenated));
    return encrypt::encrypt(&hash_concatenated);
}