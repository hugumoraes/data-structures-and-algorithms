class Array:
    def __init__(self, length = 0):
        self.length = length
        self.data = {}

    def get(self, index):
        return self.data[index]
    
    def push(self, item):
        self.data[self.length] = item
        self.length += 1

        return self.length
    
    def pop(self):
        if self.length == 0:
            return None

        last_item = self.data[self.length - 1]

        del self.data[self.length - 1]

        self.length -= 1

        return last_item
    
    def delete(self, index):
        item = self.data[index]

        self.shift_items(index)

        return item
    
    def shift_items(self, index):
        for i in range(index, self.length - 1):
            self.data[i] = self.data[i + 1]

        del self.data[self.length - 1]

        self.length -= 1

    
names = Array()

print(names.__dict__)