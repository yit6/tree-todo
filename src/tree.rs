use tui::widgets::ListItem;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub done: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            done: false,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn to_list_items_without_self(&self) -> Vec<ListItem> {
        let mut output = Vec::new();
        for child in &self.children {
            output.append(&mut child.convert_to_list_items(0));
        }
        output
    }

    fn convert_to_list_items(&self, depth: usize) -> Vec<ListItem> {
        let mut string = "    ".repeat(depth);
        string += "[";
        if self.done {
            string += "*]";
        } else {
            string += " ]";
        }
        string += &self.name;
        string += "\n";
        let mut output = Vec::new();
        output.push(ListItem::new(string));
        for child in &self.children {
            output.append(&mut child.convert_to_list_items(depth+1));
        }
        output
    }

    pub fn to_done_list(&self) -> Vec<&bool> {
        let mut output = Vec::new();
        for child in &self.children {
            output.append(&mut child.convert_to_done_list());
        }
        output
    }

    fn convert_to_done_list(&self) -> Vec<&bool> {
        let mut output = Vec::new();
        output.push(&self.done);
        for child in &self.children {
            output.append(&mut child.convert_to_done_list());
        }
        output
    }

    pub fn set_done(&mut self, value: bool) {
        self.done = value;
    }
}
