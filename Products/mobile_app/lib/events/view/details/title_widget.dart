

import 'package:flutter/material.dart';

class TitleWidget extends StatelessWidget {
  final String title;
  final String caption;

  const TitleWidget({
    super.key,
    required this.title,
    required this.caption,
  });

  @override
  Widget build(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    final TextTheme textTheme = theme.textTheme;

    return Container(
      margin: const EdgeInsets.only(top: 20, left: 20, right: 20),
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
      decoration: BoxDecoration(
        border: BorderDirectional(
          start: BorderSide(
            width: 2,
            color: Theme.of(context).primaryColor,
          ),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: textTheme.titleMedium,
          ),
          Text(
            caption,
            style: textTheme.labelSmall,
          ),
        ],
      ),
    );
  }
}