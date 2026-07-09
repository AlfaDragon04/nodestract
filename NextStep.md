# NextStep: Stato dell'Analisi e Modifiche Apportate

Questo documento tiene traccia delle problematiche individuate sul compilatore/interprete di **NodeStract (NS)** per eliminare tracce di generazione AI ("AI smell") e allineare il codice agli standard accademici.

---

## Stato delle problematiche e miglioramenti

1. [x] **File e directory orfani/inutilizzati (`src/execute/`)** — **COMPLETATO**
2. [x] **Disallineamento della versione del compilatore (`welcome.rs`)** — **COMPLETATO**
3. [x] **Commenti bilingue (Inglese/Italiano) e docstring incoerenti** — **COMPLETATO**
4. [x] **Gestione degli errori a runtime frammentata (Standard Output)** — **COMPLETATO**
5. [x] **Modalità interattiva (REPL)** — **RIFIUTATO (SCELTA DELLO SVILUPPATORE)**

---

## Nuove Criticità Rilevate e Piano d'Azione (Audit Accademico)

### 1. [x] Il Bug del Blocco `Finally` nei Costrutti `Try-Catch-Finally`
* **Implementazione**: Risolto in [statements.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/statements.rs). Lo stato dei flag di flusso (`last_return`, `loop_break`, `loop_continue`, `exception`) viene temporaneamente preservato e azzerato durante l'esecuzione del blocco `finally`, ripristinandolo in coda se nessun nuovo evento sovrascrive lo stato. Aggiunto test di validazione `7_finally.ns`.

### 2. [x] Uso Silente di Variabili Inesistenti / Non Dichiarate (AI Smell)
* **Implementazione**: Risolto in [expressions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/expressions.rs) e [interpreter.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/interpreter.rs). Aggiunto il metodo `has_var()` ed impostato un errore critico in `eval_expression()` per sollevare un'eccezione a runtime se si tenta di leggere variabili non definite. Aggiunto test di validazione `6_undefined_var.ns`.

### 3. [x] Limitazione Lessicale sul Range `..` Senza Spazi
* **Implementazione**: Risolto in [lexer.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/lexer/lexer.rs). Aggiunto il controllo con lookahead in `read_number()` per fermare la lettura dei decimali all'incontro dell'operatore di range `..`. Aggiornato il test integrato `2_range.ns` per usare `1..5` senza spazi.

### 4. [x] Presenza di Log a Console Residui in `fs.rs` e `net.rs`
* **Implementazione**: Risolto in [fs.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/fs.rs), [net.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/net.rs) e [functions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/functions.rs). Rimosse tutte le stampe dirette a terminale `println!` per gli errori; le funzioni restituiscono ora `Result<Value, String>` e i messaggi d'errore vengono trasformati in eccezioni intercettabili a runtime dell'interprete.

### 5. [x] Mancanza di Riflessione / Interrogazione dei Tipi a Runtime
* **Decisione**: Esclusa su indicazione dell'utente per concentrarsi sulle anomalie critiche e rispettare la scadenza.

### 6. [x] Stringhe Letterali Non Chiuse Ignorate dal Lexer
* **Implementazione**: Risolto in [lexer.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/lexer/lexer.rs) e [parser.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/parser/parser.rs). Il lexer rileva la mancata chiusura delle virgolette a fine file restituendo `Token::Unknown('"')`, e la funzione di controllo pre-parsing solleva un errore sintattico indicando linea e colonna.

### 7. [x] Commenti Multilinea a Fine Riga negli Import
* **Implementazione**: Risolto in [check.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/import/check.rs). Aggiunto un ciclo di pulizia dei commenti `/* ... */` inline sulla riga corrente prima di effettuare lo split e l'analisi sintattica dell'importazione.
