use std::collections::HashMap;
#[derive(Debug)]
struct MyArray {
    data: HashMap<i32, i32>,
    length: i32,
}

impl MyArray {
    fn new() -> Self {
        Self {
            length: 0,
            data: HashMap::new(),
        }
    }

    fn get(&self, index: i32) -> Option<&i32> {
        self.data.get(&index)
    }

    fn push(&mut self, item: i32) {
        self.data.insert(self.length, item);

        self.length += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        if self.length == 0 {
            // Handle the case when there are no elements to pop
            return None;
        }

        let last_index = self.length - 1;
        let last_item = self.data.get(&last_index).cloned();

        self.data.remove(&last_index);

        self.length -= 1;

        last_item
    }

    fn delete(&mut self, index: i32) -> Option<i32> {
        let item = self.data.get(&index).cloned();

        self.shift_items(index);

        item
    }

    fn shift_items(&mut self, index: i32) {
        if index >= self.length - 1 {
            return;
        }

        for i in index..(self.length - 1) {
            let next_item = self.data.get(&(i + 1)).cloned();

            if let Some(next_item_value) = next_item {
                if let Some(value) = self.data.get_mut(&i) {
                    *value = next_item_value;
                } else {
                    return;
                }
            } else {
                return;
            }
        }

        self.data.remove(&(self.length - 1));

        self.length -= 1;
    }
}

fn main() {
    let mut my_array = MyArray::new();

    my_array.push(1);
    my_array.push(2);
    my_array.push(3);
    my_array.push(4);
    my_array.push(5);
    my_array.push(6);

    let last_item = my_array.pop();
    let deleted_item = my_array.delete(0);
    let first_item = my_array.get(0);

    println!("First item: {:?}", first_item);
    println!("Last item: {:?}", last_item);
    println!("Deleted item: {:?}", deleted_item);
    println!("my_array: {:?}", my_array);
}
