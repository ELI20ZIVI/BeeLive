# BeeLive

BeeLive - Progetto del corso di Ingegneria del Software

Questa e' l'unica repository del progetto "BeeLive", e di conseguenza tutti i componenti che abbiamo sviluppato, relativi a questo progetto, sono all'interno di questa repository.

### Repository structure

- I deliverable creati per il corso sono nelle cartelle `Deliverable-1...4`. Sono progetti di "TexStudio" per cui e' possibile che sia necessario ri-renderizzarli per poterli vedere in formato PDF.

- in `BeeLive/Products` vi sono i progetti che abbiamo creato per realizzare il *management_server*, il *public_server*, la *desktop_app* e la *mobile_app*. Vi è il progetto `BeeLive/Products/beelive_frontend_commons` contenente alcuni elementi condivisi tra la mobile e la desktop application. 

- in `BeeLive/Products/Docker` vi sono le configurazioni per docker compose e i diversi container, nonche' le configurazioni per mongobd e casdoor (i quali non necessitavano di un progetto a se' stante vero e proprio).

La repository contiene anche altri file, rimasugli di deliverable precedenti che in quanto tali possono essere tranquillamente ignorati.

### Cloning

Per effettuare il deployment di questo progetto, e' inizialmente necessario clonare la repository includendo anche i submodules, in quanto abbiamo dovuto modificare [acw/simple_asn1](https://github.com/acw/simple_asn1)  in quanto outdated.

```bash
git clone --recurse-submodule https://github.com/ELI20ZIVI/BeeLive
```

### Backend deployment

Il sistema e' suddiviso in diversi container Docker contenenti i diversi moduli. Abbiamo creato un singolo `docker-compose.yml` contenente le configurazioni dei container (e.g. la configurazione di rete attraverso la quale i container comunicano).

Il file e' collocato in `BeeLive/Products/Docker/docker-compose.yml` per cui per effettuare il deployment del sistema basta effettuare un "docker compose up" all'interno della suddetta cartella:

```bash
cd BeeLive/Products/Docker
docker compose up -d
```

### Frontend app running

**NOTA:** il frontend e' attualmente configurato per connettersi al nostro server personale, per cui per connettersi ad un altro server (e.g. localhost) e' necessario modificare, nel codice, l'ip a cui la mobile_app o la desktop_app si connettono (rispettivamente, in `.../desktop_app/lib/main.dart:18` e `.../mobile_app/lib/main.dart:14`)

Per eseguire l'applicazione desktop, e' necessario effettuare alcune operazioni preliminarie prima di runnarla con flutter:

```bash
cd BeeLive/Products/desktop_app
flutter pub get # download dependencies
dart run build_runner build # run builders
flutter run
```

Per eseguile l'applicazione mobile...

### Testing

Per eseguire i test del `management_server`: 

```bash
cd BeeLive/Products/management_server
cargo test -- --test-threads=1
```

E' importante specificare `--test-threads=1` in quanto l'esecuzione contemporanea dei test causa problemi di concorrenza, da cui incoerenza e irriproducibilita'.

Analogamente, per eseguire i test del `public_server`:

```bash
cd BeeLive/Products/public_server
cargo test -- --test-threads=1
```
