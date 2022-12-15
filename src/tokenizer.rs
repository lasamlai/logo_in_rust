use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "STRING" = pattern "\"[a-zA-Z_]+";
        "DEFAULT" | "LABEL" = pattern r":[a-zA-Z_]+";
        "DEFAULT" | "PROC" = pattern r"[a-zA-Z_]+";
        "DEFAULT" | "NUM" = pattern r"-?[0-9]+(\.[0-9])?";
        "DEFAULT" | "SPEC" = pattern r"[\[\]+\-*/<>()]";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}
