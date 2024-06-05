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
import 'package:beelive_frontend_commons/beelive_frontend_commons.dart';
import 'package:shared_preferences/shared_preferences.dart' as sp;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // The URI of the web server.
  //final mwsUri = Uri(scheme: "http", host: "93.49.96.13", port: 14124);
  final mwsUri = Uri(scheme: "http", host: "localhost", port: 14124);
  // The casdoor instance is temporary assumed to be on the same host as the public server.
  final casdoorUri = mwsUri.replace(scheme: "http", port: 9987);

  // Overrides the client with the actual web server client.
  Client.override(ManagementWebServerClient(mwsUri.toString()));
  
  KeyValueStorage.override(
    SharedPreferences(await sp.SharedPreferences.getInstance()),
  );

  // TODO: must find a way to hide the secret when it becomes official.
  final AuthenticationProvider provider =
      CasdoorAuthenticationProvider.defaultConfig(
    clientSecret: "8cd8dde871a54de9f5846b1b061e1040c160833f",
    serverUrl: casdoorUri,
  );

  // Authenticator done via JWT.
  Authenticator.override(JwtAuthenticator(provider));

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

    final navigationView = NavigationView(appBar: appBar, pane: pane);

    return FutureBuilder(
      future: Authenticator().authorization(),
      initialData: "",
      builder: (context, snap) {
        debugPrint(snap.data);
        if (snap.hasData) {
          if (snap.data?.isEmpty ?? true) {
            return const Center(child: ProgressRing(value: null));
          } else {
            return navigationView;
          }
        } else {
          Authenticator().authenticate(context).then((_) => setState(() {}));
          return const Center(child: ProgressRing(value: null));
        }
      },
    );
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
