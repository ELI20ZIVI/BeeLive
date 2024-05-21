


import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/data_transfer_objects/risk_level.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:geojson_vi/geojson_vi.dart';
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

  const EventManagerScreen({
    super.key,
  });

  @override
  Widget build(BuildContext context) {

    var event = Event.defaultNewEvent(0);

    return Column(
      children: [
        ActionBar(event: event),
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
  
    const mapActions = Expanded(child: SizedBox.shrink());

    final mapWidget = BeeLiveMap(
      children: [
        openStreetMapTileLayer,
        PolygonLayer(
          polygonCulling: false,
          polygons: polygons,
        ),
      ],
    );

    return ConstrainedBox(
      constraints: const BoxConstraints(maxHeight: 200),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          mapActions,
          AspectRatio(
            aspectRatio: 4 / 3,
            child: mapWidget,
          ),
        ],
      ),
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
          description,
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

/// The action bar for event management
class ActionBar extends StatelessWidget {

  final Event event;

  const ActionBar({
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
        onPressed: () { Client.implementation.submitNewEvent(event); },
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
