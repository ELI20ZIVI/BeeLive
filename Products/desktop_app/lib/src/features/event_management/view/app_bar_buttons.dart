import 'package:fluent_ui/fluent_ui.dart';

class WindowButtons extends StatelessWidget {
  const WindowButtons({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        IconButton(
          icon: const Padding(
            padding: EdgeInsets.all(10),
            child: Icon(FluentIcons.calculator_subtract),
          ),
          onPressed: () {},
        ),
        IconButton(
          icon: const Padding(
            padding: EdgeInsets.all(10),
            child: Icon(FluentIcons.calculator_multiply),
          ),
          onPressed: () {},
        ),
      ],
    );
  }
}