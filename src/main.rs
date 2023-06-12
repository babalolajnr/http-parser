use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    error::{context, VerboseError},
    Err as NomErr, IResult,
};

struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
enum Scheme {
    HTTP,
    HTTPS,
}

type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, PartialEq, Eq)]
enum Host {
    HOST(String),
    IP([u8; 4]),
}

type QueryParam<'a> = (&'a str, Option<&'a str>);

type QueryParams<'a> = Vec<QueryParam<'a>>;

impl From<&str> for Scheme {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => panic!("Invalid scheme"),
        }
    }
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn scheme(input: &str) -> Res<&str, Scheme> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
    .map(|(next_input, scheme)| (next_input, scheme.into()))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use nom::error::{ErrorKind, VerboseErrorKind};

    use super::*;

    #[test]
    fn test_scheme() {
        assert_eq!(scheme("HTTP://"), Ok(("", Scheme::HTTP)));
        assert_eq!(scheme("HTTPS://"), Ok(("", Scheme::HTTPS)));
        assert_eq!(scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(
            scheme("bla://yay"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("bla://yay", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("bla://yay", VerboseErrorKind::Context("scheme")),
                ]
            }))
        );
    }
}
