/// Scrip for the Input and Output commands

use std::io;
use crate::order_book_mod::Order;

impl Order{

    pub fn new_order()->Order{


        // Side
        let side_in: bool= loop {
            println!("Enter side: [a] for ask [b] for bid");
            let mut ab =String::new();
            io::stdin().read_line(&mut ab).expect("Failed to read input side");
            ab = ab.trim().to_string();
            if ab.eq("b"){
                let side_in= true;
                break side_in;
            }else if ab.eq("a"){
                let side_in= false;
                break side_in;
            }
            println!("Retry")
        };



        let tr_price: u32 = loop {
            let mut p_input = String::new();
            println!("Enter price: ");
    
            io::stdin().read_line(&mut p_input).expect("Failed to read input");
    
            match p_input.trim().parse() {
                Ok(num) => break num, // Exit the loop with the valid number
                Err(_) => println!("Invalid price! Please enter a valid u32."),
            }
        };



/*
        // PRICE
        let mut p_input = String::new();

        println!("Enter price: ");
        
        io::stdin().read_line(&mut p_input).expect("Failed to read price");
    
        // Trim whitespace and attempt to parse as u32
        let tr_price: u32 = match p_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid price! Please enter a valid u32.");
                return;
            
        };


        // Size
        let mut s_input = String::new();

        println!("Enter size: ");
        
        io::stdin().read_line(&mut s_input).expect("Failed to read size");
    
        // Trim whitespace and attempt to parse as u32
        let tr_size: u32 = match s_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid size! Please enter a valid u32.");
                return;
            }
        };
*/

        let tr_size: u32 = loop {
            let mut s_input = String::new();
            println!("Enter size: ");
    
            io::stdin().read_line(&mut s_input).expect("Failed to read input");
    
            match s_input.trim().parse() {
                Ok(num) => break num, // Exit the loop with the valid number
                Err(_) => println!("Invalid size! Please enter a valid u32."),
            }
        };

        println!("NEW ORDER {:#?}",Order{
            id: 50,
            modify: 0,
            partial: 0,
            size: tr_size,
            price: tr_price,
            side: side_in,
            o_type: 1
        });

        Order{
            id: 50,
            modify: 0,
            partial: 0,
            size: tr_size,
            price: tr_price,
            side: side_in,
            o_type: 1
        }
    }
}

