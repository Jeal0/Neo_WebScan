# Journal IA.md de mon projet neo_WebScan

## 09/11 · Claude - Demande des meilleurs dépendances utiles à mon projet neo_WebScan

Demande : [Explication de mon projet] - Quels sont les dépendences les plus adaptés ?
Gardé   : Les dépendances Reqwest et clap 
Commit  : "ajout des dépendances Reqwest et Clap"

## 09/11 · Chatgpt - explication d'une erreur à la compilation

Demande : erreur à la compilation du programme
Gardé   : ok(), permettant de validé le bon fonctionnement du programme
Corrigé : ajout de la fonction ok() au programme
Commit  : "première requête sur une URL" [IA]

## 10/11 - claude - detection une faille xss

Demande : comment ajouter au code la detection une faille xss en Rust
Gardé : format! - Ok(reponse) - let contenu = reponse.text()?; - if contenu.contains(payload)
corrigé : Ok(reponse) au lieu de reponse seul 
commit : "ajout de scan de vulnérabilités de bases : https, headers, chemins d'accès et une faille xss"