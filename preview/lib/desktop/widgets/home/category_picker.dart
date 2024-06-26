import 'package:beelive/common/problem.dart';
import 'package:fluent_ui/fluent_ui.dart';

class CategoryPicker extends StatelessWidget {
  const CategoryPicker({
    super.key,
    required this.problem,
  });

  final Problem problem;

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
            RiskLevelPicker(onChanged: (_) {}),
          ],
        ),
      ),
    );

    final viewer = Wrap(
      children: problem.categories.map((cat) {
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

class RiskLevelPicker extends StatelessWidget {
  const RiskLevelPicker({
    super.key,
    required this.onChanged,
  });

  final Function(RiskLevel?)? onChanged;

  @override
  Widget build(BuildContext context) {
    return ComboBox<RiskLevel>(
      items: RiskLevel.values.map((rl) {
        return ComboBoxItem<RiskLevel>(
          value: rl,
          child: Text(_toString(rl)),
        );
      }).toList(growable: false),
      value: RiskLevel.values.first,
      placeholder: const Text('Livello di rischio'),
      onChanged: onChanged,
    );
  }

  static String _toString(final RiskLevel level) {
    return switch (level) {
      RiskLevel.info => "Info",
      RiskLevel.warning => "Warning",
      RiskLevel.alert => "Alert",
    };
  }
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
