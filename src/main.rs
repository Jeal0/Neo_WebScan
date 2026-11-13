fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Adresse du site SANS "/" à la fin
    let site = "https://0adb008704ef963681f70cf100bd001f.web-security-academy.net/";
    let parametre = "search";
    let max_pages = 50;

    let mut a_visiter = vec![format!("{}/", site)];
    let mut deja_vus: Vec<String> = Vec::new();

    println!("Scan de : {}\n", site);


    if site.starts_with("http://") {
        println!("This website is not a https");
    }

    let chemins = [".git/HEAD", ".env", "phpinfo.php", "backup.zip", "admin/"];
    for chemin in chemins {
        let url_test = format!("{}/{}", site, chemin);
        if let Ok(reponse) = reqwest::blocking::get(&url_test) {
            if reponse.status().is_success() {
                println!("some access to file or folder : {}", url_test);
            }
        }
    }

    while let Some(page) = a_visiter.pop() {

        if deja_vus.contains(&page) {
            continue;
        }
        if deja_vus.len() >= max_pages {
            break;
        }
        deja_vus.push(page.clone());

        println!("\n--- Page : {}", page);

        let requete = match reqwest::blocking::get(&page) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let headers = requete.headers().clone();
        let body = match requete.text() {
            Ok(t) => t,
            Err(_) => continue,
        };

        // En-têtes de sécurité
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


        for morceau in body.split("href=\"").skip(1) {
            let lien = morceau.split('"').next().unwrap_or("");
            if lien.starts_with('/') && !lien.starts_with("//") {
                a_visiter.push(format!("{}{}", site, lien));
            }
        }

        let payload = "<script>alert(1)</script>";
        let separateur = if page.contains('?') { "&" } else { "?" };
        let url_xss = format!("{}{}{}={}", page, separateur, parametre, payload);
        if let Ok(reponse) = reqwest::blocking::get(&url_xss) {
            if let Ok(contenu) = reponse.text() {
                if contenu.contains(payload) {
                    println!("There is a XSS vulnerability : {}", url_xss);
                }
            }
        }
    }

    println!("\nScan finish ({} pages analyzed)", deja_vus.len());

    Ok(())
}