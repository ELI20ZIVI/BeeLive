import 'package:beelive/common/model/nullable_datetime_range.dart';
import 'package:fluent_ui/fluent_ui.dart' hide DateTimeRange;
import 'package:intl/intl.dart';

class NullableDateTimeRangePicker extends StatefulWidget {
  const NullableDateTimeRangePicker({
    super.key,
    this.begin,
    this.end,
    required this.onChanged,
  });

  final DateTime? begin, end;

  final Function(NullableDateTimeRange)? onChanged;

  @override
  State<StatefulWidget> createState() => NullableDateTimeRangePickerState();
}

class NullableDateTimeRangePickerState
    extends State<NullableDateTimeRangePicker> {
  late final NullableDateTimeRange _range;

  @override
  void initState() {
    _range = NullableDateTimeRange(begin: widget.begin, end: widget.end);
    super.initState();
  }

  void _notifyChange() {
    widget.onChanged?.call(_range);
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final actualPicker = Column(
      children: [
        DateTimeRangeForm(
          header: "Inizio",
          dt: _range.begin,
          onChanged: (dt) {
            _range.begin = dt;
            _notifyChange();
          },
        ),
        const SizedBox(height: 8),
        DateTimeRangeForm(
          header: "Fine",
          dt: _range.end,
          onChanged: (dt) {
            _range.end = dt;
            _notifyChange();
          },
        ),
      ],
    );

    return Expander(
      header: _DateTimeRangeHeader(range: _range),
      content: actualPicker,
    );
  }
}

class _DateTimeRangeHeader extends StatelessWidget {
  const _DateTimeRangeHeader({
    required this.range,
  });

  final NullableDateTimeRange range;

  @override
  Widget build(BuildContext context) {
    var (String begin, String end) = format(range);

    return Row(
      children: [
        Expanded(child: Text(begin)),
        const Icon(FluentIcons.forward),
        Expanded(
          child: Text(
            end,
            textAlign: TextAlign.right,
          ),
        ),
      ],
    );
  }

  (String, String) format(final NullableDateTimeRange range) {
    final NullableDateTimeRange(:DateTime? begin, :DateTime? end) = range;

    DateFormat formatter = DateFormat('dd MMMM y HH:mm');

    final String beginStr = begin == null ? "..." : formatter.format(begin);
    final String endStr = end == null ? "..." : formatter.format(end);

    return (beginStr, endStr);
  }
}

class DateTimeRangeForm extends StatelessWidget {
  const DateTimeRangeForm({
    super.key,
    required this.dt,
    required this.onChanged,
    required this.header,
  });

  final DateTime? dt;

  final Function(DateTime?) onChanged;

  final String header;

  @override
  Widget build(BuildContext context) {
    return InfoLabel(
      label: header,
      child: DateTimePicker(
        dt: dt,
        onChanged: onChanged,
      ),
    );
  }
}

class DateTimePicker extends StatelessWidget {
  const DateTimePicker({
    super.key,
    required this.dt,
    required this.onChanged,
  });

  final DateTime? dt;

  final Function(DateTime?) onChanged;

  @override
  Widget build(BuildContext context) {
    final datePicker = Expanded(
      flex: 3,
      child: DatePicker(
        selected: dt,
        onChanged: onChanged,
      ),
    );

    final timePicker = Expanded(
      flex: 2,
      child: TimePicker(
        hourFormat: HourFormat.HH,
        selected: dt,
        onChanged: onChanged,
      ),
    );

    return Row(
      children: [
        datePicker,
        const SizedBox(width: 8),
        timePicker,
      ],
    );
  }
}
