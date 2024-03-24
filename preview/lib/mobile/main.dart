import 'package:beelive/common/dao/static_dao.dart';
import 'package:beelive/mobile/screens/home.dart';
import 'package:flutter/material.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  FlutterLocalNotificationsPlugin notify = FlutterLocalNotificationsPlugin();
  notify
      .resolvePlatformSpecificImplementation<
          AndroidFlutterLocalNotificationsPlugin>()
      ?.requestNotificationsPermission();

  notify.show(
    1,
    'Incontro G7',
    'Modifiche al trasporto pubblico',
    const NotificationDetails(
      android: AndroidNotificationDetails(
        "info",
        "viability",
        icon: '@mipmap/ic_launcher',
        category: AndroidNotificationCategory.reminder,
      ),
    ),
  );

  runApp(const MobileApp());
}

class MobileApp extends StatelessWidget {
  const MobileApp({super.key});

  static const String title = 'BeeLive';

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: title,
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.amber),
        useMaterial3: true,
      ),
      home: const HomePage(title: title, dao: StaticDao()),
    );
  }
}
