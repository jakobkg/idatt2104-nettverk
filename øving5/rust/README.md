# API
Kompilator-tjeneren implementerer en enkel API, skrevet ned her som referanse i tilfelle jeg glemmer den mellom jeg jobber på kompilatoren og på web-tjeneren

## Forespørsler
Kompilator-tjeneren forventer å bli tilsendt programmer over HTTP POST med et JSON-objekt i body. Dette JSON-objektet forventes å ha to felter:

|navn|type|forklaring|
|----|----|----------|
|lang|string|Programmeringsspråket som skal kompileres, representert ved en forhåndsdefinert kode (se under for støttede språk og `lang`-verdier)|
|program|string|Programkoden som skal kompileres|

### Støttede språk
* Rust, med `lang: "rust"`
* C, med `lang: "c"`
* C++, med `lang: "cpp"`

## Respons
Tjeneren implementerer kun to HTTP-statuser som brukes i responser, `200 OK` og `500 Server error`.

### 200 OK
Gitt at kompilering og kjøring av programkoden har blitt gjennomført uten feil vil tjeneren svare med `200 OK` og et JSON-objekt med to felter:

|navn|type|forklaring|
|----|----|----------|
|compiler_output|string|Kompilatorens output fra `stdout` og `stderr`, og exit status|
|program_output|string|`stdout` fra det kompilerte programmet etter kjøring, og exit status|

### 500 Server Error
**OBS**: dette representerer sannsynligvis ikke en feil i den innsendte programkoden, men heller en feil under behandlingen av forespørselen (filsystem-feil e.l.)