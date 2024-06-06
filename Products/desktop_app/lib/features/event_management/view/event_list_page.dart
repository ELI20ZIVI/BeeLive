
import 'dart:async';
import 'dart:io';

import 'package:desktop_app/client/client.dart';
import 'package:fluent_ui/fluent_ui.dart';

import '../../../../main.dart';
import '../../../data_transfer_objects/event.dart';
import '../map/map.dart';
import '../map/tiles.dart';
import 'category_picker.dart';
import 'datetime_range_picker.dart';
import 'event_manager_page.dart';
import 'no_connection_infobar.dart';

/// The screen for visualizing the management of a single event.
///
/// Can be used both for creation and modification.
class _EventListScreenState extends State<EventListScreen>{


  @override
  void initState() {

    try {
      Client().getEventList().then((v) {
        var (response, list) = v;

        switch (response.statusCode){
          case 200:
            break;
          default:
            displayInfoBar(context, builder: (context, close) {
            return InfoBar(
              title: Text("Error [${response.statusCode}: ${response.reasonPhrase}]"),
              content: Text("${response.body}"),
              severity: InfoBarSeverity.error,
              action: IconButton(
                icon: const Icon(FluentIcons.clear),
                onPressed: close,
              ),
            );
          });
        }

        if (list != null) {
          setState(() {
            widget.eventList.clear();
            widget.eventList.addAll(list);
          });
        }
        else {
          debugPrint("[${response.statusCode}]\n${response.reasonPhrase}\n${response.body}");
        }
      }).catchError((err) {
        debugPrint(err.toString());
        displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
      });

      super.initState();
    } catch (e) {
      displayInfoBar(context, builder: NoConnectionInfobar.noConnectionInfobarBuilder);
    }
  }

  refreshEventList() {
    setState(() {
    });
  }

  @override
  Widget build(BuildContext context) {

    return ListView(children: widget.eventList.map((e) {
      return EventListElementWidget(event: e, homePageState: widget.homePageState, refreshHomeScreen: widget.refreshHomeScreen, eventList: widget.eventList, refreshEventList: refreshEventList);
    }).toList());
  }
}

class EventListScreen extends StatefulWidget {

  final HomePageState homePageState;
  final List<Event> eventList = [];

  final Function() refreshHomeScreen;

  EventListScreen({
    super.key, required this.homePageState, required this.refreshHomeScreen,
  });

  @override
  State<StatefulWidget> createState() => _EventListScreenState();

}

class EventListElementWidget extends StatelessWidget {

  final Function() refreshEventList;
  final List<Event> eventList;
  final Event event;
  final HomePageState homePageState;
  final Function() refreshHomeScreen;

  static const SizedBox separator = SizedBox(width: 20);

  const EventListElementWidget({super.key, required this.event, required this.homePageState, required this.refreshHomeScreen, required this.eventList, required this.refreshEventList});

  @override
  Widget build(BuildContext context) {

    return ListTile(
        leading: Text("[ID: ${event.id}]"),
        title: Row(children: [
          Text(event.title),
          separator,
          FilledButton(child: const Icon(FluentIcons.edit), onPressed: () {
            homePageState.selectedEvent = event;
            homePageState.selectedPage = 1; // redirect to event management screen
            refreshHomeScreen();
          }),
          Button(child: const Icon(FluentIcons.delete), onPressed: () async {

            var response = await Client().deleteExistingEvent(event.id as int);

            late InfoBarSeverity severity;
            late String title;
            late String content;

            switch (response.statusCode) {
              case 200:
                debugPrint("Deleting event ${event.id} [${event.title}]");
                debugPrint("${response.statusCode}");
                debugPrint("${response.body}");
                eventList.remove(event);
                refreshEventList();
                return;
              case 401:
                title = "Token invalido";
                content = "Non è stato fornito un access token valido a questa richiesta. Contattare un amministratore per risolvere il problema.";
                severity = InfoBarSeverity.error;
                break;
              case 403:
                title = "Non autorizzato";
                content = "Non sei autorizzato ad effettuare questa modifica, oppure questo evento non è di tua competenza.";
                severity = InfoBarSeverity.error;
                break;
              case 404:
                title = "Evento non trovato";
                content = "L'evento che si vuole modificare non è più presente a sistema. Forse è già stato eliminato?";
                severity = InfoBarSeverity.error;
                break;
              case 418:
                title = "Errore, evento non bloccato";
                content = "Questo evento è modificabile ma non è stato precedentemente bloccato. Contattare un amministratore per risolvere il problema.";
                severity = InfoBarSeverity.error;
                break;
              case 423:
                title = "Risorsa attualmente non disponibile";
                content = "Questo evento è attualmente bloccato da un altro utente autorizzato. Si prega di riprovare più tardi, o contattare un amministratore se il problema persiste.";
                severity = InfoBarSeverity.info;
                break;
              default:
                title = "Errore imprevisto [${response.statusCode}]";
                content = "Questo errore non era previsto, contattare un amministratore di sistema per comunicargli il problema. \n"
                    "Content della risposta HTTP: ${response.body}";
                severity = InfoBarSeverity.error;
            }
            displayInfoBar(context, builder: (context, close){
              return InfoBar(
              title: Text(title),
              content: Text(content),
              action: IconButton(
                icon: const Icon(FluentIcons.clear),
                onPressed: close,
              ),
              severity: severity,
              );
            });

          }),
    ],),
        onPressed: () {},
      );
  }
}
