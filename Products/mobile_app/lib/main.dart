import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/src/client/client.dart';
import 'package:mobile_app/src/client/dummy_client.dart';
import 'package:mobile_app/src/client/public_webserver_client.dart';
import 'package:mobile_app/src/features/events/view/home_page.dart';
import 'package:mobile_app/src/themes/theme.dart';

void main() {
  Client.override(PublicWebServerClient());

  runApp(const ProviderScope(child: BeeLiveMobile()));
}

class BeeLiveMobile extends StatelessWidget {
  const BeeLiveMobile({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'BeeLive',
      theme: theme,
      routes: {
        Navigator.defaultRouteName: (_) => HomePage(),
        /*"/login": (_) => LoginPage(),
        "/details": (_) => EventDetailsPage(),
        "/settings": (_) => SettingsPage(),*/
      },
    );
  }
}