use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Хуёво" => "Err",
        "Нормалды" => "Ok",
        "Строчка" => "String",
        "Коммуналка" => "HashMap",
        "Пиздец" => "Error",
        "Вариантик" => "Option",
        "ЧутьЧуть" => "Some",
        "Ничо" => "None",
        "Победа" => "Result",
        "Пацанчик" => "Self",
        "пиздеть" => "println",
        "перекур" => "break",
        "распиздяйственно" => "async",
        "падажжи" => "await",
        "карусель" => "loop",
        "беги" => "move",
        "коробка" => "crate",
        "зарубежный_код" => "unreachable_code",
        "будто" => "as",
        "путин" => "const",
        "ништячок" => "trait",
        "страшно" => "unsafe",
        "из" => "from",
        "весёленький" => "dyn",
        "новыйгод" => "unwrap",
        "как_ссылка" => "as_ref",
        "ихний" => "extern",
        "пиздабольство" => "false",
        "пацан" => "fn",
        "заебись" => "super",
        "вставить" => "insert",
        "отобрать" => "get",
        "можно" => "allow",
        "бежать" | "плакать" | "кричать" => "panic",
        "модуль" => "mod",
        "черепашка_ниндзя" => "mut",
        "новый" => "new",
        "где" => "where",
        "для" => "for",
        "получи_либо_вставь_с" => "get_or_insert_with",
        "база" => "main",
        "телевидение" => "pub",
        "хватит" => "return",
        "короче_так" => "impl",
        "отсылка" => "ref",
        "вариантики" => "match",
        "а_че_если" => "if",
        "не_удалось" => "else",
        "главный" => "self",
        "разрешаю" => "let",
        "постоянно" => "static",
        "структура" => "struct",
        "до_поры" => "while",
        "юзай" => "use",
        "внутрь" => "into",
        "внатуре" => "true",
        "считалочка" => "enum",

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
pub fn rzhavchina(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
