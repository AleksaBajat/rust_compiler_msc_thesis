# Rustc - reprezentacije izvornog koda

Ovaj projekat predstavlja moj master rad na temu analize i implementacije ključnih faza Rust kompajlera. Fokus je na razumevanju unutrašnje strukture Rust jezika, njegovih međureprezentacija izvornog koda kao što su HIR, THIR i MIR, kao i na procesima koji omogućavaju bezbedno i efikasno kompajliranje.

Cilj rada je da prikaže kako Rust upravlja kompleksnim procesima analize koda, optimizacije i generisanja mašinskog koda kroz svoj moderni kompajlerski dizajn, uz osvrt na korišćenje LLVM infrastrukture. Projekat može poslužiti kao resurs za studente, istraživače i inženjere zainteresovane za kompajlere i Rust programski jezik.

# Sadržaj

1. [Spisak skraćenica](#spisak-skracenica)
2. [Uvod](#uvod)
3. [Pozadina](#pozadina)
   - [Rust jezik](#rust-jezik)
   - [Cargo](#cargo)
   - [LLVM projekat](#llvm-projekat)
4. [Rust kompajler](#rust-kompajler)
5. [Međureprezentacije izvornog koda](#medjureprezentacije-izvornog-koda)
   - [Tok tokena](#tok-tokena)
     - [rustc_lexer](#rustc_lexer)
     - [rustc_parse::lexer](#rustc_parselexer)
   - [Apstraktno Sintaksno Stablo](#apstraktno-sintaksno-stablo)
   - [Međureprezentacija visokog nivoa (MVN) - High-Level Intermediate Representation (HIR)](#medjureprezentacija-visokog-nivoa-mvn---high-level-intermediate-representation-hir)
     - [Konstrukcija](#konstrukcija)
     - [Sistem upita](#sistem-upita)
     - [Inkrementalno kompajliranje](#inkrementalno-kompajliranje)
   - [Tipizirana međureprezentacija visokog nivoa (TMVN) - Typed HIR (THIR)](#tipizirana-medjureprezentacija-visokog-nivoa-tmvn---typed-hir-thir)
     - [Bezbednost](#bezbednost)
     - [Provera šablona](#provera-sablona)
   - [Međureprezentacija srednjeg nivoa (MSN) - Mid-Level Intermediate Representation (MIR)](#medjureprezentacija-srednjeg-nivoa-msn---mid-level-intermediate-representation-mir)
     - [Graf kontrole toka](#graf-kontrole-toka)
     - [Optimizacija koda u MSN-u](#optimizacija-koda-u-msn-u)
     - [Neleksički životni vekovi](#neleksicki-zivotni-vekovi)
6. [Zaključak](#zakljucak)
7. [Literatura](#literatura)
8. [Dodatak 1](#dodatak-1)
9. [Dodatak 2](#dodatak-2)
10. [Dodatak 3](#dodatak-3)
11. [Dodatak 4](#dodatak-4)
12. [Podaci o kandidatu](#podaci-o-kandidatu)
