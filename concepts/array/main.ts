class MyArray {
    length: number;
    data: Record<number, any>;

    constructor(length = 0) {
        this.length = length;
        this.data = {};
    }

    get(index: number) {
        return this.data[index];
    }

    push(item: unknown) {
        this.data[this.length] = item;

        this.length++;
    }

    pop() {
        const item = this.data[this.length - 1];

        delete this.data[this.length - 1];

        this.length--;

        return item;
    }

    delete(index: number) {
        const item = this.data[index];

        this.shiftItems(index);

        return item;
    }

    shiftItems(index: number) {
        for(let i = index; i < this.length; i++) {
            this.data[i] = this.data[i + 1];
        }

        delete this.data[this.length - 1];

        this.length--;
    }
}

const myArray = new MyArray();
