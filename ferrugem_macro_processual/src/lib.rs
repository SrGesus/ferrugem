calcadeira_ferrugem::ferrugem! {
    utiliza macro_processual::{Grupo, Identificador, FluxoDeSímbolos, ÁrvoreDeSímbolos};

    função substitui_identificador(identificador: Identificador) -> PodeSer<ÁrvoreDeSímbolos> {
        seja cadeia_identificador = identificador.torna_em_cadeia();

        seja cadeia_nova = equipara cadeia_identificador.como_cadeia() {
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
            "que" => Nenhum?,
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
            _ => &cadeia_identificador,
        };

        seja identificador_novo = Identificador::novo(cadeia_nova, identificador.extensão());
        Algum(ÁrvoreDeSímbolos::Identificador(identificador_novo))
    }

    função substitui_árvore(símbolo: ÁrvoreDeSímbolos, saída: &mutável Vetor<ÁrvoreDeSímbolos>) {
        equipara símbolo {
            ÁrvoreDeSímbolos::Grupo(grupo) => {
                seja mutável elementos_grupo = Vetor::novo();
                substitui_fluxo(grupo.fluxo(), &mutável elementos_grupo);
                seja mutável fluxo_novo = FluxoDeSímbolos::novo();
                fluxo_novo.estende(elementos_grupo);
                saída.empurra(ÁrvoreDeSímbolos::Grupo(Grupo::novo(grupo.delimitador(), fluxo_novo)));
            }
            ÁrvoreDeSímbolos::Identificador(identificador) => {
                se seja Algum(identificador) = substitui_identificador(identificador) {
                    saída.empurra(identificador);
                }
            }
            ÁrvoreDeSímbolos::Pontuação(..) | ÁrvoreDeSímbolos::Literal(..) => {
                saída.empurra(símbolo);
            }
        }
    }

    função substitui_fluxo(fluxo_de_símbolos: FluxoDeSímbolos, saída: &mutável Vetor<ÁrvoreDeSímbolos>) {
        por símbolo em fluxo_de_símbolos {
            substitui_árvore(símbolo, saída)
        }
    }

    #[macro_processual]
    público função ferrugem(elemento: FluxoDeSímbolos) -> FluxoDeSímbolos {
        seja mutável devolvido = Vetor::novo();
        substitui_fluxo(elemento, &mutável devolvido);
        seja mutável saída = FluxoDeSímbolos::novo();
        saída.estende(devolvido);
        saída
    }
}
