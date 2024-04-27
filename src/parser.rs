use core::panic;

use crate::lexer::{Lexer, TokenType};

#[derive(Default, Debug, Clone)]
pub struct PackageFile {
    pakages: Vec<Package>,
}

#[derive(Default, Debug, Clone)]
pub struct Package {
    name: String,
    inputs: Vec<Pin>,
    outputs: Vec<Pin>,
    body: Vec<Operator>,
}

#[derive(Default, Debug, Clone)]
pub enum PinKind {
    #[default]
    Logic,
    Analog,
    Clock,
}
impl From<&String> for PinKind {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "logic" => Self::Logic,
            "analog" => Self::Analog,
            "clock" => Self::Clock,
            _ => panic!("wrong pin type!"),
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct Pin {
    ident: String,
    kind: PinKind,
}

#[derive(Default, Debug, Clone)]
pub struct Operator {

}

pub fn parse_package_file(lexer: &mut Lexer) -> PackageFile {
    lexer.next_token();
    let mut pkgfile = PackageFile::default();
    loop {
        match lexer.token.ttype {
            TokenType::Empty => {
                break;
            },
            TokenType::Package => {
                pkgfile.pakages.push(parse_package(lexer));
            }
            _ => {
                panic!("Every package must start with the <<package>> identifier");
            },
        }
    }
    pkgfile
}

fn parse_package(lexer: &mut Lexer) -> Package {
    lexer.match_token(TokenType::Package);
    let pkg_name = lexer.token.literal.clone();
    lexer.match_token(TokenType::Ident);
    lexer.match_token(TokenType::OParen);
    let mut inputs = Vec::<Pin>::new();
    let mut outputs = Vec::<Pin>::new();
    loop {
        match lexer.token.ttype {
            TokenType::CParen => break,
            TokenType::In | TokenType::Out => {
                let in_out = lexer.token.ttype.clone();
                lexer.next_token();
                let ident = lexer.token.literal.clone();
                lexer.match_token(TokenType::Ident);
                lexer.match_token(TokenType::Colon);
                let pin_kind = PinKind::from(&lexer.token.literal);
                if in_out == TokenType::In {
                    inputs.push(Pin {
                        ident,
                        kind: pin_kind,
                    });
                } else {
                    outputs.push(Pin {
                        ident,
                        kind: pin_kind,
                    });
                }
            },
            TokenType::Comma => {
                lexer.match_token(TokenType::Comma);
                continue;
            },
            _ => {
                panic!("expected pin name found {:?}", lexer.token.ttype);
            }
        }
    }
    Package {
        name: pkg_name,
        inputs,
        outputs,
        body: Vec::new(),
    }
}



