/*
 * Esta é uma 'calçadeira' (bootstrap) para permitir escrever o macro processual
 * em ferrugem.
 */
use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Mau" => "Err",
        "Bom" => "Ok",
        "Cadeia" => "String",
        "Dicionário" => "HashMap",
        "Predefinido" => "Default",
        "Erro" => "Error",
        "PodeSer" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Próprio" => "Self",
        "imprime" => "println",
        "achantra" => "break",
        "assíncrona" => "async",
        "espera" => "await",
        "ciclo" => "loop",
        "desloca" => "move",
        "contentor" => "crate",
        "código_inacessível" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "característica" => "trait",
        "confia" => "unsafe",
        "em" => "in",
        "de" => "from",
        "dinâmico" => "dyn",
        "desembrulha" => "unwrap",
        "predefinido" => "default",
        "como_ref" => "as_ref",
        "externo" => "extern",
        "falso" => "false",
        "função" => "fn",
        "genial" => "super",
        "insere" => "insert",
        "obtem" => "get",
        "legal" => "allow",
        "caralho" | "cacete" | "ups" => "panic",
        "módulo" => "mod",
        "mutável" => "mut",
        "novo" => "new",
        "onde" => "where",
        "por" => "for",
        "para" => "for",
        "obtem_ou_insere_com" => "get_or_insert_with",
        "principal" => "main",
        "público" => "pub",
        "que" => None?,
        "devolve" => "return",
        "implementa" => "impl",
        "equipara" => "match",
        "se" => "if",
        "ou_então" => "else",
        "próprio" => "self",
        "seja" => "let",
        "estático" => "static",
        "estrutura" => "struct",
        "suppose" => "expect",
        "enquanto" => "while",
        "utiliza" => "use",
        "torna_em" => "into",
        "verdade" => "true",
        "enumeração" => "enum",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "FluxoDeSímbolos" => "TokenStream",
        "ÁrvoreDeSímbolos" => "TokenTree",
        "torna_em_cadeia" => "to_string",
        "como_cadeia" => "as_str",
        "extensão" => "span",
        "Vetor" => "Vec",
        "fluxo" => "stream",
        "empurra" => "push",
        "estende" => "extend",
        "delimitador" => "delimiter",
        "Pontuação" => "Punct",
        "macro_processual" => "proc_macro",
        "coleções" => "collections",
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
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
