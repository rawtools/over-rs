use std::path::PathBuf;

use clap::Args;

use anyhow::Result;
// use display_tree::{AsTree, CharSet, DisplayTree, StyleBuilder};
use crate::cli::CLI;
use crate::overlays::Repository;


#[derive(Args, Debug)]
pub struct Params {
    #[clap(short, long, help = "Display as tree")]
    tree: bool,
}


// // A tree representing a numerical expression.
// #[derive(DisplayTree)]
// enum Tree {
//     Dir {
//         #[node_label]
//         name: String,
//         #[tree]
//         children: Vec<Box<Self>>,
//     },
//     Overlay {
//         #[node_label]
//         name: String,
//     },
// }

// fn build_tree(node: &mut Dir, parts: &Vec<String>, depth: usize) {
//     if depth < parts.len() {
//         let item = &parts[depth];

//         let mut dir = match node.find_child(&item) {
//             Some(d) => d,
//             None => {
//                 let d = Dir::new(&item);
//                 node.add_child(d);
//                 match node.find_child(&item) {
//                     Some(d2) => d2,
//                     None => panic!("Got here!"),
//                 }
//             }
//         };
//         build_tree(&mut dir, parts, depth + 1);
//     }
// }


pub async fn execute(cli: &CLI, args: &Params) -> Result<()> {
    if cli.debug {
        println!("{:#?}", cli);
        println!("{:#?}", args);
    }
    
    // let repo = Repository::new( PathBuf::from(&cli.home) );

    for overlay in Repository::new( PathBuf::from(&cli.home) ).overlays()? {
        println!("{}", overlay.name);
    }

    Ok(())
}



// use termtree::Tree;

// use std::path::Path;
// use std::{env, fs, io};

// fn label<P: AsRef<Path>>(p: P) -> String {
//     p.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
// }

// fn tree<P: AsRef<Path>>(p: P) -> io::Result<Tree<String>> {
//     let result = fs::read_dir(&p)?.filter_map(|e| e.ok()).fold(
//         Tree::root(label(p.as_ref().canonicalize()?)),
//         |mut root, entry| {
//             let dir = entry.metadata().unwrap();
//             if dir.is_dir() {
//                 root.push(tree(entry.path()).unwrap());
//             } else {
//                 root.push(Tree::root(label(entry.path())));
//             }
//             root
//         },
//     );
//     Ok(result)
// }
