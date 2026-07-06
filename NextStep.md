# Prossimi Passi (NextStep) - NodeStract

Questo documento raccoglie tutte le anomalie, i bug, le omissioni rispetto alle specifiche e le debolezze architetturali riscontrate durante l'analisi statica e dinamica del codice di NodeStract. Trattandosi di un progetto a scopo didattico, non è necessario che sia pronto per la produzione di massa, ma è fondamentale che le funzionalità presenti siano corrette e coerenti in vista della consegna.

---

## 1. Difetti Gravi e Incoerenze di Esecuzione (Runtime / Semantica)

### 1.1 Scoping Dinamico dei Parametri delle Funzioni (Invece che Lessicale)
* **Descrizione del problema**: Nel modulo `src/interpreter/functions.rs` (riga 259), quando viene eseguita una funzione definita dall'utente, l'interprete crea un nuovo scope con i parametri valutati e lo inserisce in testa alla lista `self.scopes`. Tuttavia, la ricerca delle variabili in `get_var` e `set_var` scorre tutti gli scope attivi a ritroso. Questo implementa uno **scoping dinamico**: una funzione chiamata ha accesso (e può modificare) le variabili locali del chiamante se queste hanno lo stesso nome delle variabili libere.
* **Perché è un problema**: Viola il comportamento atteso di un linguaggio procedurale/funzionale moderno (ispirato a JS/Python come da `spec.md`), che dovrebbe garantire lo scoping lessicale.
* **Soluzione consigliata**: Associare a ciascuna funzione lo scope in cui è stata definita (closure) o isolare l'ambiente di esecuzione locale delle funzioni, limitando l'accesso solo allo scope globale e a quello locale della funzione stessa, escludendo lo stack delle chiamate intermedie.


---

## 2. Omissioni rispetto alle Specifiche (`spec.md`)

### 2.1 Funzioni Built-in Dichiarate ma Non Implementate
Le seguenti funzioni sono definite in `spec.md` (e mappate nei dizionari di traduzione come `it.json`), ma sono **completamente assenti** nel motore di esecuzione (`src/interpreter/functions.rs`):
* **Libreria `nfs` (File System)**:
  * `open` (apri): Dichiarata nelle specifiche ma non gestita dall'interprete.
  * `delete` (elimina): Dichiarata nelle specifiche ma non gestita dall'interprete.
* **Libreria `nnet` (Network)**:
  * `connect` (connetti): Dichiarata ma non implementata.
  * `receive` (ricevi): Dichiarata ma non implementata.

* **Soluzione consigliata**: Implementare queste funzioni in `src/interpreter/fs.rs` e `src/interpreter/net.rs`, registrandole poi nel blocco di match di `handle_function_call`.

---

## 3. Disallineamento dei Numeri di Riga negli Errori

### ~~3.1 Rimozione Fisica delle Righe di Import in `check.rs`~~ (Risolto)
* ~~**Descrizione del problema**: Durante la fase iniziale di validazione, `src/import/check.rs` filtra il codice sorgente eliminando completamente le righe che contengono istruzioni di importazione (`importa ... da ...`), restituendo un sorgente "pulito" al lexer.~~
* ~~**Perché è un problema**: Rimuovendo le righe fisicamente anziché sostituirle con righe vuote (es. `\n`), tutti gli errori di sintassi o di runtime segnalati successivamente dal parser o dall'interprete avranno numeri di riga **sfalsati** all'indietro di un numero pari agli import eseguiti.~~
* ~~**Soluzione consigliata**: Modificare `check.rs` in modo che, quando rileva e convalida una riga di import, inserisca una riga vuota (o un commento segnaposto) in `stripped_lines` anziché saltare del tutto il push. In questo modo la corrispondenza 1-a-1 dei numeri di riga viene preservata.~~

---

## 4. Limitazioni e Bug Minori

### 4.1 Separazione degli Spazi nella Funzione `print` (`stampa`)
* **Descrizione del problema**: La logica di formattazione in `functions.rs` cerca di decidere in modo intelligente se aggiungere o meno uno spazio tra gli argomenti stampati. Questo causa a volte la comparsa di doppi spazi indesiderati o comportamenti imprevisti con la punteggiatura passata come argomento separato.
* **Soluzione consigliata**: Semplificare la logica di `print` adottando un comportamento standard (es. separare sempre gli elementi con un singolo spazio fisso, in stile Python, o consentire all'utente di definire un separatore).

### 4.2 Chiamata di Funzioni su Espressioni Dinamiche
* **Descrizione del problema**: Nel parser (`src/parser/expression.rs`, riga 251), la grammatica dei blocchi di chiamata assume che il target della chiamata sia strettamente un identificatore statico (un nome memorizzato come stringa). Questo rende impossibile chiamare funzioni in modo dinamico, ad esempio caricando una funzione da un array (`voci[0]()`) o restituendo una funzione da un'altra chiamata (`genera_funzione()()`).
* **Soluzione consigliata**: Modificare il nodo AST `FunctionCall` per supportare un `Box<Expression>` come target anziché una stringa fissa.
