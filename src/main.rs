use std::io;

fn main() {
   

   // Récupérer la phrase de l'utilisateur
   println!("Veuillez entrer votre phrase :");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Erreur de la lecture lors de l'entré");

   let trimmed_input = input.trim();

   // Calculer le nombre de voyelles
   let vowel_count = trimmed_input.chars().filter(|c| "aeiouyAEIOUY".contains(*c)).count();
   println!("Nombre de voyelles dans la phrase: {}",vowel_count);

   // Gérer la question du palindrome
   let clear_input: String = trimmed_input.chars().filter(|c| c.is_alphabetic()).collect();
   let is_palindrom = clear_input.to_lowercase().chars().eq(clear_input.to_lowercase().chars().rev());
   if is_palindrom {
      println!("La phrase est un palindrome");
   } else {
      println!("La phrase n'est pas un palindrome");
   }

   // Tri des mots par longueur
   let mut words: Vec<&str> = trimmed_input.split_whitespace().collect();
   words.sort_by_key(|&word| word.len());
   println!("Mots triés par longueur: {:?}", words);

   // Conversion en acronyme
   let acronym: String = trimmed_input.split_whitespace().map(|word| word.chars().next().unwrap_or(' ')).collect();
   println!("Acronyme de la phrase : {}", acronym);
}
