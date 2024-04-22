--- UtenteAutenticabile ---



context UtenteAutenticabile::login(username, password)

pre: self.auth_token = None

post: self.auth_token != None



context UtenteAutenticabile::logout()

pre: self.auth_token != None

post: serf.auth_token = None



--- Utente ---

context Utente::aggiungiZonaInteresse(zona: Zona)

pre: self.zone_interesse -> !includes(zona)

post: self.zone_interesse -> includes(zona)



context Utente::eliminaZonaInteresse(zona: Zona)

pre: self.zone_interesse -> includes(zona)

post: self.zone_interesse -> !includes(zona)



context Utente::aggiungiCategoriaDiInteresse(categoria: Categoria)

pre: self.categorie_interesse -> !includes(categoria)

post: self.categorie_interesse -> includes(categoria)



context Utente::eliminaCategoriaDiInteresse(categoria: Categoria)

pre: self.categorie_interesse -> includes(categoria)

post: self.categorie_interesse -> !includes(categoria)



context Utente::listaEventiDiInteresse() -> Evento[0..*]

/// se un evento ha anche solo una categoria d'interesse per l'utente allora deve essere inclusa nel risultato

post: result -> forAll( e | e.categorie -> exists( c | self.categorie_interesse -> includes(c) )

/// non deve esserci alcun evento, con una categoria d'interesse per l'utente, che non sia stata inclusa nel risultato

post: Evento.allInstances() -> forAll (e | if e.categorie -> exists(c | self.categorie_di_interesse -> includes(c) then result -> includes(e)



context Utente::register(username, password) 

/// l'utente può registrarsi solo se non è autenticato, i.e. non ha un auth token

pre: self.auth_token = None



--- Utente Autorizzato ---

context UtenteAutorizzato

def: haCompetenza(evento: Event): Bool =

     evento.categorie -> exists ( c | sef.categorie_di_competenza -> includes (c) )

inv: self.eventi_creati -> forAll (e | e.creatore = self)

inv: self.eventi_creati -> forAll( e | self.haCompetenza (e) )



context UtenteAutorizzato::aggiungiEvento(evento: Evento)

pre: self.eventi_creati -> includes (evento)

post: self.haCompetenza (evento) 

post: Evento.allInstances() -> includes(evento)



context UtenteAutorizzato::eliminaEvento(evento: Evento)

pre: self.haCompetenza(evento)

// l'utente autorizzato ha il lock dell'evento

pre: evento.locked = self

post: Evento.allInstances() -> !includes (evento)





context UtenteAutorizzato::modificaEvento(evento: Evento)

pre: self.haCompetenza(evento)

pre: evento.locked = self

post: self.haCompetenza(evento)



context UtenteAutorizzato::aggiungiCategoria(categoria: Categoria)

pre: categoria.supercategoria != None

pre: self.categorie_di_competenza -> includes(categoria.supercategoria)

post: self.categorie_di_competenza -> includes(categoria)

// solo l'admin può creare categorie immutabili

post: categoria.modificabile = False



context UtenteAutorizzato::eliminaCategoria(categoria: Categoria)

pre: categoria.immutabile = false

pre: self.categorie_di_competenza -> includes(categoria.supercategoria)

post: self.categorie_di_competenza -> includes(categoria)



context UtenteAutorizzato::modificaCategoria(categoria: Categoria)

pre: categoria.immutabile = false

pre: self.categorie_di_competenza -> includes(categoria.supercategoria)

post: self.categorie_di_competenza -> includes(categoria)

post: categoria.immutabile = false



context UtenteAutorizzato::lockEvent(evento: Evento)

pre: evento.locked = None

post: evento.locked = self



context UtenteAutorizzato::unlockEvent(evento: Evento)

pre: evento.locked = self

post: evento.locked = None



context UtenteAutorizzato::listaEventiDiCompetenza(): Evento[0..*]

post: Evento.allInstances() -> forAll (e | if self.haCompetenza(e) then result -> includes(e) )



--- Evento ---
