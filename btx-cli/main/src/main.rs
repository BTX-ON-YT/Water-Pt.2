fn main() {
    println!("------------");
    println!("- BTX CLI: -");
    println!("------------");

    println!("\n");
    println!("\n");

    let mut line = String::new();
   println!("Options :\n1) list all projects by me");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   if line == "1\n" {
        println!("Projects :");
        println!("1) niceJS");
        println!("2) project2");
        println!("3) project3");
        println!("4) project4");
        println!("5) project5");
        println!("6) project6");

        println!("\n");
        let mut line2 = String::new();
        println!("Select a project to view it :");
        let b2 = std::io::stdin().read_line(&mut line).unwrap();

   }

}
