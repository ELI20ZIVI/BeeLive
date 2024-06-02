import 'package:auto_route/auto_route.dart';
import 'package:desktop_app/client/management_client.dart';
import 'package:desktop_app/data_transfer_objects/event.dart';
import 'package:desktop_app/features/event_management/view/event_list_page.dart';
import 'package:desktop_app/features/event_management/view/event_manager_page.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:desktop_app/client/client.dart';
import 'package:desktop_app/themes/theme.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

void main() {
  // Overrides the client with the actual web server client.
  //Client.override(ManagementWebServerClient("http://localhost:8080/api/v3/"));
  Client.override(ManagementWebServerClient("http://93.49.96.13:14124/api/v3/"));

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
@RoutePage()
class HomePage extends StatefulWidget {
  final Client client;

  HomePage({super.key}) : client = Client();

  @override
  State<StatefulWidget> createState() => HomePageState();
}

class HomePageState extends State<HomePage> {

  refresh() {
    setState(() {

    });
  }

  /// The home page is has multiple sub pages.\
  /// This is the variable that stores the state.
  var selectedPage = 0;

  /// Variable to store the selected event, to be used for the EventManagement screen when modifying an event. It's initially null due to no event being initially selected
  Event? selectedEvent;

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
          icon: const Icon(FluentIcons.add),
          title: Text(localization.eventCreation),
          body: EventManagerScreen(isCreationScreen: true, event: Event.defaultNewEvent(0)),
        ),
        PaneItem(
          enabled: selectedEvent != null,
          icon: const Icon(FluentIcons.edit),
          title: Text(localization.eventManagement),
          body: (selectedEvent != null) ? EventManagerScreen(isCreationScreen: false, event: selectedEvent!) : const Text("You shouldn't be able to read this text. Please contact a system administrator."),
        ),
        PaneItem(
          icon: const Icon(FluentIcons.eye_shadow),
          title: Text(localization.showEvents),
          body: EventListScreen(homePageState: this, refreshHomeScreen: refresh,),
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
