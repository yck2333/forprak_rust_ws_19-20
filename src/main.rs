use miniBFA::{BFA, Student};

use std::fs::File;
use std::io::Write;
use Foprak::{Student, BFA};


fn main() {
    //file_bfa();
    student_block();

}

pub fn student_block(){
    /* let student_1 = Student::new("Ling","Feng",2719983);
     let student_2 = Student::new("Yanping","Long",2767970);
     let mut bfa_1 = BFA::new(200,"Studentlist.txt");

     let nr_1 = bfa_1.reserve();
     bfa_1.update(nr_1,Student::vec_to_block(student_1.serialize()));

     let nr_2 = bfa_1.reserve();
     bfa_1.update(nr_2,Student::vec_to_block(student_2.serialize()));

     let student_3 = Student::deserialize( & bfa_1.get(nr_1).contents);
     println!("{:?}", student_3);*/
    let s1 = Student::new("ling", "feng", 2719983);
    let s2 = Student::new("yanping", "long", 2767970);
    let serialized1:Vec<u8> = s1.serialize();
    let serialized2:Vec<u8> = s2.serialize();
    //let block1 =
    let deserialized1:Student = Student::deserialize(&serialized1);
    let deserialized2:Student = Student::deserialize(&serialized2);
    println!("serialized1 = {:?}", serialized1);
    println!("serialized2 = {:?}", serialized2);
    println!("deserialized1 = {:?}", deserialized1);
    println!("deserialized2 = {:?}", deserialized2);
}

pub fn file_bfa(){
    let mut file = File::create("Hello.txt").expect("error");
    let mut bfa_1 = BFA::new(5, "Hello.txt");
    file.write_all(b"HelloWorldd").expect("error");
    //bfa_1.get_metadaten();
    //print!("{}",bfa_1.file.metadata().expect("error").len());

    let mut bfa_2 = BFA::new(5,"copy.txt");

    let total = bfa_1.file.metadata().unwrap().len() as usize / bfa_1.block_size + 1;
    for i in 0 .. total as u64{
        let block = bfa_1.get(i as usize);
        let nr = bfa_2.reserve();
        println!("{}",nr);

        bfa_2.update(nr, block);
    }

    bfa_1.close();
    bfa_2.close();
}
