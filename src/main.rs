use winapi::um::winbase::SetThreadExecutionState;
use tokio::{time, io, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    const DURATION: time::Duration = time::Duration::from_secs(30);
    println!("Bonjour, le verrouillage de l'écran sera désactivé toutes les 30 secondes à partir de maintenant. \nVeuillez appuyer sur \"Enter\" pour fermer l'application.");

    let mut to_wait = time::sleep(DURATION);
    let mut input = io::stdin();
    let mut empty_buff = [0u8; 1];

    loop {
        tokio::select! {
            _ = to_wait => {
                to_wait = time::sleep(DURATION);
                unsafe { SetThreadExecutionState(2) };
            }
            _ = input.read(&mut empty_buff) => {
                break
            }
        }
    }
    println!("L'application s'est terminée avec succès");
}