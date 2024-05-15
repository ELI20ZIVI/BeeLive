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

  final pwsUri = Uri(scheme: "http", host: "192.168.144.43", port: 8080);
  final casdoorUri = pwsUri.replace(scheme: "http", port: 8000);

  Client.override(PublicWebServerClient(
    "dG9ydGluaSBhbCBjaW9jY29sYXRvCg==",
    pwsUri,
  ));
  KeyValueStorage.override(
    SharedPreferences(await sp.SharedPreferences.getInstance()),
  );

  final AuthenticationProvider provider =
      CasdoorAuthenticationProvider.defaultConfig(
    clientSecret: "8cd8dde871a54de9f5846b1b061e1040c160833f",
    serverUrl: casdoorUri,
  );

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