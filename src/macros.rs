#[macro_export]
macro_rules! print_bool {
    ($message:expr) => {
        if $message {
            $message.color(AnsiColors::Green)
        } else {
            $message.color(AnsiColors::Red)
        }
    };
    ($message:expr, $t:expr, $f:expr) => {
        if $message {
            $t.color(AnsiColors::Green)
        } else {
            $f.color(AnsiColors::Red)
        }
    };
}
