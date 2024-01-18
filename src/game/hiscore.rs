use crate::{gfx::VertexArrayObject, shader::ShaderProgram, ui};
use std::{
    fs::File,
    io::{Read, Write},
};

const MAX_HIGHSCORES: usize = 5;

//Checks if a score is a new high score
pub fn is_new_highscore(score: u32, highscores: &Vec<u32>) -> bool {
    for highscore in highscores {
        if score > *highscore {
            return true;
        }
    }

    highscores.len() < MAX_HIGHSCORES
}

//Finds the smallest highscore, removes it, and then adds
//the new highscore value
pub fn add_highscore(score: u32, highscores: &mut Vec<u32>) {
    //If we have less highscores than the maximum number
    //of highscores that the game stores, just add the
    //score to the high score list
    if highscores.len() < MAX_HIGHSCORES {
        highscores.push(score);
        highscores.sort();
        return;
    }

    let mut min = highscores[0];
    let mut index = 0;
    for (i, highscore) in highscores.iter().enumerate() {
        if *highscore < min {
            min = *highscore;
            index = i;
        }
    }

    if highscores[index] < score {
        highscores[index] = score;
    }
    highscores.sort();
}

//Loads high scores from a file
//we assume that this file has a score printed on each line
pub fn load_highscores(path: &str) -> Vec<u32> {
    let mut highscores = vec![];

    match File::open(path) {
        Ok(mut file) => {
            let mut buf = String::new();

            let res = file.read_to_string(&mut buf);
            if let Err(msg) = res {
                eprintln!("{msg}");
            }

            buf.lines().for_each(|line| {
                if let Ok(val) = line.parse() {
                    highscores.push(val)
                }
            });
        }
        Err(msg) => eprintln!("{msg}"),
    }

    highscores.sort();

    //Pop off any extra elements
    while highscores.len() > MAX_HIGHSCORES {
        highscores.pop();
    }

    highscores
}

//Writes highscores to a file
pub fn write_highscores(path: &str, highscores: &Vec<u32>) {
    if highscores.is_empty() {
        return;
    }

    let mut highscore_file_contents = String::new();

    for score in highscores {
        highscore_file_contents.push_str(score.to_string().as_str());
        highscore_file_contents.push('\n');
    }

    match File::create(path) {
        Ok(mut file) => {
            let res = file.write(highscore_file_contents.as_bytes());
            if let Err(msg) = res {
                eprintln!("{msg}");
            }
        }
        Err(msg) => eprintln!("{msg}"),
    }
}

pub fn display_hiscores(
    rect_vao: &VertexArrayObject,
    text_shader: &ShaderProgram,
    highscores: &Vec<u32>,
) {
    for (i, score) in highscores.iter().enumerate() {
        ui::display_ascii_text_centered(
            rect_vao,
            text_shader,
            format!("{}: {}", highscores.len() - i, score).as_bytes(),
            0.0,
            i as f32 * 16.0 * 2.5 - highscores.len() as f32 / 2.0 * 16.0 * 2.5,
            16.0,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_hiscore() {
        let hiscores: Vec<u32> = vec![];
        assert!(is_new_highscore(0, &hiscores));
    }

    #[test]
    fn test_add_scores() {
        let mut hiscores: Vec<u32> = vec![];
        add_highscore(1, &mut hiscores);
        add_highscore(2, &mut hiscores);
        add_highscore(3, &mut hiscores);
        add_highscore(0, &mut hiscores);
        add_highscore(4, &mut hiscores);
        add_highscore(5, &mut hiscores);
        add_highscore(0, &mut hiscores);
        assert_eq!(hiscores, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_check_is_highscore1() {
        let hiscores: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert!(!is_new_highscore(1, &hiscores));
    }

    #[test]
    fn test_check_is_highscore2() {
        let hiscores: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert!(is_new_highscore(6, &hiscores));
    }

    #[test]
    fn test_check_is_highscore3() {
        let hiscores: Vec<u32> = vec![1, 2, 3, 4, 5];
        assert!(is_new_highscore(2, &hiscores));
    }

    #[test]
    fn test_check_is_highscore4() {
        let mut hiscores: Vec<u32> = vec![];
        add_highscore(1, &mut hiscores);
        add_highscore(3, &mut hiscores);
        add_highscore(4, &mut hiscores);
        add_highscore(5, &mut hiscores);
        add_highscore(6, &mut hiscores);
        let is_new = is_new_highscore(2, &hiscores);
        add_highscore(2, &mut hiscores);
        assert!(is_new);
        assert!(!is_new_highscore(2, &hiscores));
        assert_eq!(hiscores, vec![2, 3, 4, 5, 6]);
    }
}
