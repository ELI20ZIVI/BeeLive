import 'package:desktop_app/data_transfer_objects/event.dart';
import 'package:fluent_ui/fluent_ui.dart';

import '../../../data_transfer_objects/risk_level.dart';

/// The view for the category picker.
class CategoryPicker extends StatelessWidget {
  const CategoryPicker({
    super.key,
    required this.event,
  });

  final Event event;

  @override
  Widget build(BuildContext context) {
    const suggestBox = Expanded(
      child: AutoSuggestBox(
        items: [],
        placeholder: "Tag",
      ),
    );

    final actualPicker = InfoLabel(
      label: "Categorie",
      child: SizedBox(
        height: 34,
        child: Row(
          children: [
            suggestBox,
            const SizedBox(width: 8),
            RiskLevelPicker(event: event,),
          ],
        ),
      ),
    );

    final viewer = Wrap(
      children: event.categories.map((cat) {
        return Chip(child: Text(cat.toString()));
      }).toList(growable: false),
    );

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [actualPicker, viewer],
    );
  }
}

class RiskLevelPickerState extends State<RiskLevelPicker> {
  @override
  Widget build(BuildContext context) {

    late ComboBox<RiskLevel> comboBox;

    comboBox = ComboBox<RiskLevel>(
      items: RiskLevel.values.map((rl) {
        return ComboBoxItem<RiskLevel>(
          value: rl,
          child: Text(_toString(rl)),
        );
      }).toList(growable: false),
      value: widget.event.riskLevel,
      placeholder: const Text('Livello di rischio'),
      onChanged: (RiskLevel? rl) {
        rl ??= RiskLevel.info;
        setState(() {
          widget.event.riskLevel = rl!;
        });
      },
    );
    return comboBox;
  }

  static String _toString(final RiskLevel level) {
    return switch (level) {
      RiskLevel.info => "Info",
      RiskLevel.warning => "Warning",
      RiskLevel.alert => "Alert",
    };
  }

}

class RiskLevelPicker extends StatefulWidget {

  final Event event;

  const RiskLevelPicker({
    super.key,
    required this.event,
  });
  @override
  State<StatefulWidget> createState() => RiskLevelPickerState();

}

class Chip extends StatelessWidget {
  const Chip({
    super.key,
    required this.child,
  });

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(
        horizontal: 4,
        vertical: 2,
      ),
      margin: const EdgeInsets.all(2),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(4),
        color: Colors.black,
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          DefaultTextStyle(
            style: const TextStyle(color: Colors.white),
            child: child,
          ),
          const SizedBox(width: 4),
          const Icon(
            FluentIcons.status_error_full,
            color: Colors.white,
            size: 16,
          ),
        ],
      ),
    );
  }
}
