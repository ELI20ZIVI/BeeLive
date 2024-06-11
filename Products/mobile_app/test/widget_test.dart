// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility in the flutter_test package. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'dart:convert';

import 'package:flutter/material.dart';

import 'package:mobile_app/src/dtos/event.dart';
import 'package:mobile_app/src/dtos/nullable_datetime_range.dart';
import 'package:mobile_app/src/dtos/risk_level.dart';

void main() {
  /*testWidgets('Counter increments smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const MyApp());

    // Verify that our counter starts at 0.
    expect(find.text('0'), findsOneWidget);
    expect(find.text('1'), findsNothing);

    // Tap the '+' icon and trigger a frame.
    await tester.tap(find.byIcon(Icons.add));
    await tester.pump();

    // Verify that our counter has incremented.
    expect(find.text('0'), findsNothing);
    expect(find.text('1'), findsOneWidget);
  });*/

  final Event event = Event(
    id: EventId(1),
    title: "title",
    summary: "summary",
    validity: NullableDateTimeRange(begin: DateTime.now(), end: DateTime.now()),
    visibility:
        NullableDateTimeRange(begin: DateTime.now(), end: DateTime.now()),
    riskLevel: RiskLevel.info,
    document: Uri.parse(
        'https://www.ufficiostampa.provincia.tn.it/Comunicati/G7-tutte-le-limitazioni-al-traffico-e-alla-sosta'),
    events: [
      SubEvent(
        title: "title",
        description: "description",
        validity: NullableDateTimeRange(begin: DateTime.now(), end: DateTime.now()),
      ),
    ],
  );

  debugPrint(jsonEncode(event));
}
