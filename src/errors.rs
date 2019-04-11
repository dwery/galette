
#[derive(Clone, Copy, Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub line: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorCode {
    Code(i32),
    ARSP_SUFFIX,
    ARSP_AS_PIN_NAME,
    BAD_ARSP,
    BAD_CHAR,
    BAD_EOF,
    BAD_NC,
    BAD_PIN,
    BAD_PIN_COUNT,
    BAD_POWER,
    BAD_GAL_TYPE,
    BAD_SUFFIX,
    BAD_TOKEN,
    INVALID_CONTROL,
    INVERTED_ARSP,
    INVERTED_CONTROL,
    INVERTED_POWER,
    NOT_AN_INPUT,
    NOT_AN_OUTPUT,
    NO_CLK,
    NO_EQUALS,
    NO_PIN_NAME,
    // TODO: I don't really believe in these.
    PREMATURE_APRST,
    PREMATURE_ARST,
    PREMATURE_CLK,
    PREMATURE_ENABLE,
    REPEATED_APRST,
    REPEATED_ARST,
    REPEATED_AR_SP,
    REPEATED_CLK,
    REPEATED_OUTPUT,
    REPEATED_TRISTATE,
    TOO_MANY_PRODUCTS,
    TRISTATE_REG,
    UNKNOWN_PIN,
    UNMATCHED_TRISTATE,
}

const ERROR_CODES: [&str; 49] = [
    "error in source file found",
    "Line  1: type of GAL expected",
    "unexpected end of file",
    "pinname expected after '/'",
    "max. length of pinname is 8 characters",
    "illegal character in pin declaration",
    "illegal VCC/GND assignment",
    "pin declaration: expected VCC at VCC pin",
    "pin declaration: expected GND at GND pin",
    "pinname defined twice",
    "illegal use of '/'",
    "unknown pinname",
    "NC (Not Connected) is not allowed in logic equations",
    "unknown suffix found",
    "'=' expected",
    "this pin can't be used as output",
    "same pin is defined multible as output",
    "before using .E, the output must be defined",
    "GAL22V10: AR and SP is not allowed as pinname",
    ".E, .CLK, .ARST and .APRST is not allowed to be negated",
    "mode 2: pins 12, 19 can't be used as input",
    "mode 2: pins 15, 22 can't be used as input",
    "tristate control is defined twice",
    "GAL16V8/20V8: tri. control for reg. output is not allowed",
    "tristate control without previous '.T'",
    "use GND, VCC instead of /VCC, /GND",
    "pin not allowed in equations",
    "mode 3: pins 1,13 are reserved for 'Clock' and '/OE'",
    "use of VCC and GND is not allowed in equations",
    "only one product term allowed (no OR)",
    "too many product terms",
    "use of AR and SP is not allowed in equations",
    "negation of AR and SP is not allowed",
    "no equations found",
    ".CLK is not allowed when this type of GAL is used",
    ".ARST is not allowed when this type of GAL is used",
    ".APRST is not allowed when this type of GAL is used",
    "GAL20RA10: pin 1 can't be used in equations",
    "GAL20RA10: pin 13 can't be used in equations",
    "AR, SP: no suffix allowed",
    "AR or SP is defined twice",
    "missing clock definition (.CLK) of registered output",
    "before using .CLK, the output must be defined",
    "before using .ARST, the output must be defined",
    "before using .APRST the output must be defined",
    "several .CLK definitions for the same output found",
    "several .ARST definitions for the same output found",
    "several .APRST definitions for the same output found",
    "use of .CLK, .ARST, .APRST only allowed for registered outputs"
];

fn error_string(err_code: ErrorCode) -> &'static str {
    match err_code {
        ErrorCode::Code(i) => ERROR_CODES[i as usize],
        ErrorCode::ARSP_AS_PIN_NAME => "GAL22V10: AR and SP is not allowed as pinname",
        ErrorCode::ARSP_SUFFIX => "AR, SP: no suffix allowed",
        ErrorCode::BAD_ARSP => "use of AR and SP is not allowed in equations",
        ErrorCode::BAD_NC => "NC (Not Connected) is not allowed in logic equations",
        ErrorCode::BAD_CHAR => "bad character in input",
        ErrorCode::BAD_EOF => "unexpected end of file",
        ErrorCode::BAD_GAL_TYPE => "Line  1: type of GAL expected",
        ErrorCode::BAD_PIN => "illegal character in pin declaration",
        ErrorCode::BAD_PIN_COUNT => "wrong number of pins",
        ErrorCode::BAD_POWER => "use of VCC and GND is not allowed in equations",
        ErrorCode::BAD_SUFFIX => "unknown suffix found",
        ErrorCode::BAD_TOKEN => "unexpected token",
        ErrorCode::INVERTED_ARSP => "negation of AR and SP is not allowed",
        ErrorCode::INVALID_CONTROL => "use of .CLK, .ARST, .APRST only allowed for registered outputs",
        ErrorCode::INVERTED_CONTROL => ".E, .CLK, .ARST and .APRST is not allowed to be negated",
        ErrorCode::INVERTED_POWER => "use GND, VCC instead of /VCC, /GND",
        ErrorCode::NOT_AN_INPUT => "pin not allowed in equations",
        ErrorCode::NOT_AN_OUTPUT => "this pin can't be used as output",
        ErrorCode::NO_CLK => "missing clock definition (.CLK) of registered output",
        ErrorCode::NO_PIN_NAME => "pinname expected after '/'",
        ErrorCode::NO_EQUALS => "'=' expected",
        ErrorCode::PREMATURE_APRST => "before using .APRST the output must be defined",
        ErrorCode::PREMATURE_ARST => "before using .ARST, the output must be defined",
        ErrorCode::PREMATURE_CLK => "before using .CLK, the output must be defined",
        ErrorCode::PREMATURE_ENABLE => "before using .E, the output must be defined",
        ErrorCode::REPEATED_APRST => "several .APRST definitions for the same output found",
        ErrorCode::REPEATED_ARST => "several .ARST definitions for the same output found",
        ErrorCode::REPEATED_AR_SP => "AR or SP is defined twice",
        ErrorCode::REPEATED_CLK => "several .CLK definitions for the same output found",
        ErrorCode::REPEATED_OUTPUT => "same pin is defined multible as output",
        ErrorCode::REPEATED_TRISTATE => "tristate control is defined twice",
        ErrorCode::TOO_MANY_PRODUCTS => "too many product terms",
        ErrorCode::TRISTATE_REG => "GAL16V8/20V8: tri. control for reg. output is not allowed",
        ErrorCode::UNKNOWN_PIN => "unknown pinname",
        ErrorCode::UNMATCHED_TRISTATE => "tristate control without previous '.T'",
    }
}

pub fn print_error(err: Error) {
    println!("Error in line {}: {}", err.line, error_string(err.code));
}
