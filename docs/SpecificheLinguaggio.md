# Specifiche del Linguaggio NodeStract (NS)

Questo documento definisce le regole sintattiche, le parole chiave consentite e i criteri di validazione per il linguaggio NodeStract (NS).

---

## 1. Regole sulle Importazioni (Header Import)

Le importazioni sono obbligatorie e devono seguire regole strutturali rigide:
1. **Posizione in testa**: Tutti i costrutti di importazione (`import` / `importa` / ecc.) devono trovarsi all'inizio del file, in linee consecutive.
2. **Nessun import tardivo**: Non è consentito inserire un'istruzione di importazione dopo una qualsiasi istruzione di tipo diverso (es. dichiarazioni di variabili, espressioni, commenti, ecc.).
3. **Importazione di una Lingua Obbligatoria**: Il programma deve importare esplicitamente almeno una lingua dal modulo `translate` (es. `import english from translate` o `importa italiano da translate`). In assenza di un import di lingua, il programma non può essere compilato.
4. **Bootstrapping degli Import**: Le parole chiave `import` e `from` (e le loro traduzioni) sono sempre attive e disponibili in testa al file per consentire la lettura delle direttive di importazione. Successivamente vengono rimosse dal vocabolario e possono essere usate liberamente come identificatori (es. nomi di variabili).

---

## 2. Attivazione Dinamica delle Parole Chiave (Keywords)

NodeStract supporta fino a 7 lingue contemporaneamente (`english`, `italian`, `spanish`, `french`, `german`, `portuguese`, `romanian`).
* Le parole chiave di una lingua (come `crea`, `fissa`, `se`, `mentre`, `funzione` in italiano) diventano attive **solo se** la corrispondente lingua è stata importata.
* Se una lingua non viene importata, i suoi termini non sono considerati parole riservate e possono essere usati come normali nomi di variabili o funzioni (es. se non importi l'italiano, puoi dichiarare `let se = 10` senza errori). Se invece importi l'italiano, l'uso di `se` come variabile genererà un errore di sintassi.

---

## 3. Protezione delle Funzioni Built-in (Librerie)

Le funzioni predefinite (matematiche, I/O e file system) appartengono a moduli specifici:
- **`nio` (Input/Output)**: `print`, `input`
- **`nfs` (File System)**: `read`, `write`, `delete`
- **`nmath` (Matematica)**: `sin`, `cos`, `sqrt`, `random`, `round`, `min`, `max`, `abs`, `log`, `pow`

Per poter utilizzare una di queste funzioni, è obbligatorio importare il relativo modulo (es. `import * from nmath` oppure `import sin from nmath`). L'uso di una funzione built-in senza aver importato il modulo corrispondente causerà un errore di compilazione.

---

## 4. Assenza di Classi e Tipi Statici

NodeStract è un linguaggio procedurale, funzionale e a tipizzazione dinamica ispirato a JavaScript e Python:
- Non esistono le classi (`class`) né l'operatore `new`.
- Non si dichiarano i tipi di dato per le variabili; si utilizzano semplicemente `let` o `const` (nelle loro relative traduzioni).
- Non sono ammessi caratteri underscore `_` nei costrutti chiave e nelle traduzioni ufficiali del linguaggio.

---

## 5. Risoluzione Dinamica delle Chiamate tramite Stringhe (Dynamic Call String Fallback)

NodeStract supporta un comportamento particolare di chiamata a funzione dinamica (metaprogrammazione):
- Se un'espressione di chiamata a funzione (es. `target(argomenti)`) valuta come target una stringa, l'interprete risolverà a runtime il nome della funzione cercando nella tabella dei simboli globali una funzione o una built-in che abbia esattamente lo stesso nome specificato all'interno del valore della stringa.

### Esempio d'uso:
```ns
importa italiano da translate
importa stampa da nio

crea nome_funzione = "stampa"
// Questa chiamata invocherà dinamicamente la funzione 'stampa'
nome_funzione("Ciao da chiamata dinamica!")
```
Questo meccanismo è supportato per consentire un comportamento dinamico assimilabile alle callback e alle chiamate per nome tipiche dei linguaggi di scripting più flessibili.

