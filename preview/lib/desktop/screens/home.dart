import 'package:beelive/common/dao/dao.dart';
import 'package:beelive/desktop/widgets/home/event_manager_screen.dart';
import 'package:fluent_ui/fluent_ui.dart';

class HomePage extends StatefulWidget {
  final Dao dao;

  const HomePage({super.key, required this.dao, required this.appName});

  final String appName;

  @override
  State<StatefulWidget> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    final appBar = NavigationAppBar(
      title: Text(widget.appName),
      actions: const Align(
        alignment: Alignment.centerRight,
        child: WindowButtons(),
      ),
    );

    return NavigationView(
      appBar: appBar,
      pane: NavigationPane(
        selected: 0,
        footerItems: [
          PaneItem(
            icon: const Icon(FluentIcons.settings),
            title: const Text("Impostazioni"),
            body: const SizedBox.shrink(),
          ),
        ],
        items: [
          PaneItem(
            icon: const Icon(FluentIcons.edit),
            title: const Text("Gestione Eventi"),
            body: EventManagerScreen(dao: widget.dao),
          ),
          PaneItem(
            icon: const Icon(FluentIcons.eye_shadow),
            title: const Text("Visualizza Eventi"),
            body: const SizedBox.shrink(),
          ),
          PaneItem(
            icon: const Icon(FluentIcons.tag),
            title: const Text("Gestione Categorie"),
            body: const SizedBox.shrink(),
          ),
          PaneItem(
            icon: const Icon(FluentIcons.map_layers),
            title: const Text("Gestione Layer"),
            body: const SizedBox.shrink(),
            enabled: false,
          ),
        ],
      ),
    );
  }
}

class WindowButtons extends StatelessWidget {
  const WindowButtons({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        IconButton(
          icon: const Padding(
            padding: EdgeInsets.all(10),
            child: Icon(FluentIcons.calculator_subtract),
          ),
          onPressed: () {},
        ),
        IconButton(
          icon: const Padding(
            padding: EdgeInsets.all(10),
            child: Icon(FluentIcons.calculator_multiply),
          ),
          onPressed: () {},
        ),
      ],
    );
  }
}
