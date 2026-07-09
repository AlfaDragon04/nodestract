# Analisi delle Incompletezze, Bug e Indizi di Utilizzo di AI in NodeStract

Questo documento raccoglie l'analisi approfondita del progetto **NodeStract**, evidenziando i bug, le incompletezze strutturali e le anomalie ("hallmarks" o indizi tipici di codice generato da AI) che potrebbero far insospettire un docente di informatica durante la valutazione.

Per ciascun punto viene fornita la spiegazione in termini semplici, il comportamento attuale, cosa succede se non si interviene, la soluzione proposta e il motivo di tale scelta.

---

## 1. Dipendenze Inutilizzate nel File di Configurazione (`Cargo.toml`)

### Spiegazione Semplice
Il file `Cargo.toml` descrive al gestore di pacchetti Rust (Cargo) quali librerie esterne servono al nostro programma. È l'equivalente della lista della spesa. Avere ingredienti nella lista che poi non vengono mai usati nella ricetta indica distrazione o, peggio, copia-incolla acritico da template generati da AI.

### Come funziona attualmente
Nel file [Cargo.toml](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/Cargo.toml) sono dichiarate le librerie `aes-gcm` (crittografia simmetrica) e `hex` (conversione in esadecimale). Tuttavia, nel codice sorgente Rust non c'è traccia di importazioni o utilizzi di queste librerie.

### Cosa succede se non si fa
Un docente noterà immediatamente che il progetto compila dipendenze crittografiche pesanti inutilmente. È una firma classica delle AI, che spesso inseriscono librerie "standard" di sicurezza/utility per allucinazione o piani di sviluppo poi abbandonati. Riduce il voto sulla qualità del codice.

### Come intendiamo farlo
Rimuovere del tutto le righe relative a `aes-gcm` e `hex` dal file `Cargo.toml` e ripulire il file `Cargo.lock` rieseguendo `cargo build`.

### Perché così e non in un altro modo
È la soluzione più pulita e professionale: se una dipendenza non serve, va eliminata. Non ha senso tenerla o scrivere codice fittizio per giustificarla.

---

## 2. Supporto Fittizio all'Asincronia (`async` / `await` Morti)

### Spiegazione Semplice
In programmazione, l'asincronia permette di fare più cose contemporaneamente (es. fare una richiesta web senza bloccare il resto del programma). Le parole chiave `async` e `await` servono rispettivamente a definire una funzione asincrona e ad aspettare che finisca. 
In NodeStract, queste parole chiave vengono lette dal compilatore, ma all'atto pratico non fanno assolutamente nulla: il programma le ignora ed esegue tutto in modo sincrono (bloccante).

### Come funziona attualmente
1. Nel parser ([statement.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/parser/statement.rs#L38-L45)), se viene incontrata la parola `async` prima di `function`, viene impostato un flag `is_async: true` nell'AST (l'albero che rappresenta il codice).
2. Nel parser delle espressioni ([expression.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/parser/expression.rs#L158-L161)), la parola `await` viene accettata e avvolge l'espressione successiva.
3. Nell'interprete ([expressions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/expressions.rs#L102)), `Expression::Await(value)` valuta semplicemente il valore interno in modo sincrono e bloccante. Il flag `is_async` non viene letto da nessuna parte nell'interprete.

### Cosa succede se non si fa
Un professore di informatica che corregge il progetto noterà subito che avete inserito nel manuale d'uso e nel parser dei concetti avanzati come `async/await` senza implementarli davvero. Questo dimostra che il codice del parser è stato generato da un'AI che ha inserito funzionalità "alla cieca" senza una vera architettura concorrente sotto.

### Come intendiamo farlo
Abbiamo due opzioni:
1. **Rimuovere del tutto** `async` e `await` da dizionari JSON, parser e AST per rendere il linguaggio onestamente sincrono.
2. **Implementare un'asincronia reale** (ad esempio usando i thread di Rust o simulando una coda di micro-task), ma questo complicherebbe enormemente l'interprete per un progetto triennale.
*Raccomandazione:* Rimuovere le parole chiave per mostrare una sintassi pulita e coerente con la tesi che si tratta di un linguaggio didattico procedurale semplice.

### Perché così e non in un altro modo
La rimozione (Opzione 1) è la scelta migliore. È coerente con quanto scritto nella stessa documentazione critica: *"L'assenza di asincronia reale [...] è ottimale per mantenere lineare l'esecuzione e comprensibile il codice dell'interprete"*. Avere codice morto che simula una feature inesistente è un grave errore metodologico.

---

## 3. Clonazione Inefficiente dello Scope nelle Chiamate di Funzione (Scoping Lessicale "Naive")

### Spiegazione Semplice
Ogni volta che si chiama una funzione, questa ha diritto al suo "spazio" privato dove salvare le proprie variabili (lo scope locale). Deve però poter leggere anche le variabili globali (scoping lessicale). 
Per fare questo, l'interprete attuale esegue una copia fisica (clonazione) dell'intera tabella delle variabili globali a ogni singola chiamata di funzione, e poi copia indietro le modifiche alla fine.

### Come funziona attualmente
Nel file [functions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/functions.rs#L270-L284):
```rust
let global_scope = self.scopes[0].clone();
let function_scopes = vec![global_scope, new_scope];
let old_scopes = std::mem::replace(&mut self.scopes, function_scopes);
// ... esecuzione corpo ...
let mut finished_scopes = std::mem::replace(&mut self.scopes, old_scopes);
self.scopes[0] = finished_scopes.remove(0);
```

### Cosa succede se non si fa
Questo approccio funziona per piccoli programmi, ma dal punto di vista dell'ingegneria del software è pessimo. Clonare interi dizionari in memoria a ogni chiamata di funzione distrugge le prestazioni ed è un tipico trucco "veloce" usato dalle AI per risolvere problemi di scoping in Rust senza implementare strutture dati adeguate (come i puntatori o gli indici). Un docente di compilatori boccerebbe questa scelta all'istante.

### Come intendiamo farlo
Modificare la gestione degli scope. Invece di distruggere e clonare lo stack di scope (`self.scopes`), possiamo semplicemente aggiungere lo scope locale in cima a `self.scopes` quando entriamo nella funzione, ed effettuare le ricerche di variabili partendo dall'ultimo scope (locale) fino al primo (globale), limitando l'accesso solo agli scope consentiti (es. saltando gli scope locali delle funzioni chiamanti intermedie).
O ancora più semplicemente, passare un riferimento o usare un sistema di risoluzione degli scope che non richieda la clonazione della memoria globale.

### Perché così e non in un altro modo
Evita allocazioni di memoria inutili e rispetta i sacri testi di progettazione dei linguaggi di programmazione, dimostrando padronenza dell'uso dei vettori come stack di scope in Rust.

---

## 4. Gestione Silente degli Errori Critici (Type Error e Const Assignment)

### Spiegazione Semplice
Se in un linguaggio di programmazione provi a moltiplicare un numero per una stringa (es. `5 * "ciao"`) o a sovrascrivere una costante (es. `const X = 10; X = 20;`), il linguaggio deve bloccarsi immediatamente segnalando l'errore. 
NodeStract attualmente stampa l'errore a schermo ma fa finta di nulla, continuando a eseguire le righe successive come se non fosse successo niente, usando `null` come risultato.

### Come funziona attualmente
1. Nelle operazioni binarie ([ops.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/ops.rs)), in caso di tipi incompatibili, viene stampato un messaggio a console con `println!` e restituito `Value::Null`.
2. Nell'assegnazione di costanti ([interpreter.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/interpreter.rs#L91-L102)), se la variabile è costante viene stampato un errore e si esce dalla funzione `set_var`, ma l'interprete non interrompe il programma.

### Cosa succede se non si fa
L'esecuzione procede con valori inconsistenti (`null`), causando comportamenti imprevedibili e bug a catena nello script dell'utente. Un interprete accademico serio deve sollevare eccezioni fatali che interrompono l'esecuzione se non gestite da un blocco `try/catch`.

### Come intendiamo farlo
Quando viene rilevato un errore di tipo critico o una scrittura su costante:
1. Impostare il flag `self.exception = Some(Value::String(messaggio_errore))` nell'interprete.
2. Assicurarsi che le funzioni che valutano espressioni o eseguono statement controllino questo flag e interrompano immediatamente l'esecuzione (proprio come avviene già per altre eccezioni).

### Perché così e non in un altro modo
Sfrutta il meccanismo di gestione delle eccezioni (`try/catch` e `self.exception`) già parzialmente implementato nell'interprete, garantendo coerenza architetturale e robustezza senza dover stravolgere la pipeline.

---

## 5. Mancanza di Controllo sul Numero di Argomenti nelle Funzioni (Arity Check)

### Spiegazione Semplice
Se definisci una funzione che accetta 3 argomenti `funzione somma(a, b, c)` e la chiami passandone solo 2 `somma(10, 20)`, l'interprete dovrebbe avvisarti che manca un argomento prima ancora di avviare la funzione. NodeStract invece assegna silenziosamente il valore `null` a `c`, provocando in seguito errori oscuri e difficili da diagnosticare.

### Come funziona attualmente
In [functions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/functions.rs#L261-L269), la chiamata di funzione associa i parametri posizionali. Se il numero di argomenti passati è inferiore ai parametri richiesti, assegna `Value::Null` senza generare alcun avviso o eccezione.

### Cosa succede se non si fa
Questo comportamento (simile a quello di JavaScript ma meno tollerato in ambito accademico) nasconde bug logici negli script. Inoltre, l'assenza di un controllo formale sull'arità delle chiamate è un'altra incompletezza tipica dei prototipi AI scritti di fretta.

### Come intendiamo farlo
Prima di eseguire il corpo di una funzione definita dall'utente, confrontare la lunghezza del vettore `args` (argomenti passati) con la lunghezza del vettore `params` (parametri richiesti). Se non corrispondono, sollevare un'eccezione di runtime indicando la discrepanza (es. *"La funzione X richiede N argomenti, ma ne sono stati passati M"*).

### Perché così e non in un altro modo
Rende il linguaggio robusto, sicuro e coerente con la tipizzazione dinamica ordinata. Impedisce fallimenti a cascata all'interno delle funzioni.

---

## 6. Euristica Fragile e Spaziatura Incoerente in `stampa` (`print`)

### Spiegazione Semplice
La funzione built-in `stampa` (o `print`) serve a mostrare messaggi sul terminale. Se passiamo più cose da stampare separate da virgola, vogliamo che l'output sia formattato in modo prevedibile. L'interprete attuale ha una serie di regole complicate (euristiche) per decidere se mettere o meno uno spazio tra le parole basandosi sulla punteggiatura, il che rende l'output instabile e bizzarro.

### Come funziona attualmente
In [functions.rs](file:///w:/University/3o%20anno/ICDD/Esame/Progetto/NodeStract/src/interpreter/functions.rs#L15-L24), il ciclo accumula gli argomenti valutati convertendoli in stringhe. Attualmente non mette spazi, oppure usa euristiche fragili (in realtà, il codice attuale unisce semplicemente le stringhe senza spazi: `output.push_str(&val.to_string())`). Questo costringe l'utente a inserire spazi manuali ovunque o produce scritte tutte attaccate.

### Cosa succede se non si fa
Scrivere programmi didattici in NodeStract diventa frustrante perché l'output non è formattato in modo naturale. Inoltre, soluzioni di formattazione approssimative saltano subito all'occhio durante una demo live davanti al professore.

### Come intendiamo farlo
Sostituire la logica di accumulo in `print` in modo che separi gli argomenti multipli con uno spazio singolo (come fa Python o JavaScript con le virgole) e vada a capo alla fine della chiamata, oppure fornire sia `stampa` (che non va a capo) sia `stampaLinea` (che va a capo) con una formattazione semplice e standardizzata.

### Perché così e non in un altro modo
È lo standard industriale per i linguaggi didattici e di scripting. Semplifica la sintassi per gli studenti che useranno il linguaggio e pulisce il codice Rust dell'interprete eliminando logiche astruse.
