import 'package:flutter/material.dart';

class LoadingWidget extends StatelessWidget {
  const LoadingWidget({super.key});

  @override
  Widget build(final BuildContext context) {
    // Circular progress bar a null
    return const CircularProgressIndicator(value: null);
  }

}
