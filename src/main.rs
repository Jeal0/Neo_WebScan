fn main() -> Result<(), Box<dyn std::error::Error>> {

    let url = "https://0a1900d10372e2368195c59500c50050.web-security-academy.net/?search=";
    let requete = reqwest::blocking::get(url)?;

    let headers = requete.headers().clone();
    let body = requete.text()?;

    println!("Scan de : {}\n", url);


    if url.starts_with("http://") {
        println!("This website is not a https");
    }

    let en_tetes_attendus = [
        "content-security-policy",
        "x-frame-options",
        "x-content-type-options",
        "strict-transport-security",
    ];
    for nom in en_tetes_attendus {
        if !headers.contains_key(nom) {
            println!("no header : {}", nom);
        }
    }


    let chemins = [".git/HEAD", ".env", "phpinfo.php", "backup.zip", "admin/"];
    for chemin in chemins {
        let url_test = format!("{}{}", url, chemin);
        if let Ok(reponse) = reqwest::blocking::get(&url_test) {
            if reponse.status().is_success() {
                println!("some access to file or folder : {}", url_test);
            }
        }
    }

    let payload = "<script>alert(1)</script>";
    let url_xss = format!("{}?search={}", url, payload);
    if let Ok(reponse) = reqwest::blocking::get(&url_xss) {
        let contenu = reponse.text()?;
        if contenu.contains(payload) {
            println!("There is a XSS vulnerability");
        }
    }

    println!("\nScan finish ({} bytes analyzed)", body.len());

    Ok(())
}