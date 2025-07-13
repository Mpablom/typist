use rand::seq::SliceRandom;
use rand::Rng;

const LESSONS: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "El sol brilla sobre el campo verde y calido.",
    "Pack my box with five dozen liquor jugs.",
    "Cada dia trae nuevas oportunidades para mejorar.",
    "How razorback-jumping frogs can level six piqued gymnasts.",
    "Mi gato juega con una pelota de papel.",
    "Sphinx of black quartz, judge my vow.",
    "Hoy es un buen dia para comenzar de nuevo.",
    "Just keep typing and do not look back.",
    "La vida es mejor cuando uno aprende cosas nuevas.",
    "Programming is the art of telling another human what one wants the computer to do.",
    "La practica constante es el camino hacia la maestria en mecanografia.",
    "The five boxing wizards jump quickly.",
    "Aprendiendo a teclear con velocidad y precision.",
    "Jackdaws love my big sphinx of quartz.",
    "El conocimiento es la llave que abre las puertas del futuro.",
    "Typing tutor applications help improve your keyboard skills.",
    "La paciencia y la perseverancia son virtudes del buen mecanografo.",
    "Bright vixens jump; dozy fowl quack.",
    "Cada error es una oportunidad para mejorar tu precision.",
];
pub fn get_random_lesson() -> String {
    let mut rng = rand::thread_rng();
    let mut lesson = String::new();

    for _ in 0..10 {
        let sentence = LESSONS.choose(&mut rng).unwrap_or(&"");
        lesson.push_str(sentence);
        lesson.push(' ');

        if lesson.split_whitespace().count() % 5 == 0 && rng.gen_bool(0.5) {
            lesson.push('\n');
        }
    }

    lesson
}
