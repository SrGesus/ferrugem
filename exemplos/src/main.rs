ferrugem::ferrugem! {
    externo contentor ferrugem;

    utiliza std::coleções::Dicionário como Dic;

    característica ChaveValor {
        função insere(&próprio, chave: Cadeia, valor: Cadeia);
        função obtem(&próprio, chave: Cadeia) -> Resultado<PodeSer<&Cadeia>, Cadeia>;
    }

    estático mutável DICIONÁRIO: PodeSer<Dic<Cadeia, Cadeia>> = Nenhum;

    estrutura Concreta;

    implementa ChaveValor para Concreta {
        função insere(&próprio, chave: Cadeia, valor: Cadeia) {
            seja dic = perigo {
                DICIONÁRIO.obtem_ou_insere_com(Predefinido::predefinido)
            };
            dic.insere(chave, valor);
        }
        função obtem(&próprio, chave: Cadeia) -> Resultado<PodeSer<&Cadeia>, Cadeia> {
            se seja Algum(dic) = perigo { DICIONÁRIO.como_ref() } {
                Bom(dic.obtem(&chave))
            } ou_então {
                Mau("mau maria que o gato já mia.".torna_em())
            }
        }
    }

    público(contentor) função pode_ser(i: u32) -> PodeSer<Resultado<u32, Cadeia>> {
        se i % 2 == 1 {
            se i == 42 {
                Algum(Mau(Cadeia::de("merda")))
            } ou_então {
                Algum(Bom(33))
            }
        } ou_então {
            Nenhum
        }
    }

    assíncrona função exemplo() {
    }

    assíncrona função exemplo2() {
        exemplo().espera;
    }

    função principal() {
        seja mutável x = 31;

        equipara x {
            42 => {
                imprime!("sardinha de são joão")
            }
            _ => imprime!("está feito, ó patrão")
        }

        por i em 0..10 {
            seja val = ciclo {
                pára i;
            };

            enquanto que x < val {
                x += 1;
            }

            x = se seja Algum(resultado) = pode_ser(i) {
                resultado.desembrulha()
            } ou_então {
                12
            };
        }

        //secundária();
    }

    #[legal(código_inacessível)]
    função secundária() {
        caralho!("oh não"); // for the true Portuguese experience
        cacete!("nossa senhora"); // for friends speaking pt-br
        ups!("cai o carmo e a trindade"); // in SFW contexts
    }
}
