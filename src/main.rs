fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://docs.rs/clap/latest/clap/";
    let requete = reqwest::blocking::get(url)?;
    let body = requete.text()?;

    println!("{}", body);

    Ok(())
}