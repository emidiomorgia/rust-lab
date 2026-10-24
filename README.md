<div align="center">

# 🦀 Rust Lab

**Laboratorio pratico di Rust per sviluppatori C# e Java: dal linguaggio ai servizi enterprise.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/edition-2024-informational)](https://doc.rust-lang.org/edition-guide/)
[![Lezioni](https://img.shields.io/badge/lezioni-0%2F25%20completate-lightgrey)](#-percorso-e-stato-di-avanzamento)
[![Stato](https://img.shields.io/badge/stato-in%20corso-yellow)](#-percorso-e-stato-di-avanzamento)

[Abstract](#-abstract) ·
[Percorso](#-percorso-e-stato-di-avanzamento) ·
[Struttura](#-struttura-del-repository) ·
[Avvio rapido](#-avvio-rapido) ·
[Contribuire](#-contribuire) ·
[Licenza](#-licenza)

</div>

---

## 📖 Abstract

**Rust Lab** è un monorepo di corsi di laboratorio dedicati a Rust, organizzato come un
[Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html): ogni lezione è un
crate indipendente, compilabile e testabile da solo, ma tutte condividono la stessa toolchain e la
stessa cartella di build.

Il percorso è pensato per chi arriva da **C#** e **Java** e vuole imparare Rust per prova diretta,
non per sola lettura. Ogni lezione propone un problema concreto da risolvere in autonomia, a
difficoltà crescente, e rende esplicito:

1. l'**argomento** trattato;
2. il **livello di complessità**;
3. gli **strumenti di Rust** coinvolti.

Si parte dalla sintassi (tipi, ownership, borrowing, lifetime, trait, generics, closure, gestione
degli errori), si prosegue con i temi avanzati (smart pointer, thread, `async`/`await` con Tokio,
streaming) e si arriva a un contesto enterprise: API REST con Axum, serializzazione con Serde,
PostgreSQL, MongoDB, Kafka, resilienza e backpressure, osservabilità, packaging con Docker.

> **Filo conduttore.** Dove C# e Java offrono comodità a runtime (garbage collector, reflection,
> container di dependency injection), Rust sposta lo stesso lavoro a compile time. Il laboratorio
> serve a farlo diventare un'abitudine, non una nozione.

---

## 🎯 Obiettivi

- Padroneggiare il modello di **ownership e borrowing** e leggere gli errori del compilatore.
- Scrivere codice idiomatico: `Result`/`Option`, iteratori, trait, composizione al posto
  dell'ereditarietà.
- Costruire **concorrenza sicura** (thread, canali, `Send`/`Sync`) e **asincronia** con Tokio.
- Realizzare un servizio **enterprise** completo: REST, persistenza, messaggistica, resilienza,
  osservabilità.

## 👥 A chi è rivolto

A sviluppatori con esperienza in C#, Java o linguaggi analoghi. Non è richiesta alcuna conoscenza
pregressa di Rust. Ogni argomento viene affiancato al suo equivalente (o alla sua assenza) in
C# e Java.

## ✅ Prerequisiti

| Strumento | Versione | Note |
|---|---|---|
| [Rust](https://rustup.rs/) (via `rustup`) | stable, edition 2024 | include `cargo`, `rustfmt`, `clippy` |
| Git | recente | |
| Docker | recente | solo per le lezioni 18-22 (database, Kafka) |

```bash
rustup default stable
rustc --version
cargo --version
```

---

## 🗺️ Percorso e stato di avanzamento

**Legenda stato:** 🟢 completata · 🟡 in esecuzione · ⚪ da iniziare

**Complessità:** `Bassa` · `Media` · `Alta`

**Avanzamento complessivo:** `░░░░░░░░░░░░░░░░░░░░` 0 / 25 completate · 1 in esecuzione

### Parte I · Sintassi

| # | Lezione | Argomenti | Complessità | Stato | Sorgenti |
|---|---|---|---|---|---|
| 01 | Statistiche su sequenze numeriche | Cargo, tipi primitivi e cast, array/`Vec`/slice, `if`/`loop` come espressioni, test | Bassa | 🟡 In esecuzione | [l01-stats](l01-stats/) |
| 02 | Ownership e borrowing | Move vs `Copy`, `&T`, `&mut T`, reborrow | Media | ⚪ Da iniziare | — |
| 03 | Lifetime e stringhe | Annotazioni di lifetime, `String` vs `&str` | Media | ⚪ Da iniziare | — |
| 04 | Struct, enum e `match` | Tipi somma, `match` esaustivo, guardie, binding | Media | ⚪ Da iniziare | — |
| 05 | Collezioni e iteratori | `Vec`, `HashMap`, `HashSet`, adapter lazy | Media | ⚪ Da iniziare | — |
| 06 | Closure | `Fn`, `FnMut`, `FnOnce`, `move` | Media | ⚪ Da iniziare | — |
| 07 | Gestione degli errori | `Result`, `Option`, operatore `?`, errori custom | Media | ⚪ Da iniziare | — |
| 08 | Generics e trait bound | Monomorfizzazione, bound, `where` | Media | ⚪ Da iniziare | — |

### Parte II · Argomenti avanzati

| # | Lezione | Argomenti | Complessità | Stato | Sorgenti |
|---|---|---|---|---|---|
| 09 | Trait e polimorfismo | Default method, static vs dynamic dispatch, `dyn` | Alta | ⚪ Da iniziare | — |
| 10 | Smart pointer | `Box`, `Rc`, `Arc`, `RefCell` | Alta | ⚪ Da iniziare | — |
| 11 | Multithreading | `thread::spawn`, `Mutex`, `Arc`, `mpsc` | Alta | ⚪ Da iniziare | — |
| 12 | Concorrenza sicura | `Send`, `Sync`, data race a compile time | Alta | ⚪ Da iniziare | — |
| 13 | Async/await | `Future`, runtime Tokio, `join!`, `select!` | Alta | ⚪ Da iniziare | — |
| 14 | I/O e streaming | `Read`/`Write`, `AsyncRead`/`AsyncWrite`, `Stream` | Alta | ⚪ Da iniziare | — |

### Parte III · Enterprise

| # | Lezione | Argomenti | Complessità | Stato | Sorgenti |
|---|---|---|---|---|---|
| 15 | Configurazione e ambienti | `config`, `dotenvy`, `serde`, struct tipizzate | Bassa | ⚪ Da iniziare | — |
| 16 | API REST con Axum | Routing, estrattori, `State<Arc<AppState>>` | Media | ⚪ Da iniziare | — |
| 17 | Serializzazione e mapper | Serde, `From`/`TryFrom`, DTO vs dominio | Media | ⚪ Da iniziare | — |
| 18 | PostgreSQL | `sqlx`, Diesel, pool, transazioni | Media | ⚪ Da iniziare | — |
| 19 | MongoDB | Driver ufficiale, BSON con Serde, cursori come `Stream` | Media | ⚪ Da iniziare | — |
| 20 | Kafka | `rdkafka`, producer/consumer, commit | Alta | ⚪ Da iniziare | — |
| 21 | Resilienza | Retry con backoff, DLQ, backpressure, `tower` | Alta | ⚪ Da iniziare | — |
| 22 | Streaming end-to-end | Composizione di `Stream`, `buffer_unordered` | Alta | ⚪ Da iniziare | — |
| 23 | Logging e osservabilità | `tracing`, span, `#[instrument]`, OpenTelemetry | Media | ⚪ Da iniziare | — |
| 24 | Build e packaging | Profilo release, LTO, Docker multistage | Media | ⚪ Da iniziare | — |
| 25 | Progetto finale | Servizio integrato end-to-end | Alta | ⚪ Da iniziare | — |

---

## 🗂️ Struttura del repository

```text
rust-lab/
├── Cargo.toml          # workspace: members = ["lezioni/*"]
├── README.md
├── LICENSE
├── .gitignore          # target/
└── lezioni/
    ├── l01-stats/      # lezione 01
    ├── l02-.../        # una cartella (crate) per lezione
    └── ...
```

Ogni cartella in `lezioni/` è un crate autonomo con il proprio `Cargo.toml`, il codice e i test.

---

## 🚀 Avvio rapido

```bash
# 1. clona il repository
git clone https://github.com/<utente>/rust-lab.git
cd rust-lab

# 2. esegui una lezione
cargo run -p l01-stats

# 3. lancia i test della lezione
cargo test -p l01-stats

# 4. compila in release
cargo build --release -p l01-stats
```

L'opzione `-p <crate>` seleziona un singolo crate del workspace. Senza, i comandi agiscono su tutti.

### Qualità del codice

```bash
cargo fmt --all            # formattazione
cargo clippy --all-targets # lint
cargo test --workspace     # tutti i test
```

---

## 🧭 Come si lavora su una lezione

1. Leggi la spiegazione e il testo del problema.
2. Crea il crate con `cargo new lezioni/lNN-nome`.
3. Risolvi il problema in autonomia, **senza assistenza AI**, un passo alla volta.
4. Scrivi i test insieme al codice, nello stesso file, in un modulo `#[cfg(test)]`.
5. Verifica con `cargo test`, `cargo clippy` e `cargo build --release`.
6. Un commit per passo; a lezione conclusa, aggiorna lo stato nella tabella sopra.

---

## 🤝 Contribuire

Il repository è nato come percorso personale di studio, ma suggerimenti e correzioni sono benvenuti.

1. Apri una **issue** descrivendo il problema o la proposta.
2. Fai un fork e crea un branch: `git checkout -b fix/descrizione-breve`.
3. Assicurati che `cargo fmt`, `cargo clippy` e `cargo test` passino senza errori.
4. Apri una **pull request** collegata alla issue.

## 🗒️ Convenzioni

- Un crate per lezione, nominato `lNN-nome-breve`.
- Firme idiomatiche: `&[T]` e `&str` nei parametri di sola lettura, `Result` per gli errori
  recuperabili, `panic!` solo per violazioni di invarianti.
- Nessun `unwrap()` nel codice che non sia di test.
- Commenti e documentazione in italiano; identificatori in inglese o italiano, ma coerenti
  all'interno di una lezione.

---

## 📄 Licenza

Distribuito con licenza **MIT**. Vedi il file [LICENSE](LICENSE) per il testo completo.

```text
MIT License

Copyright (c) 2026 Emidio Morgia
```

## 📚 Riferimenti

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Tokio](https://tokio.rs/) · [Axum](https://docs.rs/axum) · [Serde](https://serde.rs/) · [sqlx](https://docs.rs/sqlx)
