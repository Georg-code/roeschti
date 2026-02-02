use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Fäh" => "Err",
        "Oke" => "Ok",
        "Zeichecheti" | "Wörtli" => "String",
        "Dictionär" => "HashMap",
        "Standard" => "Default",
        "Fähler" => "Error",
        "Vilicht" => "Option",
        "Öppis" => "Some",
        "Nüt" => "None",
        "Resultat" | "Summa_Summarum" => "Result",
        "Selber" => "Self",
        "usdrucke" | "säg" => "println",
        "ahalte" | "item" | "momentli" => "break",
        "asynchron" | "chum_i_hüt_nöd_chumi_morn" => "async",
        "druf-warte" => "await",
        "schlaufe" => "loop",
        "bewege" => "move",
        "chiste" => "crate",
        "unerreichbare_code" => "unreachable_code",
        "als" => "as",
        "konstant" | "festi_grössi" => "const",
        "eigeschaft" => "trait",
        "unsicher" | "gföhrli" => "unsafe",
        "in" => "in",
        "vo" => "from",
        "dynamisch" | "juflig" => "dyn",
        "uspacke" => "unwrap",
        "standard" | "nullacht_füfzä" => "default",
        "als_referenz" => "as_ref",
        "es" => "io",
        "extern" | "fremde_fötzel" => "extern",
        "falsch" => "false",
        "funktion" => "fn",
        "super" | "supi" => "super",
        "infüege" | "ifüege" => "insert",
        "hole" => "get",
        "erlaube" => "allow",
        "scheisse" | "panik" | "huere_schafseckel" | "verreis" => "panic",
        "modul" => "mod",
        "veränderbar" => "mut",
        "neus" => "new",
        "wo" => "where",
        "für" => "for",
        "hole_oder_ifüege_mit" => "get_or_insert_with",
        "haupt" => "main",
        "öffentlich" => "pub",
        "was" => None?,
        "zruggäh" | "git" => "return",
        "implementiere" => "impl",
        "referenz" => "ref",
        "überistimme" | "pässlet" => "match",
        "falls" | "goht" => "if",
        "susch" => "else",
        "selber" => "self",
        "lahn" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "erwarte" | "gang_mol_devo_us" => "expect",
        "solang" => "while",
        "bruch" => "use",
        "in" | "drii" => "into",
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
