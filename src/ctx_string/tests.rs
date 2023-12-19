use super::*;

#[test]
fn parsing() {
    assert_eq!(
        CtxString::new("My ass\n smells bad").unwrap(),
        CtxString(vec![Token::Literal(String::from("My ass\n smells bad"))])
    );
    assert_eq!(
        CtxString::new("My ${ass}\n smells like shit").unwrap(),
        CtxString(vec![
            Token::Literal(String::from("My ")),
            Token::Var(String::from("ass")),
            Token::Literal(String::from("\n smells like shit")),
        ])
    );
    assert_eq!(
        CtxString::new("It's my \\${ass}").unwrap(),
        CtxString(vec![Token::Literal(String::from("It's my ${ass}")),])
    );
    assert_eq!(
        CtxString::new("${Eat my {ass\\}}").unwrap(),
        CtxString(vec![Token::Var(String::from("Eat my {ass}")),])
    );
    assert_eq!(
        CtxString::new("${var}left").unwrap(),
        CtxString(vec![
            Token::Var(String::from("var")),
            Token::Literal(String::from("left")),
        ])
    );
    assert_eq!(
        CtxString::new("right${var}").unwrap(),
        CtxString(vec![
            Token::Literal(String::from("right")),
            Token::Var(String::from("var")),
        ])
    );
    assert_eq!(
        CtxString::new("Never gonna {give} ${you} %up").unwrap(),
        CtxString(vec![
            Token::Literal(String::from("Never gonna {give} ")),
            Token::Var(String::from("you")),
            Token::Literal(String::from(" ")),
            Token::DateTime(String::from("%u")),
            Token::Literal(String::from("p")),
        ])
    );
    assert_eq!(
        CtxString::new("Never gonna let \\%you %_down").unwrap(),
        CtxString(vec![
            Token::Literal(String::from("Never gonna let %you ")),
            Token::DateTime(String::from("%_d")),
            Token::Literal(String::from("own")),
        ])
    );
    assert_eq!(
        CtxString::new("${{ inside vars are okay}").unwrap(),
        CtxString(vec![Token::Var(String::from("{ inside vars are okay"))])
    );
    assert!(CtxString::new("Unescaped $ will cause fuckage").is_err());
    assert!(CtxString::new("This ${var should end somewhere").is_err());
    assert!(CtxString::new("This is also not ok %").is_err());
}
