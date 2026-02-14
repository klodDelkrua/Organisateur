mod functions; 
mod user_interface;
mod storage;
fn main() {
    println!("Bienvenue sur Organise");
    println!("Ideal pour organiser vos journees et vos idees");
    user_interface::run_application();
}
