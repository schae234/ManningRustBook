#![allow(unused_variables)]

#[allow(dead_code)]
fn listing_3_1(){

    type File = String;
    
    fn open(f: &mut File) -> bool {
        true
    }
    
    fn close(f: &mut File) -> bool {
        true
    }
    
    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }
    
    fn main() {
        let mut f1 = File::from("f1.txt");
        open(&mut f1);
        //read(f1, vec![]);
    close(&mut f1);
    }
}

#[allow(dead_code)]
fn listing_3_3(){
    #[derive(Debug)]
    struct File {
    name: String,
    data: Vec<u8>,
    }
    
    fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    
    // Must use a reference here 
    let f1_name = &f1.name;
    // the len fn auto-borrows
    let f1_length = f1.data.len();
    
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
    }
}


fn listing_3_6(){
    #[derive(Debug)]
    struct File {
    name: String,
    data: Vec<u8>,
    }
    
    fn open(f: &mut File) -> bool {
        true
    }
    
    fn close(f: &mut File) -> bool {
        true
    }
    
    fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
    
    fn main() {
        // Create the file struct
        let mut f2 = File {
            name: String::from("2.txt"),
            data: vec![114, 117, 115, 116, 33],
        };
    
        // allocate a buffer
        let mut buffer: Vec<u8> = vec![];
    
        // the open and close functions are just place
        // holders right now
        open(&mut f2);
        let f2_length = read(&f2, &mut buffer);
        close(&mut f2);
    
        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f2);
        println!("{} is {} bytes long", &f2.name, f2_length);
        println!("{}", text)
    }
}
