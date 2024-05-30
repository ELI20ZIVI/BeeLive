import 'package:desktop_app/src/client/management_client.dart';
import 'package:desktop_app/src/features/event_management/view/event_list_page.dart';
import 'package:desktop_app/src/features/event_management/view/event_manager_page.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:desktop_app/src/client/client.dart';
import 'package:desktop_app/src/themes/theme.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

void main() {
  // Overrides the client with the actual web server client.
  Client.override(ManagementWebServerClient(Uri.parse("http://93.49.96.13:14124/api/v3/insert_new_event")));

  runApp(const ProviderScope(child: BeeLiveDesktop()));
}

/// The BeeLive desktop GUI.
class BeeLiveDesktop extends StatelessWidget {
  const BeeLiveDesktop({super.key});

  @override
  Widget build(BuildContext context) {
    // TODO: use advanced routing.
    return FluentApp(
      onGenerateTitle: (context) => AppLocalizations.of(context)!.appName,
      theme: theme,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      // TODO: get locale from environment.
      locale: const Locale('it'),
      home: HomePage(),
    );
  }
}

/// The BeeLive home page.
class HomePage extends StatefulWidget {
  final Client client;

  HomePage({super.key}) : client = Client();

  @override
  State<StatefulWidget> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {

  /// The home page is has multiple sub pages.\
  /// This is the variable that stores the state.
  var selectedPage = 0;

  @override
  Widget build(BuildContext context) {
    final localization = AppLocalizations.of(context)!;

    // The app bar.
    final appBar = NavigationAppBar(
      title: Text(localization.appName),
      actions: const Align(
        alignment: Alignment.centerRight,
        child: WindowButtons(),
      ),
    );

    // The navigation drawer.
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
          icon: const Icon(FluentIcons.eye_shadow),
          title: Text(localization.showEvents),
          body: const EventListScreen(),
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
/// The window bar decoration buttons. Style: windows.
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
