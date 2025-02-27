use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bitcoin); // synthesized by LALRPOP

#[test]
fn bitcoin() {
    assert!(bitcoin::ExprParser::new().parse("22").is_ok());
    assert!(bitcoin::ExprParser::new().parse("(22)").is_ok());
    assert!(bitcoin::ExprParser::new().parse("((((22))))").is_ok());
    assert!(bitcoin::ExprParser::new().parse("((22)").is_err());
}

fn main() {
    println!(
        "{:?}",
        bitcoin::ExprParser::new().parse("2147483648").unwrap()
    );
    println!("{:?}", bitcoin::ExprParser::new().parse("1").unwrap());
    println!(
        "{:?}",
        bitcoin::ExprParser::new()
            .parse("-1+2-(4+7)+(7-4)")
            .unwrap()
    );

    println!(
        "{:?}",
        bitcoin::ExprParser::new()
            .parse(
                r#"let _private_z_123 = " 안여누아ㅓㅁ 231#@$@!  /₩12 ㅓ2'''ㅣ;ㅣ x y z123AS S""#
            )
            .unwrap()
    );

    println!("{:?}", bitcoin::ExprParser::new().parse("if").unwrap())
}
