import 'package:beelive/desktop/widgets/home/category_picker.dart';
import 'package:collection/collection.dart';

import 'package:beelive/common/dao/dao.dart';
import 'package:beelive/common/map/map.dart';
import 'package:beelive/common/map/tiles.dart';
import 'package:beelive/common/problem.dart';
import 'package:beelive/desktop/widgets/home/datetime_range_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_map/flutter_map.dart';

class EventManagerScreen extends StatelessWidget {
  const EventManagerScreen({
    super.key,
    required this.dao,
  });

  final Dao dao;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const ActionBar(),
        Expanded(
          child: _EventWidget(
            key: ValueKey(dao.problems()[1]),
            problem: dao.problems()[1],
          ),
        ),
      ],
    );
  }
}

class _EventWidget extends StatelessWidget {
  const _EventWidget({
    required super.key,
    required this.problem,
  });

  final Problem problem;

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          flex: 1,
          child: _EventGenericForm(problem: problem),
        ),
        const Divider(direction: Axis.vertical),
        Expanded(
          flex: 1,
          child: _SubEventsWidget(problem: problem),
        ),
      ],
    );
  }
}

class _SubEventsWidget extends StatelessWidget {
  const _SubEventsWidget({
    required this.problem,
  });

  final Problem problem;

  @override
  Widget build(BuildContext context) {
    final tabs = problem.events.mapIndexed((i, e) {
      return Tab(
        text: Text(e.title),
        body: _SubEventWidget(event: e),
        icon: Text('${i + 1}'),
      );
    }).toList(growable: false);

    return TabView(
      currentIndex: 2,
      tabWidthBehavior: TabWidthBehavior.compact,
      onChanged: (i) {},
      tabs: tabs,
      onNewPressed: () {},
    );
  }
}

class _SubEventWidget extends StatelessWidget {
  const _SubEventWidget({
    required this.event,
  });

  final Event event;

  @override
  Widget build(BuildContext context) {
    const separator = SizedBox(height: 20);

    final title = InfoLabel(
      label: "Titolo",
      child: TextBox(
        controller: TextEditingController(text: event.title),
      ),
    );

    final description = InfoLabel(
      label: "Descrizione",
      child: TextBox(
        maxLines: null,
        controller: TextEditingController(
          text: event.description,
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

    final map = InfoLabel(
      label: "Mappa",
      child: _MapManager(event: event),
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
    required this.event,
  });

  final Event event;

  @override
  Widget build(BuildContext context) {
    final polygons = event.polygons.map((p) {
      return Polygon(
        points: p.points,
        isFilled: true,
        color: Colors.blue.withOpacity(0.2),
        borderColor: Colors.blue,
        borderStrokeWidth: 2,
        isDotted: true,
        rotateLabel: true,
        holePointsList: p.holes,
      );
    }).toList(growable: false);

    const mapActions = Expanded(child: SizedBox.shrink());

    final mapWidget = BeeLiveMap(
      children: [
        openStreetMapTileLayer,
        PolygonLayer(
          polygonCulling: true,
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
    required this.problem,
  });

  final Problem problem;

  @override
  Widget build(BuildContext context) {
    const separator = SizedBox(height: 20);

    final title = InfoLabel(
      label: "Titolo",
      child: TextFormBox(
        initialValue: problem.title,
        textAlignVertical: TextAlignVertical.center,
      ),
    );

    final summary = InfoLabel(
      label: "Riassunto",
      child: TextBox(
        controller: TextEditingController(
          text: problem.summary,
        ),
      ),
    );

    final validity = InfoLabel(
      label: "Validità",
      child: NullableDateTimeRangePicker(
        onChanged: (_) {},
        begin: problem.validity.begin,
        end: problem.validity.end,
      ),
    );

    final visibility = InfoLabel(
      label: "Visibilità",
      child: NullableDateTimeRangePicker(
        onChanged: (_) {},
        begin: problem.visibility.begin,
        end: problem.visibility.end,
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
          CategoryPicker(problem: problem),
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
