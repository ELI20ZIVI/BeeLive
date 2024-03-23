import 'package:BeeLive/common/dao/static_dao.dart';
import 'package:BeeLive/mobile/screens/home.dart';
import 'package:flutter/material.dart';

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