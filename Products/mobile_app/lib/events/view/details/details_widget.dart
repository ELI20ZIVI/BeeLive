


import 'package:flutter/material.dart';
import 'package:mobile_app/dtos/event.dart';

class DetailsWidget extends StatefulWidget {
  final Event event;

  const DetailsWidget({super.key, required this.event});

  @override
  State<DetailsWidget> createState() => _DetailsWidgetState();

}

class _DetailsWidgetState extends State<DetailsWidget> {
  
  @override
  Widget build(final BuildContext context) {
    // TODO: implement
    return const SizedBox.shrink();
  }

}
