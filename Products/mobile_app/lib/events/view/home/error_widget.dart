

import 'package:flutter/material.dart';

/// Widget class for visualizing a loading error.
class ErrorWidget extends StatelessWidget {
  final Object error;
  final StackTrace stacktrace;

  const ErrorWidget(this.error, this.stacktrace, {super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Text('${error.toString()}\n$stacktrace'),
    );
  }
}
