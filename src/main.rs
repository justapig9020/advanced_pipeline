mod arthmatic_unit;
mod decoder;
mod display;
mod execution_path;
mod graph;
mod processor;
mod register;
mod result_bus;
use processor::Processor;
use std::io;

fn main() -> Result<(), String> {
    let program = vec![
        "addi R1, R0, #100", // R1 = 100
        "addi R2, R0, #200", // R2 = 200
        "add R3, R1, R2",    // R3 = 300
        "add R4, R1, R3",    // R4 = 400
        "add R3, R4, R3",    // R3 = 700
        "addi R1, R5, #400", // R1 = 400
        "add R5, R1, R2",    // R5 = 600
                             /* R1: 400
                              * R2: 200
                              * R3: 700
                              * R4: 400
                              * R5: 600
                              */
    ];
    let mut p = Processor::new();
    p.add_path("arth")?;
    p.add_path("arth")?;
    loop {
        let line = p.fetch_address();
        let inst = if line >= program.len() {
            "NOP"
        } else {
            program[line]
        };
        println!("Line {}:", line);
        println!("{}", p);
        if pause() {
            break;
        }
        p.next_cycle(inst)?;
    }
    println!("");
    println!("Emulation finished");
    println!("{:#?}", p);
    Ok(())
}

fn pause() -> bool {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().eq("exit")
}
