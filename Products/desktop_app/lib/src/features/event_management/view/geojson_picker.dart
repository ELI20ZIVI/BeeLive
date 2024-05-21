
import 'package:file_picker/file_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:path/path.dart';

class GeoJSONFilePicker extends StatefulWidget {

  const GeoJSONFilePicker({super.key});

  @override
  State<StatefulWidget> createState() => _GeoJSONFilePickerState();

}

class _GeoJSONFilePickerState extends State<GeoJSONFilePicker> {

  String path = "";

  @override
  Widget build(BuildContext context) {

    //uso un textbox così che esce più carino
    TextBox currentPath = TextBox(placeholder: path, readOnly: true,);

    return InfoLabel(
        label: "GeoJSON File:",
        child: Row(
          children: [
            Button(
                child: const Text("Sfoglia"),
                onPressed: () async {
                  FilePickerResult? result = await FilePicker.platform.pickFiles(type: FileType.custom, allowedExtensions: ['json']);
                  setState(() {
                    path = basename((result?.paths[0]) ?? "");
                  });
                  debugPrint(result.toString());
                }
            ),
            const SizedBox(width: 3,),
            Expanded(child: currentPath,),
          ],
        )
    );
  }

}