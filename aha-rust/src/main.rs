#[derive(Copy, Clone)]
enum ColorMode {
    Mode3Bit,
    Mode8Bit,
    Mode24Bit,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Mode3Bit
    }
}

#[derive(Copy, Clone)]
struct AnsiParsingState {
    foreground: i32,
    background: i32,
    bold: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    strike: bool,
    fc_mode: ColorMode,
    bc_mode: ColorMode,
    highlighted: bool,
    // inverse: bool,
    // invisible: bool,
}

impl Default for AnsiParsingState {
    fn default() -> Self {
        AnsiParsingState {
            foreground: -1,
            background: 0,
            bold: false,
            italic: false,
            underline: false,
            blink: false,
            strike: false,
            fc_mode: ColorMode::Mode3Bit,
            bc_mode: ColorMode::Mode3Bit,
            highlighted: false,
        }
    }
}

struct telem {
    digit: [u8; 8],
    digit_count: u8,
    value: u64,
    next: *mut telem,
}

fn parse_ansi(input: &str) -> String {
    let fcstyle: [&str; 10] = [
        "dimgray",
        "red",
        "green",
        "yellow",
        "blue",
        "purple",
        "aqua",
        "white",
        "black",
        "white",
    ];

    let bcstyle: [&str; 10] = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "purple",
        "aqua",
        "white",
        "black",
        "white",
    ];

    let mut state = Default::default();
    let mut last_state: AnsiParsingState;

    let mut index: usize = 0;
    let chars: Vec<char> = input.chars().collect();
    while index < input.len() {
        let mut c = chars[index];
        match c {
            '\x1B' => {
                last_state = state;
                index += 1;
                c = chars[index];
                if c == '[' {
                    let mut buf: [char; 1024] = ['\0'; 1024];
                    buf[0] = '[';
                    let mut counter = 1;
                    while (c < 'A') || ((c > 'Z') && (c < 'a')) || (c > 'z') {
                        index += 1;
                        c = chars[index];
                        buf[counter] = c;
                        if c == '>' {
                            break;
                        }
                        counter += 1;
                        if counter > 1022 {
                            break;
                        }
                    }
                    buf[counter - 1] = '\0';
                    let pointer_telem: *mut telem = std::ptr::null_mut();
                    match c {
                        'm' => {

                        }
                    }
                }
            },
            '\x0D' => {

            },
            '\x08' => {

            },
            _ => {
            },
        }
    }

    String::new()
}

fn main() {
}
