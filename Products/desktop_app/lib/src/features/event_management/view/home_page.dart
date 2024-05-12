

import 'dart:ffi';

import 'package:desktop_app/src/dao/dao.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:latlong2/latlong.dart';

import '../../../data_transfer_objects/event.dart';
import '../map/map.dart';
import '../map/tiles.dart';
import 'category_picker.dart';
import 'datetime_range_picker.dart';

class EventManagerScreen extends StatelessWidget {
  const EventManagerScreen({
    super.key,
    required this.dao,
  });

  final Dao dao;

  @override
  Widget build(BuildContext context) {
    print("build!!!!!");
    return Column(
      children: [
        const ActionBar(),
        Expanded(
          child: _EventWidget(event: Event.defaultNewEvent(0)),
        ),
      ],
    );
  }
}

class _EventWidget extends StatelessWidget {
  const _EventWidget({
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

class _SubEventsWidgetState extends State<_SubEventsWidget> {

  List<Tab> tabs = [];
  int currentIndex = 0;

  @override
  Widget build(BuildContext context) {

    tabs = widget.event.events.asMap().entries.map((entry) {
      late Tab tab;
      tab = Tab(
        text: Text(entry.value.title),
        icon: Text('${entry.key+1}'),
        body: _SubEventWidget(subevent: entry.value),
        onClosed: () {
          setState(() {
            widget.event.events.remove(entry.value);
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

        widget.event.events.add(subevent);
        late Tab tab;
        tab = Tab (
            text: Text(subevent.title),
            body: _SubEventWidget(subevent: subevent),
            icon: Text('${index+1}'),
            onClosed: () {
              setState(() {
                widget.event.events.remove(subevent);
                tabs.remove(tab);
              });
            }
        );
        tabs.add(tab);
      }),
    );
  }

}

class _SubEventsWidget extends StatefulWidget {
  const _SubEventsWidget({
    required this.event,
  });

  final Event event;

  @override
  State<StatefulWidget> createState() => _SubEventsWidgetState();
}

class _SubEventWidget extends StatelessWidget {

  SubEvent subevent;

  _SubEventWidget({
    required this.subevent,
  });

  TextEditingController? titleTEController;
  TextEditingController? descriptionTEController;

  @override
  Widget build(BuildContext context) {

    titleTEController ??= TextEditingController(text: subevent.title);
    titleTEController!.addListener(() { subevent.title = titleTEController!.text; });
    subevent.description ??= "";
    descriptionTEController ??= TextEditingController(text: subevent.description!);
    descriptionTEController!.addListener(() { subevent.description = descriptionTEController!.text; });

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
        onChanged: (_) {},
        begin: subevent.validity.begin,
        end: subevent.validity.end,
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
    if (subEvent.polygons != null) {
      polygons = subEvent.polygons!.features.map((p) {
        var coordinates;

        var geometry = p?.geometry;
        if (geometry is GeoJSONPolygon) {
          var pol = geometry as GeoJSONPolygon;
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
    }

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

class _EventGenericForm extends StatelessWidget {
  const _EventGenericForm({
    required this.event,
  });

  final Event event;

  @override
  Widget build(BuildContext context) {
    const separator = SizedBox(height: 20);

    final title = InfoLabel(
      label: "Titolo",
      child: TextFormBox(
        initialValue: event.title,
        textAlignVertical: TextAlignVertical.center,
      ),
    );

    final summary = InfoLabel(
      label: "Riassunto",
      child: TextBox(
        controller: TextEditingController(
          text: event.summary,
        ),
      ),
    );

    final validity = InfoLabel(
      label: "Validità",
      child: NullableDateTimeRangePicker(
        onChanged: (_) {},
        begin: event.validity.begin,
        end: event.validity.end,
      ),
    );

    final visibility = InfoLabel(
      label: "Visibilità",
      child: NullableDateTimeRangePicker(
        onChanged: (_) {},
        begin: event.visibility.begin,
        end: event.visibility.end,
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
          validity,
          separator,
          visibility,
          separator,
          CategoryPicker(event: event),
        ],
      ),
    );
  }
}

class ActionBar extends StatelessWidget {
  const ActionBar({
    super.key,
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
    final specific = _specificCommands.map(converter);

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