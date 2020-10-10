#[cfg(test)]
mod tests {
    #[test]
    fn printing_colored_text_to_the_terminal() {
        use ansi_term::Color;

        println!(
            "This is {} in color, {} in color and {} in color",
            Color::Red.paint("red"),
            Color::Blue.paint("blue"),
            Color::Green.paint("green")
        );
    }

    #[test]
    fn bold_text_in_terminal() {
        use ansi_term::Style;
        println!(
            "{} and this if not",
            Style::new().bold().paint("This is Bold")
        );
    }

    #[test]
    fn bold_and_colored_text_in_terminal() {
        use ansi_term::{Color, Style};
        println!(
            "{}, {} and {}",
            Color::Yellow.paint("This is colored"),
            Style::new().bold().paint("this is bold"),
            Color::Yellow.bold().paint("this is bold and colored")
        );
    }
}
