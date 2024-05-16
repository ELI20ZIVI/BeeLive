import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/authentication_provider.dart';
import 'package:mobile_app/src/authenticator/authentication_provider/casdoor_authentication_provider.dart';
import 'package:mobile_app/src/authenticator/authenticator.dart';
import 'package:mobile_app/src/authenticator/jwt_authenticator.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/client/public_webserver_client.dart';
import 'package:mobile_app/src/logger/riverpod_logger.dart';
import 'package:mobile_app/src/routes/routes.dart';
import 'package:mobile_app/src/storage/key_value_storage.dart';
import 'package:mobile_app/src/storage/key_value_storage/shared_preferences.dart';
import 'package:mobile_app/src/themes/theme.dart';
import 'package:shared_preferences/shared_preferences.dart' as sp;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // The URI of the web server.
  final pwsUri = Uri(scheme: "http", host: "93.49.96.13", port: 7839);
  // The casdoor instance is temporary assumed to be on the same host as the public server.
  final casdoorUri = pwsUri.replace(scheme: "http", port: 8000);

  // TODO: must find a way to hide the secret when it becomes official.
  Client.override(PublicWebServerClient(
    "dG9ydGluaSBhbCBjaW9jY29sYXRvCg==",
    pwsUri,
  ));
  // A persistent key-value storage for application preferences.
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

  runApp(const ProviderScope(
    observers: [RiverpodLogger()],
    child: BeeLiveMobile(),
  ));
}

class BeeLiveMobile extends StatelessWidget {
  const BeeLiveMobile({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'BeeLive',
      theme: theme,
      routerConfig: BeeLiveRouter().config(),
    );
  }
}
