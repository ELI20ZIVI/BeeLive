


import 'package:desktop_app/client/client.dart';
import 'package:desktop_app/features/event_management/view/geojson_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:http/http.dart' as http;
import 'package:latlong2/latlong.dart';

import '../../../data_transfer_objects/event.dart';
import '../map/map.dart';
import '../map/tiles.dart';
import 'category_picker.dart';
import 'datetime_range_picker.dart';

/// The screen for visualizing the management of a single event.
///
/// Can be used both for creation and modification.
class EventManagerScreen extends StatelessWidget {

  // variable to control whether this screen is in "Management" mode (i.e. when you are modifying an event) or "Creation" mode (i.e. when creaitng a new event)
  // it's true when in creation mode.
  final bool isCreationScreen;
  final Event event;

  const EventManagerScreen({
    super.key, required this.isCreationScreen, required this.event,
  });

  @override
  Widget build(BuildContext context) {

    var actionBar = (isCreationScreen) ? EventCreationActionBar(event: event) : EventManagementActionBar(event: event);

    return Column(
      children: [
        actionBar,
        Expanded(
          child: _EventWidget(event: event),
        ),
      ],
    );
  }
}

/// This widget shows the modification form for the event attributes.
class _EventWidget extends StatelessWidget{

  _EventWidget({
    required this.event,
  });

  final Event event;

  @override
  Widget build(BuildContext context) {


    return Row(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          flex: 1,
          child: _EventGenericForm(event: event),
        ),
        const Divider(direction: Axis.vertical),
        Expanded(
          flex: 1,
          child: _SubEventsWidget(event: event),
        ),
      ],
    );
  }
}

/// This widget shows the modification form for the various subevents.
class _SubEventsWidgetState extends State<_SubEventsWidget> {

  List<Tab> tabs = [];
  int currentIndex = 0;

  @override
  Widget build(BuildContext context) {

    tabs = widget.event.subevents.asMap().entries.map((entry) {
      late Tab tab;
      tab = Tab(
        text: Text(entry.value.title),
        icon: Text('${entry.key+1}'),
        body: _SubEventWidget(subevent: entry.value),
        onClosed: () {
          setState(() {
            widget.event.subevents.remove(entry.value);
            tabs.remove(tab);
          });
        }
      );
      return tab;
    }).toList();

    return TabView(
      currentIndex: currentIndex,
      tabWidthBehavior: TabWidthBehavior.compact,
      onChanged: (index) => setState(() {
        currentIndex = index;
      }),
      tabs: tabs,
      onNewPressed: () => setState(() {

        var subevent = SubEvent.defaultNewSubevent();
        int index = tabs.length+1;

        widget.event.subevents.add(subevent);
        late Tab tab;
        tab = Tab (
            text: Text(subevent.title),
            body: _SubEventWidget(subevent: subevent),
            icon: Text('${index+1}'),
            onClosed: () {
              setState(() {
                widget.event.subevents.remove(subevent);
                tabs.remove(tab);
              });
            }
        );
        tabs.add(tab);
      }),
    );
  }

}

/// This widget shows the modification form for a single subevent.
class _SubEventsWidget extends StatefulWidget {
  const _SubEventsWidget({
    required this.event,
  });

  final Event event;

  @override
  State<StatefulWidget> createState() => _SubEventsWidgetState();
}

class _SubEventWidget extends StatelessWidget {

  final SubEvent subevent;

  _SubEventWidget({
    required this.subevent,
  });

  final TextEditingController titleTEController = TextEditingController();
  final TextEditingController descriptionTEController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    titleTEController.text = subevent.title;
    titleTEController.addListener(() { subevent.title = titleTEController.text; });
    subevent.description ??= "";
    descriptionTEController.text = subevent.description!;
    descriptionTEController.addListener(() { subevent.description = descriptionTEController.text; });

    const separator = SizedBox(height: 20);

    final title = InfoLabel(
      label: "Titolo",
      child: TextBox(
        controller: titleTEController,
      ),
    );

    final description = InfoLabel(
      label: "Descrizione",
      child: TextBox(
        maxLines: null,
        controller: descriptionTEController,
      ),
    );

    final validity = InfoLabel(
      label: "Validità",
      child: NullableDateTimeRangePicker(
        nullableDateTimeRange: subevent.validity,
      ),
    );

    final geojsonpicker = GeoJSONFilePicker(subevent: subevent);

    final map = InfoLabel(
      label: "Mappa",
      child: _MapManager(subEvent: subevent),
    );

    return ListView(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      children: [
        title,
        separator,
        description,
        separator,
        validity,
        separator,
        geojsonpicker,
        separator,
        map,
      ],
    );
  }
}

class _MapManager extends StatelessWidget {
  const _MapManager({
    required this.subEvent,
  });

  final SubEvent subEvent;

  @override
  Widget build(BuildContext context) {

    List<Polygon> polygons = [];
    polygons = subEvent.polygons.features.map((p) {
      List<LatLng> coordinates = [];

      var geometry = p?.geometry;
      if (geometry is GeoJSONPolygon) {
        var pol = geometry;
        coordinates = pol.coordinates[0].map((c) { return LatLng(c[1], c[0]); } ).toList(growable: false);
      }

      return Polygon(
        points: coordinates,
        isFilled: true,
        color: Colors.blue.withOpacity(0.2),
        borderColor: Colors.blue,
        borderStrokeWidth: 2,
        isDotted: true,
        rotateLabel: true,
        //holePointsList: p.holes,
      );
    }).toList(growable: false);

    final mapWidget = BeeLiveMap(
      children: [
        openStreetMapTileLayer,
        PolygonLayer(
          polygonCulling: false,
          polygons: polygons,
        ),
      ],
    );

    return AspectRatio(
      aspectRatio: 4 / 3,
      child: mapWidget,
    );
  }
}

class _EventGenericFormState extends State<_EventGenericForm> {

  final TextEditingController titleTEController = TextEditingController();
  final TextEditingController descriptionTEController = TextEditingController();
  final TextEditingController summaryTEController = TextEditingController();
  final TextEditingController remotedocumentTEController = TextEditingController();

  @override
  Widget build(BuildContext context) {

    titleTEController.text = widget.event.title;
    descriptionTEController.text = widget.event.description;
    summaryTEController.text = widget.event.summary;
    remotedocumentTEController.text = (widget.event.remoteDocument ?? ""  ).toString();

    titleTEController.addListener(() { widget.event.title = titleTEController.text; });
    descriptionTEController.addListener(() { widget.event.description = descriptionTEController.text; });
    summaryTEController.addListener(() { widget.event.summary = summaryTEController.text; });
    remotedocumentTEController.addListener(() { widget.event.remoteDocument = Uri.tryParse(remotedocumentTEController.text); });

    const separator = SizedBox(height: 20);

    final title = InfoLabel(
      label: "Titolo",
      child: TextFormBox(
        textAlignVertical: TextAlignVertical.center,
        controller: titleTEController,
      ),
    );

    final summary = InfoLabel(
      label: "Riassunto",
      child: TextBox(
        controller: summaryTEController,
      ),
    );

    final description = InfoLabel(
      label: "Descrizione",
      child: TextBox(
        maxLines: null,
        controller: descriptionTEController,
      ),
    );

    final remoteDocument = InfoLabel(
      label: "Link al documento remoto (URI)",
      child: TextBox(
        controller: remotedocumentTEController,
      ),
    );

    final validity = InfoLabel(
      label: "Validità",
      child: NullableDateTimeRangePicker(
        nullableDateTimeRange: widget.event.validity,
      ),
    );

    final visibility = InfoLabel(
      label: "Visibilità",
      child: NullableDateTimeRangePicker(
        nullableDateTimeRange: widget.event.visibility,
      ),
    );

    return Form(
      child: ListView(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        children: [
          title,
          separator,
          summary,
          separator,
          description,
          separator,
          remoteDocument,
          separator,
          validity,
          separator,
          visibility,
          separator,
          // Disabilitato le categorie in quanto non ancora supportate
          //CategoryPicker(event: widget.event),
          Align (
            alignment: Alignment.bottomRight,
            child: RiskLevelPicker(event: widget.event),
          )
        ],
      ),
    );
  }
}

class _EventGenericForm extends StatefulWidget {
  const _EventGenericForm({
    required this.event,
  });

  final Event event;

  @override
  State<StatefulWidget> createState() => _EventGenericFormState();

}

/// The action bar for event creation
class EventCreationActionBar extends StatelessWidget {

  final Event event;

  const EventCreationActionBar({
    super.key,
    required this.event,
  });

  static final List<(String, IconData, Function())> _universalCommands = [
    ("Nuovo", FluentIcons.add, () {}),
    ("Apri Bozza", FluentIcons.open_enrollment, () {}),
  ];

  static final List<(String, IconData, Function())> _specificCommands = [
    ("Salva Bozza", FluentIcons.save, () {}),
    ("Pubblica", FluentIcons.publish_content, () {}),
  ];

  @override
  Widget build(BuildContext context) {
    CommandBarButton converter(e) {
      var (String label, IconData icon, Function() action) = e;
      return CommandBarButton(
        onPressed: action,
        label: Text(label),
        icon: Icon(icon),
      );
    }

    final universal = _universalCommands.map(converter);
    //var specific = _specificCommands.map(converter);
    var specific = [CommandBarButton(
        onPressed: () async {
          late http.Response response;
          try {
            response = await Client.implementation.submitNewEvent(event);
          } catch (e) {
            debugPrint(e.toString());
            displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
            return;
          }
          displayInfoBar(context, builder: (context, close) {

            String status = "Errore imprevisto. Codice d'errore: ${response.statusCode}";
            String content = "Questo errore non era previsto. Contattare uno sviluppatore o un amministratore di sistema per comunicargli questo incidente.\n"
                "Ecco il contenuto della risposta HTTP: ${response.body}";
            InfoBarSeverity severity = InfoBarSeverity.error;

            switch (response.statusCode) {
              case 201:
                status = "Successo";
                content = "Evento correttamente inserito nel sistema e pubblicato.";
                severity = InfoBarSeverity.success;
                break;
              case 422:
                status = "Errore, impossibile elaborare l'evento. [${response.statusCode}]";
                content = "I dati inseriti sono corretti, però ci sono dei constraint che non vengono rispettati. Hint: questo errore è solitamente associato ad una data di fine"
                    "che precede una data di inizio.";
                severity = InfoBarSeverity.error;
                break;
              case 400:
                status = "Errore, richiesta malformata. [${response.statusCode}]";
                content = "Il server non ha potuto elaborare la richiesta. Questo non dovrebbe accadere: contatta un amministratore di sistema per informarlo di questo errore.\n"
                    "Ecco il contenuto della risposta HTTP: ${response.body}";
            }

            return InfoBar(
            title: Text(status),
            content: Text(content),
            action: IconButton(
              icon: const Icon(FluentIcons.clear),
              onPressed: close,
            ),
            severity: severity,
            );
          });
          },
        label: const Text("Pubblica"),
        icon: const Icon(FluentIcons.publish_content),
    )];

    return Padding(
      padding: const EdgeInsets.all(10),
      child: CommandBar(
        isCompact: false,
        primaryItems: [
          ...universal,
          const CommandBarSeparator(),
          ...specific,
        ],
      ),
    );
  }
}

/// The action bar for event editing
class EventManagementActionBar extends StatelessWidget {

  final Event event;

  const EventManagementActionBar({
    super.key,
    required this.event,
  });

  static final List<(String, IconData, Function())> _specificCommands = [
    ("Pubblica modifiche", FluentIcons.publish_content, () {}),
  ];

  @override
  Widget build(BuildContext context) {
    CommandBarButton converter(e) {
      var (String label, IconData icon, Function() action) = e;
      return CommandBarButton(
        onPressed: action,
        label: Text(label),
        icon: Icon(icon),
      );
    }

    //var specific = _specificCommands.map(converter);
    var specific = [CommandBarButton(
      onPressed: () async {

        late http.Response response;
        try {
          response = await Client.implementation
              .modifyExistingEvent(event);
        } catch (e) {
          debugPrint(e.toString());
          displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
          return;
        }
        displayInfoBar(context, builder: (context, close) {

          String status = "Errore imprevisto. Codice d'errore: ${response.statusCode}";
          String content = "Questo errore non era previsto. Contattare uno sviluppatore o un amministratore di sistema per comunicargli questo incidente.\n"
              "Ecco il contenuto della risposta HTTP: ${response.body}";
          InfoBarSeverity severity = InfoBarSeverity.error;

          switch (response.statusCode) {
            case 200:
              status = "Successo";
              content = "Evento correttamente modificato.";
              severity = InfoBarSeverity.success;
              break;
            case 401:
              status = "Errore";
              content = "Non è stato fornito alcun access token alla richiesta.";
              severity = InfoBarSeverity.error;
            case 403:
              status = "Non autorizzato";
              content = "Non sei autorizzato ad effettuare questa modifica, oppure questo evento non è di tua competenza.";
              severity = InfoBarSeverity.error;
            case 404:
              status = "Evento non trovato";
              content = "L'evento che si vuole modificare non è più presente a sistema. Forse è già stato eliminato?";
              severity = InfoBarSeverity.error;
            case 422:
              status = "Modifiche non valide. [${response.statusCode}]";
              content = "I dati inseriti sono corretti, però ci sono dei constraint che non vengono rispettati. Hint: questo errore è solitamente associato ad una data di fine"
                  "che precede una data di inizio.";
              severity = InfoBarSeverity.error;
              break;
            case 418:
              status = "Errore, evento non bloccato";
              content = "Questo evento è modificabile ma non è stato precedentemente bloccato. Contattare un amministratore per risolvere il problema.";
              severity = InfoBarSeverity.error;
            case 423:
              status = "Risorsa attualmente non disponibile";
              content = "Questo evento è attualmente bloccato da un altro utente autorizzato. Si prega di riprovare più tardi, o contattare un amministratore se il problema persiste.";
              severity = InfoBarSeverity.info;
            case 400:
              status = "Errore, richiesta malformata. [${response.statusCode}]";
              content = "Il server non ha potuto elaborare la richiesta. Questo non dovrebbe accadere: contatta un amministratore di sistema per informarlo di questo errore.\n"
                  "Ecco il contenuto della risposta HTTP: ${response.body}";
          }

          return InfoBar(
            title: Text(status),
            content: Text(content),
            action: IconButton(
              icon: const Icon(FluentIcons.clear),
              onPressed: close,
            ),
            severity: severity,
          );
        });
      },
      label: const Text("Pubblica modifiche"),
      icon: const Icon(FluentIcons.publish_content),
    )];

    return Padding(
      padding: const EdgeInsets.all(10),
      child: CommandBar(
        isCompact: false,
        primaryItems: [
          ...specific,
        ],
      ),
    );
  }
}
