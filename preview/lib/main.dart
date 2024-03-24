import 'dart:io';

import 'package:beelive/desktop/main.dart' as desktop;
import 'package:beelive/mobile/main.dart' as mobile;
import 'package:flutter/foundation.dart';
import 'package:intl/intl.dart';

void main() {
  Intl.defaultLocale = 'it_IT';

  if (!kIsWeb && Platform.isAndroid) {
    mobile.main();
  } else {
    desktop.main();
  }
}
