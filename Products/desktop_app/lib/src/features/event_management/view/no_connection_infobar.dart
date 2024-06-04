import 'package:fluent_ui/fluent_ui.dart';

class NoConnectionInfobar {
  static Widget noConnectionInfobarBuilder(BuildContext context, Function() close) {
    return InfoBar(
      title: const Text('Impossibile connettersi al server'),
      content: const Text('Non è stato possibile connetters al server, controllare la connessione e riprovare.'),
      action: IconButton(
        icon: const Icon(FluentIcons.clear),
        onPressed: close,
      ),
      severity: InfoBarSeverity.error,
    );
  }
}