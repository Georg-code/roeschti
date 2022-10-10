use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fähl" => "Err",
        "Oke" => "Ok",
        "Zeichecheti" => "String",
        "Dictionär" => "HashMap",
        "Standart" => "Default",
        "Fähler" => "Error",
        "Vilicht" => "Option",
        "Espaar" => "Some",
        "Nüt" => "None",
        "Resultat" => "Result",
        "Selber" => "Self",
        "usdrucke" => "println",
        "ahalte" => "break",
        "asynchron" => "async",
        "warte" => "await",
        "schlaufe" => "loop",
        "bewege" => "move",
        "cagette" => "crate",
        "code_unerreichbar" => "unreachable_code",
        "als" => "as",
        "constante" => "const",
        "eigeschaft" => "trait",
        "unsicher" => "unsafe",
        "in" => "in",
        "vo" => "from",
        "dynamisch" => "dyn",
        "entpacke" => "unwrap",
        "standart" => "default",
        "as_referenz" => "as_ref",
        "es" => "io",
        "extern" => "extern",
        "falsch" => "false",
        "funktion" => "fn",
        "super" => "super",
        "infüege" => "insert",
        "hole" => "get",
        "erlaube" => "allow",
        "scheisse" | "panik" | "huere_schafseckel" => "panic",
        "modul" => "mod",
        "veränderbar" => "mut",
        "nüs" => "new",
        "wo" => "where",
        "für" => "for",
        "hole_oder_ifüege_mit" => "get_or_insert_with",
        "haupt" => "main",
        "öffentlich" => "pub",
        "was" => None?,
        "zruggäh" => "return",
        "implementiere" => "impl",
        "referenz" => "ref",
        "überistimme" => "match",
        "falls" => "if",
        "susch" => "else",
        "selber" => "self",
        "lahn" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "erwarte" => "expect",
        "solang" => "while",
        "bruch" => "use",
        "in" => "into",
        "wahr" => "true",
        "ufzellig" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn röschti(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
