# User Stories

> Nota: _MA_ sono features delle mobile apps, _WS_ sono features dei web server.

### Utente

- Da utente, voglio visualizzare la lista degli eventi che ci sono in città.
    - Instanziazione _MongoDB_.
    - (_MA_) Creazione screen.
    - (_WS_) Creazione API endpoint.
    - (_WS_) Creazione metodo del DAO.
    - (_MA_) Creazione metodo del Client (component).
    - (_WS_) Creazione _Event Processor_ per estrarre gli "header".
        - Nota all'implementatore: ora come ora può essere tranquillamente un pass-through al DAO. È previsto nel caso in futuro si voglia estendere il tutto con cache.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente, voglio visualizzare su una cartina le zone colpite dai diversi eventi, in modo da capire visivamente se in una zona che mi interessa vi è un evento attivo.
    - (_MA_) Estensione screen.
    - (_MA_) Creazione tests.

- Da utente, voglio avere la possibilità di accedere ad una visualizzazione e descrizione più dettagliata di un evento in modo da conoscere meglio le specifiche di tale evento
    - (_MA_) Creazione screen.
    - (_WS_) Creazione API endpoint.
    - (_MA_) Creazione metodo del Client.
    - (_WS_) Creazione metodo del DAO.
    - (_WS_) Estensione _Event Processor_ per i dettagli.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente, voglio essere in grado di impostare delle zone di interesse, in modo che io venga notificato di qualsiasi evento che insorga in una di queste zone.
    - (_MA_) Creazione screen Settings.
    - (_MA_) Creazione screen Point Picker.
    - (_WS_) Creazione API endpoint (put).
    - (_WS_) Creazione API endpoint (get).
    - (_WS_) Creazione metodi del DAO.
    - (_MA_) Creazione metodi del DAO.
    - (_MA_) Creazione metodi del Client.
    - (_WS_) Creazione _Preferences Manager_ (pass-through).
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente, voglio essere in grado di selezionare un percorso a partire da dei waypoint, per impostare dei percorsi di interesse.
    - (_MA_) Creazione screen Route Picker.
    - (_WS_) Creazione API endpoint.
    - (_MA_) Creazione metodo del Client.
    - (_WS_) Creazione _Route Calculator_.
    - Instanziazione _OpenRouteService_.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.
    
- Da utente, voglio essere in grado di impostare le categorie di eventi alle quali io sono interessato, in modo che la lista di eventi visibile dall'applicazione contenga solamente gli eventi che mi interessano, e in modo che io non venga notificato di alcun evento che io non ritenghi appartenere ad una categoria interessante.
    - (_MA_) Estensione screen Settings.
    - (_MA_) Creazione screen Category Picker.
    - (_MA_) Creazione metodi del Client (put/get).
    - (_WS_) Creazione API endpoint (put).
    - (_MA_) Creazione metodo del DAO (put/get).
    - (_WS_) Creazione metodo del DAO (put).
    - (_WS_) Modifica _Preferences Manager_.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente, voglio essere in grado di visualizzare solo eventi di categorie di interesse.
    - (_MA_) Modifica metodo del Client.
    - (_WS_) Modifica API endpoint.
        - Nota: deve essere possibile passare le categorie per parametro.
    - (_WS_) Modifica metodo del DAO.
    - (_MA_) Modifica metodo del DAO.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente, voglio essere in grado di poter registrare un account in modo da poter salvare le mie impostazioni e le aree di interesse che io ho impostato
    - (_MA_) Creazione screen Login.
    - Instanziazione _Casdoor_.
    - (_MA_) Sincronizzazione preferenze locali.
        - Nota: serve qualche flag per evitare di sovrascrivere tutto.
    - (_MA_) Client: gestione auth token.
    - (_WS_) Modifica API endpoints.
        - Nota: devono accettare auth token e gestire le autorizzazioni.
    - (_WS_) Creazione _Authentication SDK_.
    - (_WS_) Creazione policies AC di "default".
        - Nota: ruoli utenti: _user_, _banned_, _authorized_, _publisher_ (avevamo usato un altro termine, vero?), _delegate_...
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Da utente registrato, voglio essere in grado di poter accedere al mio account in modo da poter riottenere le aree di interesse e le impostazioni che ho precedentemente salvato
    - (_WS_) Creazione API endpoint (categorie/zone).
    - (_WS_) Creazione metodo del DAO (categorie/zone).
    - (_MA_) Modifica metodo del Client.
    - (_WS_) Modifica _Preferences Manager_.
    - (_WS_) Creazione tests.
    - (_MA_) Creazione tests.

- Le notifiche le metterei come feature a parte... ad alto effort perchè è uno sbatti.


### Utente autorizzato

- da utente autorizzato, devo essere in grado di aggiungere degli eventi in modo da comunicare la loro presenza agli utenti.
    - (_MA_) Creazione screen.
    - (_MA_) Creazione metodo del Client.
    - (_WS_) Creazione API endpoint.
    - (_WS_) Creazione metodo del DAO.
    - (_WS_) Creazione _Event Processor_ per operazioni booleane sulle zone.
    - (_WS_) Creazione _Event manager_.
    - (_MA_) Creazione tests.
    - (_WS_) Creazione tests.

> Da qui in poi sono da sistemare.

- da utente autorizzato, devo essere in grado di modificare degli eventi in modo da evitare errori comunicativi.

- da utente autorizzato, devo essere in grado di eliminare degli eventi.

- da utente autorizzato, devo essere in grado di visualizzare gli eventi di mia competenza in modo da modificarli o eliminarli.

- da utente autorizzato, devo essere in grado di aggiungere le sottocategorie nelle quali le mie categorie di competenza si suddividono, in modo che gli utenti possano individuare in modo più preciso i tipi di eventi a cui sono interessati.
    - (_MA_) Creazione screen.
    - (_MA_) Creazione metodo del Client.
    - (_WS_) Creazione API endpoint.
    - (_WS_) Creazione metodo del DAO.
    - (_MA_) Creazione tests.
    - (_WS_) Creazione tests.

- da utente autorizzato, devo essere in grado di modificare le sottocategorie nelle quali le mie categorie di competenza si suddividono, in modo da riorganizzare le categorie.

- da utente autorizzato, devo essere in grado di eliminare le sottocategorie nelle quali le mie categorie di competenza si suddividono, in modo da evitare pollution.

- Visualizzazione log.
- Creazione log.
- Aggiunta eventi in formato custom.

- da utente autorizzato, voglio essere in grado di notificare tutti gli utenti, interessati ad una particolare categoria di mia competenza, della creazione di un nuovo evento nella suddetta categoria

