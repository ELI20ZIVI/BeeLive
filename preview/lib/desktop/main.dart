import 'package:beelive/common/dao/static_dao.dart';
import 'package:beelive/desktop/screens/home.dart';
import 'package:fluent_ui/fluent_ui.dart';

void main() {

  runApp(const DesktopApp());
}

class DesktopApp extends StatelessWidget {
  const DesktopApp({super.key});

  @override
  Widget build(BuildContext context) {
    return FluentApp(
      title: 'BeeLive Manager',
      debugShowCheckedModeBanner: false,
      theme: FluentThemeData(
        accentColor: Colors.orange,
      ),
      home: const HomePage(dao: StaticDao(), appName: 'BeeLive Manager'),
    );
  }
}