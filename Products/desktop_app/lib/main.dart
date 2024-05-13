import 'package:desktop_app/src/client/management_client.dart';
import 'package:desktop_app/src/features/event_management/view/home_page.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/themes/theme.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

void main() {

  Client.override(ManagementWebServerClient(Uri.parse("http://localhost:8080/api/v3/insert_event")));

  runApp(const ProviderScope(child: BeeLiveDesktop()));
}

class BeeLiveDesktop extends StatelessWidget {
  const BeeLiveDesktop({super.key});

  @override
  Widget build(BuildContext context) {
    return FluentApp(
      onGenerateTitle: (context) => AppLocalizations.of(context)!.appName,
      theme: theme,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      locale: const Locale('it'),
      home: HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  final Client client;

  HomePage({super.key}) : client = Client();

  @override
  State<StatefulWidget> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {

  var selectedPage = 0;

  @override
  Widget build(BuildContext context) {
    final localization = AppLocalizations.of(context)!;

    final appBar = NavigationAppBar(
      title: Text(localization.appName),
      actions: const Align(
        alignment: Alignment.centerRight,
        child: WindowButtons(),
      ),
    );

    final pane = NavigationPane(
      selected: selectedPage,
      onChanged: (selected) {
        setState(() => selectedPage = selected);
      },
      footerItems: [
        PaneItem(
          enabled: false,
          icon: const Icon(FluentIcons.settings),
          title: Text(localization.settings),
          body: const SizedBox.shrink(),
        ),
      ],
      items: [
        PaneItem(
          icon: const Icon(FluentIcons.edit),
          title: Text(localization.eventManagement),
          body: const EventManagerScreen(),
        ),
        PaneItem(
          enabled: false,
          icon: const Icon(FluentIcons.eye_shadow),
          title: Text(localization.showEvents),
          body: const SizedBox.shrink(),
        ),
        PaneItem(
          enabled: false,
          icon: const Icon(FluentIcons.tag),
          title: Text(localization.categoriesManagement),
          body: const SizedBox.shrink(),
        ),
        PaneItem(
          enabled: false,
          icon: const Icon(FluentIcons.map_layers),
          title: Text(localization.layersManagement),
          body: const SizedBox.shrink(),
        ),
      ],
    );

    return NavigationView(appBar: appBar, pane: pane);
  }
}

// TODO: mi sa che c'è già qualche package che li supporta adattivi.
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