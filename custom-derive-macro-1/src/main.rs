trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}


struct Swap {
    qty_1: u32,
    qty_2: u32,
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.qty_1.to_le_bytes()); //be stands for Big Endian, Big Endian stores the most significant byte first
        result.extend_from_slice(&self.qty_2.to_le_bytes());
        result
    }

    // Big Endian	    [0, 0, 0, 1]
    // Little Endian	[1, 0, 0, 0]

}

fn main(){

    let s = Swap {
        qty_1: 1,
        qty_2: 2
    };

    let bytes = s.serialize();
    print!("{:?}", bytes);
}
