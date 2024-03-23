import 'dart:io';

import 'package:BeeLive/desktop/main.dart';
import 'package:BeeLive/mobile/main.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:intl/intl.dart';

void main() {
  Intl.defaultLocale = 'it_IT';
  Widget app = Platform.isAndroid ? const MobileApp() : const DesktopApp();
  runApp(app);
}



