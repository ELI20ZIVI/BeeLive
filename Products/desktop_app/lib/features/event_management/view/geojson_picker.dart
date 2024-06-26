
import 'dart:io';

import 'package:desktop_app/data_transfer_objects/event.dart';
import 'package:file_picker/file_picker.dart';
import 'package:fluent_ui/fluent_ui.dart';
import 'package:geojson_vi/geojson_vi.dart';
import 'package:path/path.dart';

class GeoJSONFilePicker extends StatefulWidget {

  final SubEvent subevent;

  const GeoJSONFilePicker({super.key, required this.subevent});

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

                  FilePickerResult? result = await FilePicker.platform.pickFiles(
                    type: FileType.custom,
                    allowedExtensions: ['json'],
                  );

                  if (result != null && result.paths[0] != null) {

                    String filePath = result.paths[0]!;

                    String fileContent = await File(filePath).readAsString();
                    debugPrint(fileContent);
                    GeoJSONFeatureCollection geojson = GeoJSONFeatureCollection.fromJSON(fileContent);
                    path = basename(filePath);
                    widget.subevent.polygons = geojson;

                    setState(() {});
                  }

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
