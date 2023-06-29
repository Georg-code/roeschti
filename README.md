# rust uf Schwizerdütsch
![](https://github.com/Georg-code/rost_ch/raw/hauptast/logo.png)

Aren't you _müed_ from writing Rust programs in English? Do you like saying
"huere schafsäckel" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Schwizerdütsch touch to your
programs?

**roscht** (Schwizerdütsch/dütsch for _Rust_) is here to save your day, as it allows you to
write Rust programs in Swissgerman, using Swissgerman keywords, Swissgerman function names,
Swissgerman idioms.

This has been designed to be used as the official programming language for every `bünzli`

If you want to pay someone for this, pay the french guy. I only adapted his work. He does some nice shit. Worth a look. If you still want to support me just star this repo that will make me happy.
[His Repo](https://github.com/bnjbvr/rouille) \
[His liberapay](https://liberapay.com/bnjbvr/).



### chlises bispil

```rust
roeschti::roeschti! {
    bruch std::collections::Dictionär als Dico;

    eigeschaft schlüsselwärt {
        funktion schrib(&selber, schlüssel: Zeichecheti, wärt: Zeichecheti);
        funktion lis(&selber, schlüssel: Zeichecheti) -> Vilicht<&Zeichecheti>;
    }

    statisch veränderbar DICTIONÄR: <Dico<Zeichecheti, Zeichecheti>> = Nüt;

    struktur Konkret;

    implementiere schlüsselwärt für Konkret {
        funktion schrib(&selber, schlüssel: Zeichecheti, wärt: Zeichecheti) {
            lahn dico = unsicher {
                DICTIONÄR.hole_oder_ifüege_mit(Standart::standart)
            };
            dico.infüege(schlüssel, wärt);
        }
        funktion lis(&selber, schlüssel: Zeichecheti) -> Resultat<Vilicht<&Zeichecheti>, Zeichecheti> {
            falls lahn Espaar(dico) = unsicher { DICTIONÄR.as_referenz() } {
                Oke(dico.lis(&schlüssel))
            } susch {
                Fähl("Usem dico usehole".in())
            }
        }
    }
}
```

## Mitmache

First of all, _merci_ for considering participating to this joke, the
Swiss government and Roger Federer will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hauptast` (Swissgerman for
`mainbranch`).

Please don't introduce swear words.

## but why would you do zat

1) why are you looking at this? Same reason
2) Reason 1
## Other languages
- French [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)


## d'Lizenz

[Öffentlichi Lizenz, wenn da druf drucks chaschs au nomal uf französisch lese](http://sam.zoy.org/lprab/),
