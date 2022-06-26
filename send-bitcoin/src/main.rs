use std::io;
use rand::Rng;

fn send_bitcoin()
{
    println!("\nWe are going to send some Bitcoin!\n");

    let clients = vec!["Homer", "Marge", "Bart", "Lisa"];
    println!("\nWho do you want to send Bitcoin to?\n");
    for client in&clients{
        println!("{}", client);
    }
    print!("\n");

    let mut recipient =String::new();
    io::stdin().read_line(&mut recipient);
    if clients.contains(&recipient.trim()){

        println!("\nHow many Bitcoin do you want to send?\n");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

        println!("\nYou just sent {} Bitcoin to {}!\n", amount, recipient);

    } else{

        println!("\n{} isn't in your contacts!\n", recipient.trim());
    }

}

fn receive_bitcoin()
{

    println!("\nWe are going to receive some Bitcoin!\n");
    
    let amount = rand::thread_rng().gen_range(1, 10);
    println!("\nYou just received {} Bitcoin!\n", amount);
}

fn exit_console()
{
    println!("\nInvaild option, must be (s) or (r)!\n");
}

fn console()
{

    println!("\nLet's have fun with Bitcoin!\n");
    
    println!("\nDo you want to send (s) or receive (r) Bitcoin?\n");
    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("s"){
        send_bitcoin()
    }else if command.trim().eq("r"){
        receive_bitcoin()
    }else{
        exit_console()
    }
}

fn main() 
{
    console()
}
